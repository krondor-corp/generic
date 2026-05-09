import { z } from "zod";

// Raw environment schema
const envSchema = z.object({
  PORT: z.coerce.number().default(3001),

  ENV: z
    .enum(["development", "staging", "production", "test"])
    .default("development"),
});

// Parse env vars with better error handling
const parseEnv = () => {
  const result = envSchema.safeParse(process.env);

  if (!result.success) {
    const missing = result.error.issues
      .filter((i) => i.code === "invalid_type" && i.received === "undefined")
      .map((i) => i.path.join("."));

    if (missing.length) {
      const MAX_TO_SHOW = 5;
      const truncated =
        missing.length > MAX_TO_SHOW ? missing.slice(0, MAX_TO_SHOW) : missing;

      const message = `Missing environment variables: ${truncated.join(", ")}${
        missing.length > MAX_TO_SHOW
          ? `, and ${missing.length - MAX_TO_SHOW} more`
          : ""
      }.\nMake sure these are set in your .env file or environment.`;

      throw new Error(message);
    }
    throw result.error;
  }

  return result.data;
};

const env = parseEnv();

export const config = {
  server: {
    port: env.PORT,
    env: env.ENV,
  },
} as const;

export type Config = typeof config;
