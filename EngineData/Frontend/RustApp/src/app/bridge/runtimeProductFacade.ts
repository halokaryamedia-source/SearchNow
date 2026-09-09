import type { ProductRuntimeSnapshot } from "../shared/types";
import { runtimeApi } from "./runtimeApi";

export async function loadProductRuntimeSnapshot(): Promise<ProductRuntimeSnapshot> {
  try {
    const runtime = await runtimeApi.getRuntimeStatus();
    return {
      ready: runtime.appReady,
      summary: runtime.appReady ? "Desktop runtime ready" : "Desktop runtime unavailable",
      runtime,
    };
  } catch {
    return {
      ready: false,
      summary: "Desktop runtime unavailable",
      runtime: null,
    };
  }
}

export const runtimeProductFacade = {
  loadProductRuntimeSnapshot,
};
