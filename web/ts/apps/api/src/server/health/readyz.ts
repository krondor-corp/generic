import { z } from "zod";

import { createHandler } from "@repo/http-api";

export const handler = createHandler({
  responseSchema: z.object({
    status: z.literal("ok"),
    timestamp: z.string(),
    uptime: z.number(),
    memory: z.object({
      rss: z.number(),
      heapTotal: z.number(),
      heapUsed: z.number(),
    }),
  }),
  handler: async () => {
    return {
      status: "ok",
      timestamp: new Date().toISOString(),
      uptime: process.uptime(),
      memory: process.memoryUsage(),
    };
  },
});
