import type { AppRoute } from "./types";

export type AppRouteMeta = {
  id: AppRoute;
  label: string;
  kicker: string;
  description: string;
};

export const APP_ROUTES: AppRouteMeta[] = [
  { id: "library", label: "Library", kicker: "Local content", description: "Browse detected Bedrock worlds and packs" },
  { id: "discover", label: "Discover", kicker: "Catalog", description: "Search connected content sources" },
  { id: "downloads", label: "Downloads", kicker: "Transfers", description: "Track queue progress and download history" },
  { id: "settings", label: "Settings", kicker: "Application", description: "Manage discovery and runtime preferences" },
];

export function isAppRoute(value: string | null): value is AppRoute {
  return value !== null && APP_ROUTES.some((route) => route.id === value);
}

export function routeMeta(route: AppRoute): AppRouteMeta {
  return APP_ROUTES.find((item) => item.id === route) ?? APP_ROUTES[0];
}
