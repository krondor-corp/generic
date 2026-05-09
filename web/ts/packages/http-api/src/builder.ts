import { Request, Response } from "express";
import { z, ZodSchema } from "zod";
import { v4 as uuid } from "uuid";
import { createErrorFactory, TypedError } from "@quotientjs/error";

import { HttpStatusCode } from "./types";
import { Handler } from "./types";

/**
 * Error factory for API route errors
 */
export const ApiRouteError = createErrorFactory({
  InvalidRequestBody: ["Failed to parse request body"],
  InvalidResponse: "Invalid response",
  InternalServerError: ["An internal server error occurred", { error: String }],
});

/**
 * Create a standard error response with appropriate status code based on error type
 * Uses the ApiRouteError type system to determine the appropriate status code
 */
export function errorResponse({
  error,
  res,
  statusCode,
}: {
  error: TypedError;
  res: Response;
  statusCode?: HttpStatusCode;
}) {
  // Determine status code based on error type if not explicitly provided
  let code: HttpStatusCode;

  if (statusCode) {
    code = statusCode;
  } else if (ApiRouteError.is(error, ApiRouteError.Type.InvalidRequestBody)) {
    code = 422; // Unprocessable Entity
  } else {
    code = 500; // Internal Server Error (default)
  }

  // Format the error response
  const errorBody: Record<string, any> = {
    error: code === 500 ? "Internal server error" : error.message,
  };

  res.status(code).json(errorBody);
}

/**
 * Creates a new API route handler with authentication, validation, and error handling
 * Now works with the unified Handler type that supports optional schemas
 */
export function createHandler<
  TRequestSchema extends ZodSchema | undefined = undefined,
  TQuerySchema extends ZodSchema | undefined = undefined,
  TError extends TypedError = TypedError,
  TResponseSchema extends ZodSchema | undefined = undefined,
>(
  config: Handler<TRequestSchema, TQuerySchema, TError, TResponseSchema>,
): (req: Request, res: Response) => Promise<void> {
  // Default configuration
  const {
    requestSchema,
    querySchema,
    responseSchema = z.any(),
    errorToResponse = (_error: TError) => ({
      statusCode: 500,
      error: "Internal server error",
    }),
    handler,
  } = config;

  // Set up default schemas based on metho
  const finalRequestSchema = requestSchema || z.any();
  const finalQuerySchema = querySchema || z.any();

  const requestId = uuid();

  return async (req: Request, res: Response): Promise<void> => {
    /* Reject requests with the wrong method (this is a sanity check) */

    try {
      /* Parse and validate request body according to the schema */
      let parsedBody: any;
      try {
        const rawBody =
          req.method === "GET"
            ? {} // For GET requests, the body is empty
            : await req.body;
        // Validate against schema
        parsedBody = finalRequestSchema.parse(rawBody);
      } catch (error) {
        throw ApiRouteError.InvalidRequestBody({});
      }

      /* Parse and validate query parameters */
      let parsedQuery: any;
      try {
        // Extract query parameters from URL
        const params = req.query;
        // Validate against schema
        parsedQuery = finalQuerySchema.parse(params);
      } catch (error) {
        throw ApiRouteError.InvalidRequestBody({});
      }

      /* Call the handler */
      let result: any;
      try {
        // Call handler with validated data
        result = await handler({
          requestId,
          body: parsedBody,
          query: parsedQuery,
        });
      } catch (error) {
        // Catch any errors raised from the handler and use its
        //  errorToResponse function to format the response
        const { statusCode, error: errorMessage } = errorToResponse(
          error as TError,
        );
        errorResponse({
          error: error as TError,
          res,
          statusCode,
        });
        return;
      }

      // Validate the response against the schema
      const validatedResult = responseSchema.parse(result);
      // Create the response
      res.json(validatedResult);
    } catch (error: unknown) {
      // Check if this is a known API route error (including validation errors)
      if (ApiRouteError.isKnown(error)) {
        // Handle known API route errors
        errorResponse({
          error: error as TError,
          res,
        });
      } else {
        // Handle unknown errors
        const serverError = ApiRouteError.InternalServerError({
          error: error instanceof Error ? error.message : "Unknown error",
        });
        errorResponse({
          error: serverError,
          res,
        });
      }
    }
  };
}
