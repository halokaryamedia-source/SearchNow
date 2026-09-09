use std::{env, fs, path::PathBuf};

fn main() {
    #[cfg(target_os = "windows")]
    {
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be available"));
        let icon_path = out_dir.join("searchnow-build-placeholder.ico");
        fs::write(&icon_path, build_placeholder_ico()).expect("write build placeholder icon");

        let windows = tauri_build::WindowsAttributes::new().window_icon_path(&icon_path);
        let attributes = tauri_build::Attributes::new().windows_attributes(windows);
        tauri_build::try_build(attributes).expect("failed to run Tauri build script");
        return;
    }

    #[cfg(not(target_os = "windows"))]
    tauri_build::build();
}

#[cfg(target_os = "windows")]
fn build_placeholder_ico() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(70);

    // ICONDIR: one icon image.
    bytes.extend_from_slice(&[0, 0, 1, 0, 1, 0]);
    // ICONDIRENTRY: 1x1, 32bpp, 48-byte image payload starting at byte 22.
    bytes.extend_from_slice(&[1, 1, 0, 0, 1, 0, 32, 0]);
    bytes.extend_from_slice(&48_u32.to_le_bytes());
    bytes.extend_from_slice(&22_u32.to_le_bytes());

    // BITMAPINFOHEADER. Height is doubled because ICO stores XOR + AND masks.
    bytes.extend_from_slice(&40_u32.to_le_bytes());
    bytes.extend_from_slice(&1_i32.to_le_bytes());
    bytes.extend_from_slice(&2_i32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&32_u16.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&4_u32.to_le_bytes());
    bytes.extend_from_slice(&0_i32.to_le_bytes());
    bytes.extend_from_slice(&0_i32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());

    // One opaque black BGRA pixel + one DWORD-aligned transparent AND-mask row.
    bytes.extend_from_slice(&[0, 0, 0, 255]);
    bytes.extend_from_slice(&[0, 0, 0, 0]);
    bytes
}
