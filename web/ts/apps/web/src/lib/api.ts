// API client for communicating with the backend

const API_BASE_URL = import.meta.env.VITE_API_URL || "http://localhost:3001";

export interface User {
  id: string;
  email: string;
  name?: string;
  picture?: string;
}

export interface ApiError {
  error: string;
  message?: string;
}

class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const url = `${this.baseUrl}${path}`;

    const response = await fetch(url, {
      ...options,
      credentials: "include", // Important: include cookies in requests
      headers: {
        "Content-Type": "application/json",
        ...options.headers,
      },
    });

    if (!response.ok) {
      if (response.status === 401) {
        // Redirect to login on 401
        window.location.href = `${this.baseUrl}/app/login`;
        throw new Error("Unauthorized");
      }

      const error = (await response.json()) as ApiError;
      throw new Error(error.error || "API request failed");
    }

    return response.json();
  }

  // Example API calls
  async echo(message: string): Promise<{ message: string }> {
    return this.request("/api/v0/echo", {
      method: "POST",
      body: JSON.stringify({ message }),
    });
  }
}

export const api = new ApiClient(API_BASE_URL);
