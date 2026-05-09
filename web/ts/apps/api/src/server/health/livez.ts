import { z } from "zod";

import { createHandler } from "@repo/http-api";

export const handler = createHandler({
  responseSchema: z.object({
    status: z.literal("ok"),
  }),
  handler: async () => {
    return {
      status: "ok",
    };
  },
});
