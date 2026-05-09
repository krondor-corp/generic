locals {
  name_prefix = "${var.project_name}-production"
}

resource "digitalocean_ssh_key" "root" {
  name       = "${local.name_prefix}-root"
  public_key = var.root_public_key
}

module "droplet" {
  source = "../../modules/digitalocean/droplet"

  name     = "${local.name_prefix}-droplet"
  region   = var.region
  size     = var.size
  ssh_keys = [digitalocean_ssh_key.root.id]
  tags     = [var.project_name, "production"]
}

module "project" {
  source = "../../modules/digitalocean/project"

  name        = local.name_prefix
  description = "${var.project_name} production"
  resources   = [module.droplet.urn]
}
