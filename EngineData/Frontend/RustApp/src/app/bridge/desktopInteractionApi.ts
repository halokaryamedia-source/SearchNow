import { invoke } from "@tauri-apps/api/core";

export const desktopInteractionApi = {
  chooseDownloadDirectory(): Promise<string | null> {
    return invoke<string | null>("choose_download_directory");
  },

  openDownloadDirectory(directory: string): Promise<void> {
    return invoke<void>("open_download_directory", { directory });
  },
};
