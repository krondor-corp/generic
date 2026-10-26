---
title: Deploying
order: 7
---

Services are deployed with [Kamal](https://kamal-deploy.org/), which builds Docker containers and rolls them out behind a Traefik reverse proxy with automatic SSL.

## Deploy a service

```bash
bin/kamal py deploy
# or
make kamal ARGS="py deploy"
```

First-time setup for a new service:

```bash
bin/kamal py setup
bin/kamal py deploy
```

## How it works

1. `bin/kamal` wraps itself in `confit ssh` to load the Terraform-generated deploy key
2. Kamal reads the service config from `config/deploy/<service>.yml`
3. Secrets are injected from `.kamal/secrets`, which calls `confit resolve --reveal`
4. Kamal builds the Docker image (context: `web/`, dockerfile: `web/<service>/Dockerfile`; `ios` uses the repo root as context because it depends on `web/rust/crates/core` by path)
5. The image is pushed to Docker Hub and deployed with zero-downtime rolling updates

## Service configs

Each service has a Kamal config at `config/deploy/<service>.yml`. These are ERB templates that call confit for dynamic values:

```yaml
service: <%%= `confit resolve project.name`.strip %>-py
image: <%%= `confit resolve services.py.repo`.strip %>-py

servers:
  web:
    - <%%= `confit --set stage=production resolve credentials.server.ip`.strip %>

proxy:
  ssl: true
  host: <%%= `confit resolve services.py.domain`.strip %>
  app_port: <%%= `confit resolve services.py.port`.strip %>
```

Environment variables are defined directly in the Kamal config — not in `confit.toml`.

## Secrets

`.kamal/secrets` resolves secrets via confit and exports them for Kamal:

```bash
DOCKER_HUB_TOKEN=$(confit resolve --reveal credentials.cloud.docker_hub_token)
```

Reference these in the Kamal config under `env.secret`.

## Adding a new service

1. Add the service source under `web/<name>/` with a `Dockerfile`
2. Add a `[services.<name>]` entry in `confit.toml` with `repo`, `domain`, and `port`
3. Create `config/deploy/<name>.yml` using an existing config as a template
4. Run `make infra ARGS="apply"` to create the DNS record (the subdomain is picked up automatically from `services.<name>.domain`)
5. Deploy: `bin/kamal <name> setup && bin/kamal <name> deploy`

## Available services

| Service | Domain | Port | Stack |
|---------|--------|------|-------|
| py | `py.<zone>` | 8000 | Python (FastAPI) |
| rust | `rust.<zone>` | 3000 | Rust (Axum) |
| ts-web | `ts.<zone>` | 3000 | TypeScript (Next.js) |
| ios | `ios.<zone>` | 3000 | Rust (Axum) — the mobile example's page + API |

## Publishing the iOS app

The app is not a Kamal service; it goes to Apple. `bin/testflight` (or `make testflight`) builds the app signed for App Store Connect and uploads it, and the `TestFlight` GitHub workflow runs the same script on a macOS runner whenever `mobile/tauri` changes on `main`, or on demand from the Actions tab. No infrastructure beyond an Apple Developer account.

### One-time setup

In order:

1. **Apple Developer Program** membership for the team (paid, individual or organization). Note the Team ID under *Membership details*.
2. **App ID.** In *Certificates, Identifiers & Profiles → Identifiers*, register an App ID with the explicit bundle id `org.krondor.generic.ios` (must equal `identifier` in `mobile/tauri/example/app/src-tauri/tauri.conf.json`). No capabilities needed.
3. **App record.** In App Store Connect, *Apps → +*, platform iOS, name "Generic", that bundle id, any SKU. This is the record builds attach to.
4. **API key.** *Users and Access → Integrations → Team Keys → +*, role **Admin** (Xcode uses it to create the distribution certificate and provisioning profile on the fly; App Manager cannot). Download the `.p8` once; Apple never shows it again.
5. **1Password item** `APP_STORE_CONNECT` in the app vault: username = Key ID, credential = the `.p8` contents (paste the whole file as the password field), plus text fields `issuer` (Issuer ID, shown above the keys table) and `team` (Team ID). `confit.toml` reads these as `credentials.apple.*`; `make validate` confirms they resolve.
6. **First upload.** `make testflight` from a Mac with Xcode. It generates the Xcode project if needed, builds with the team from confit, exports an `.ipa` for App Store Connect, and uploads it with `xcrun altool`. Apple processes the build in a few minutes; for internal testers that is all. For the public link, answer the export-compliance question once on the build (the app uses only standard HTTPS: "No" to proprietary encryption), create an external group, and submit the build for the brief beta review.
7. **Link it.** Put the public TestFlight URL in `services.ios.testflight_url` and `make kamal ARGS="ios deploy"` so the page at `ios.<zone>` links to it.

### How the script works

```bash
bin/testflight                      # build + upload
bin/testflight --skip-upload        # just the .ipa
bin/testflight --build-number 42    # override the build number
```

- Resolves `credentials.apple.*` from confit, writes the `.p8` to a temp dir (deleted on exit), and exports `APPLE_DEVELOPMENT_TEAM` plus the three `APPLE_API_*` variables Tauri reads.
- Runs `make init-ios` if `src-tauri/gen/apple` is missing, then `tauri ios build --export-method app-store-connect`. Signing is automatic (`-allowProvisioningUpdates` with the API key), so no certificate or profile is installed in a keychain, and a laptop and a CI runner behave the same.
- Uploads the `.ipa` with `xcrun altool --upload-app`.

**Build numbers.** `CFBundleVersion` is set to the UTC minute (`YYYYMMDDHHMM`) through `bundle.iOS.bundleVersion`, so every upload is unique and increasing wherever it ran. `CFBundleShortVersionString` stays `version` from `tauri.conf.json`; bump that when testers should see a new version. Tauri's own `--build-number` appends to the version and produces a four-part string App Store Connect rejects, so the script does not use it.

### CI

`.github/workflows/testflight.yml` needs nothing beyond the existing `OP_SERVICE_ACCOUNT_TOKEN` secret: the Apple credentials come from 1Password through confit, exactly like the server deploys. The job installs `xcodegen` (which `tauri ios init` shells out to), the Rust `aarch64-apple-ios` target, and pnpm, then runs `bin/testflight` and keeps the `.ipa` as a workflow artifact for 14 days. macOS runners bill at 10x Linux minutes; a cold build is around 15 minutes and the Rust cache brings it down to about 5.

**Android** is not wired up: `make build-android` produces the bundle, and the same script shape works with `bundletool` and a Play Console service-account JSON if you need it.
