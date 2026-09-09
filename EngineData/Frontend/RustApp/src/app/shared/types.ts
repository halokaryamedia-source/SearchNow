export type AppRoute = "library" | "discover" | "downloads" | "settings";

export type RuntimeStatus = {
  appReady: boolean;
  appVersion: string;
  platform: string;
  architecture: string;
  backend: string;
};

export type ProductRuntimeSnapshot = {
  ready: boolean;
  summary: string;
  runtime: RuntimeStatus | null;
};
