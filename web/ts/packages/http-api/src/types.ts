import { z, ZodSchema } from "zod";
import { TypedError } from "@quotientjs/error";

export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
export type HttpStatusCode =
  | 200
  | 201
  | 204
  | 400
  | 401
  | 403
  | 404
  | 405
  | 422
  | 500;

// Helper type for the handler function to enable type inference
export type ApiRouteHandler<TRequest, TQuery, TResponse = any> = ({
  requestId,
  body,
  query,
}: {
  // Request id that gets attached to the span and logger
  requestId: string;
  // Request body
  body: TRequest;
  // Query parameters
  query: TQuery;
}) => Promise<TResponse>;

export type ErrorToResponse<TError extends TypedError> = (error: TError) => {
  statusCode: HttpStatusCode;
  error: string;
};

/**
 * Helper type to determine the final request type based on the schema
 * - If no schema is provided, defaults to empty object
 * - If schema is provided, infers the type from it
 */
type InferRequestType<T> = T extends ZodSchema ? z.infer<T> : {};

/**
 * Helper type to determine the final query type based on the schema
 * - If no schema is provided, defaults to Record<string, string>
 * - If schema is provided, infers the type from it
 */
type InferQueryType<T> = T extends ZodSchema
  ? z.infer<T>
  : Record<string, string>;

type InferResponseType<T> = T extends ZodSchema ? z.infer<T> : any;

/**
 * Unified API route handler configuration that supports optional schemas
 * - requestSchema and querySchema are both optional
 * - When not provided, sensible defaults are used
 * - Full type inference from schemas when provided
 * - Method is added by the route builder when used with routeBuilder()
 * - Can be used directly with createHandler() by including method
 */
export interface Handler<
  TRequestSchema extends ZodSchema | undefined = undefined,
  TQuerySchema extends ZodSchema | undefined = undefined,
  TError extends TypedError = TypedError,
  TResponseSchema extends ZodSchema | undefined = undefined,
> {
  /**
   * Schema to validate the request body against (optional)
   * Defaults to z.object({}) for GET requests, z.any() for others
   */
  requestSchema?: TRequestSchema;
  /**
   * Schema to validate query parameters against (optional)
   * Defaults to accepting any query parameters as Record<string, string>
   */
  querySchema?: TQuerySchema;
  /**
   * Schema to validate the response against (optional)
   * Defaults to z.any()
   */
  responseSchema?: TResponseSchema;
  /** Error to response mapping function */
  errorToResponse?: ErrorToResponse<TError>;
  /** Main handler function */
  handler: ApiRouteHandler<
    InferRequestType<TRequestSchema>,
    InferQueryType<TQuerySchema>,
    InferResponseType<TResponseSchema>
  >;
}
