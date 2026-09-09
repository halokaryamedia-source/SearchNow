import { invoke } from "@tauri-apps/api/core";
import type { RuntimeStatus } from "../shared/types";

export const runtimeApi = {
  getRuntimeStatus(): Promise<RuntimeStatus> {
    return invoke<RuntimeStatus>("get_runtime_status");
  },
};
