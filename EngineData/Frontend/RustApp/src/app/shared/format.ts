import type { CatalogContentType, DownloadJobState, LocalContentType } from "./types";

export function formatBytes(value: number | null | undefined): string {
  if (value == null) return "Unknown size";
  if (value < 1024) return `${value} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let amount = value / 1024;
  let unit = 0;
  while (amount >= 1024 && unit < units.length - 1) {
    amount /= 1024;
    unit += 1;
  }
  return `${amount >= 10 ? amount.toFixed(0) : amount.toFixed(1)} ${units[unit]}`;
}

export function formatDateTime(timestampMs: number): string {
  if (!timestampMs) return "Unknown time";
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(timestampMs));
}

export function localContentTypeLabel(type: LocalContentType): string {
  return {
    behaviorPack: "Behavior Pack",
    resourcePack: "Resource Pack",
    skinPack: "Skin Pack",
    world: "World",
  }[type];
}

export function catalogContentTypeLabel(type: CatalogContentType): string {
  return {
    world: "World",
    addon: "Add-On",
    resourcePack: "Resource Pack",
    skin: "Skin",
    persona: "Persona",
    other: "Other",
  }[type];
}

export function downloadStateLabel(state: DownloadJobState): string {
  return {
    queued: "Queued",
    preparing: "Preparing",
    transferring: "Downloading",
    finalizing: "Finalizing",
    cancelRequested: "Cancelling",
    completed: "Finished",
    failed: "Failed",
    cancelled: "Cancelled",
    interrupted: "Interrupted",
  }[state];
}

export function progressPercent(downloaded: number, total: number | null): number | null {
  if (total == null || total <= 0) return null;
  return Math.max(0, Math.min(100, Math.round((downloaded / total) * 100)));
}
