import { z } from "zod";

import { createHandler } from "@repo/http-api";

export const EchoRequestSchema = z.any();

export type EchoRequest = z.infer<typeof EchoRequestSchema>;

export const EchoResponseSchema = z.any();

export type EchoResponse = z.infer<typeof EchoResponseSchema>;

export const handler = createHandler({
  requestSchema: EchoRequestSchema,
  responseSchema: EchoResponseSchema,
  handler: async ({ body }: { body: EchoRequest }) => {
    return {
      ...body,
    };
  },
});
