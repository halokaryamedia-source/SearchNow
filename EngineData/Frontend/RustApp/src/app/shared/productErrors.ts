import type { ProductError } from "./types";

const FRIENDLY_MESSAGES: Record<string, string> = {
  catalog_provider_unavailable: "The content source is currently unavailable.",
  catalog_provider_query_failed: "The content source could not complete this request.",
  catalog_provider_data_invalid: "The content source returned unsupported data.",
  catalog_query_invalid: "The search request could not be processed.",
  download_transfer_timeout: "The download timed out. Try again.",
  download_transfer_read_failed: "The download was interrupted. Try again.",
  download_transfer_incomplete: "The download ended before the file was complete.",
  download_payload_write_failed: "The file could not be written to disk.",
  download_payload_sync_failed: "The completed file could not be saved safely.",
  download_destination_name_invalid: "The file name is not supported on this device.",
  download_finalize_incomplete: "The download is not complete yet.",
  download_scheduler_failed: "Downloads are temporarily paused. Refresh to continue.",
  download_transport_unavailable: "This download source is currently unavailable.",
  dialog_path_invalid: "The selected folder could not be used.",
  directory_open_invalid: "This folder could not be opened.",
  directory_open_failed: "This folder could not be opened.",
  settings_load_failed: "SearchNow could not load your settings.",
  settings_save_failed: "SearchNow could not save your settings.",
};

export function toProductError(error: unknown, fallbackMessage: string): ProductError {
  let code = "runtime_command_failed";
  let rawMessage = "";

  if (typeof error === "object" && error !== null) {
    const candidate = error as { code?: unknown; message?: unknown };
    if (typeof candidate.code === "string" && candidate.code.trim()) code = candidate.code;
    if (typeof candidate.message === "string") rawMessage = candidate.message.trim();
  } else if (typeof error === "string") {
    rawMessage = error.trim();
  }

  return {
    code,
    message: FRIENDLY_MESSAGES[code] ?? (rawMessage || fallbackMessage),
  };
}
