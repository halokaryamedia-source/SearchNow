import type {
  AppSettings,
  BackendDiagnosticsSnapshot,
  CatalogPage,
  CatalogRequest,
  DownloadJob,
  DownloadManagerSnapshot,
  LocalBackendSnapshot,
  MinecraftDiscoverySnapshot,
  ProductError,
  ProductResult,
  ProductRuntimeSnapshot,
  QueueCatalogDownloadRequest,
} from "../shared/types";
import { runtimeApi } from "./runtimeApi";

function normalizeProductError(error: unknown, fallbackMessage: string): ProductError {
  if (typeof error === "object" && error !== null) {
    const candidate = error as { code?: unknown; message?: unknown };
    if (typeof candidate.message === "string") {
      return {
        code: typeof candidate.code === "string" ? candidate.code : "runtime_command_failed",
        message: candidate.message,
      };
    }
  }
  if (typeof error === "string" && error.trim()) {
    return { code: "runtime_command_failed", message: error };
  }
  return { code: "runtime_command_failed", message: fallbackMessage };
}

async function productCall<T>(operation: () => Promise<T>, fallbackMessage: string): Promise<ProductResult<T>> {
  try {
    return { ok: true, data: await operation() };
  } catch (error) {
    return { ok: false, error: normalizeProductError(error, fallbackMessage) };
  }
}

export async function loadProductRuntimeSnapshot(): Promise<ProductRuntimeSnapshot> {
  const backendResult = await productCall(
    () => runtimeApi.getBackendSnapshot(),
    "SearchNow could not read the desktop runtime snapshot.",
  );

  if (backendResult.ok) {
    const { runtime, diagnostics } = backendResult.data;
    const degraded = diagnostics.health.state === "degraded";
    return {
      ready: runtime.appReady,
      summary: !runtime.appReady
        ? "Desktop runtime unavailable"
        : degraded
          ? "Desktop runtime ready with diagnostics"
          : "Desktop runtime ready",
      runtime,
      backend: backendResult.data,
      error: null,
    };
  }

  const runtimeResult = await productCall(
    () => runtimeApi.getRuntimeStatus(),
    "SearchNow could not connect to the desktop runtime.",
  );
  if (runtimeResult.ok) {
    return {
      ready: runtimeResult.data.appReady,
      summary: runtimeResult.data.appReady ? "Desktop runtime ready" : "Desktop runtime unavailable",
      runtime: runtimeResult.data,
      backend: null,
      error: backendResult.error,
    };
  }

  return {
    ready: false,
    summary: "Desktop runtime unavailable",
    runtime: null,
    backend: null,
    error: runtimeResult.error,
  };
}

export const runtimeProductFacade = {
  loadProductRuntimeSnapshot,

  loadLibrary(): Promise<ProductResult<LocalBackendSnapshot>> {
    return productCall(
      () => runtimeApi.scanLocalLibrary(),
      "SearchNow could not scan the local Minecraft library.",
    );
  },

  discoverMinecraft(): Promise<ProductResult<MinecraftDiscoverySnapshot>> {
    return productCall(
      () => runtimeApi.discoverMinecraft(),
      "SearchNow could not detect Minecraft Bedrock storage.",
    );
  },

  loadSettings(): Promise<ProductResult<AppSettings>> {
    return productCall(
      () => runtimeApi.loadAppSettings(),
      "SearchNow could not load application settings.",
    );
  },

  saveSettings(settings: AppSettings): Promise<ProductResult<AppSettings>> {
    return productCall(
      () => runtimeApi.saveAppSettings(settings),
      "SearchNow could not save application settings.",
    );
  },

  loadDownloads(): Promise<ProductResult<DownloadManagerSnapshot>> {
    return productCall(
      () => runtimeApi.getDownloadSnapshot(),
      "SearchNow could not read the download queue.",
    );
  },

  queueCatalogDownload(request: QueueCatalogDownloadRequest): Promise<ProductResult<DownloadJob>> {
    return productCall(
      () => runtimeApi.queueCatalogDownload(request),
      "SearchNow could not queue this catalog download.",
    );
  },

  cancelDownload(jobId: string): Promise<ProductResult<DownloadJob>> {
    return productCall(
      () => runtimeApi.cancelDownload(jobId),
      "SearchNow could not cancel this download.",
    );
  },

  retryDownload(jobId: string): Promise<ProductResult<DownloadJob>> {
    return productCall(
      () => runtimeApi.retryDownload(jobId),
      "SearchNow could not retry this download.",
    );
  },

  removeDownload(jobId: string): Promise<ProductResult<DownloadManagerSnapshot>> {
    return productCall(
      () => runtimeApi.removeDownload(jobId),
      "SearchNow could not remove this download from history.",
    );
  },

  loadDiagnostics(): Promise<ProductResult<BackendDiagnosticsSnapshot>> {
    return productCall(
      () => runtimeApi.getBackendDiagnostics(),
      "SearchNow could not read runtime diagnostics.",
    );
  },

  queryCatalog(request: CatalogRequest): Promise<ProductResult<CatalogPage>> {
    return productCall(
      () => runtimeApi.queryCatalog(request),
      "SearchNow could not query the selected catalog provider.",
    );
  },
};
