import type { CheckSchemaResponse } from "@/bindings/CheckSchemaResponse";
import { apiFetch, type ApiResult } from "./api";

export async function listChecks(): Promise<ApiResult<CheckSchemaResponse[]>> {
    return apiFetch<CheckSchemaResponse[]>("/admin/listAvailableChecks", "GET", {});
}
