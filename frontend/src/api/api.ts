import type { ApiError } from "@/bindings/ApiError";
import type { ApiResponse } from "@/bindings/ApiResponse";

export type ApiResult<T> = ApiError | ApiResponse<T>;

const API_ROOT = "localhost:3000"

// export async function apiFetchFactory<T>(path: string, method: "GET" | "POST"): Promise<(body: Record<string, any> | undefined) => Promise<ApiResult<T>>> {
//     return (body: Record<string, any> | undefined) => {
//         return apiFetch(path, method, body ?? {})
//     }
// }

export async function apiFetch<T>(path: string, method: "GET" | "POST", body: Record<string, any>): Promise<ApiResult<T>> {
    return fetch(API_ROOT + path, {
        method: method,
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(body)
    })
        .then(response => {
            return response.json();
        })
        .then(data => {
            return data;
        })
}
