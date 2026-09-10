import { invoke } from "@tauri-apps/api/core";

export const desktopInteractionApi = {
  chooseDownloadDirectory(): Promise<string | null> {
    return invoke<string | null>("choose_download_directory");
  },
};
