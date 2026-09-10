import type { AppRoute } from "./types";

export type AppRouteMeta = {
  id: AppRoute;
  label: string;
};

export const APP_ROUTES: AppRouteMeta[] = [
  { id: "library", label: "Library" },
  { id: "discover", label: "Discover" },
  { id: "downloads", label: "Downloads" },
  { id: "settings", label: "Settings" },
];

export function isAppRoute(value: string | null): value is AppRoute {
  return value !== null && APP_ROUTES.some((route) => route.id === value);
}
