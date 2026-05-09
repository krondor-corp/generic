import { z } from "zod";

import { createHandler } from "@repo/http-api";

import pkg from "../../../package.json";

interface PackageJson {
  version: string;
  [key: string]: unknown;
}

export const handler = createHandler({
  responseSchema: z.object({
    status: z.literal("ok"),
    timestamp: z.string(),
    node_version: z.string(),
    platform: z.string(),
    arch: z.string(),
    version: z.string(),
  }),
  handler: async () => {
    return {
      status: "ok",
      timestamp: new Date().toISOString(),
      node_version: process.version,
      platform: process.platform,
      arch: process.arch,
      version: (pkg as PackageJson).version,
    };
  },
});
