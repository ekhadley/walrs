use crate::utils::get_cache;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

fn sanitize_path(image_path: &str) -> String {
    image_path
        .trim_start_matches('/')
        .replace('/', "_")
        .replace('.', "_")
}

pub fn get_cache_path(image_path: &str, send: bool) -> PathBuf {
    get_cache(send)
        .join("wal")
        .join("cache")
        .join(sanitize_path(image_path))
}

pub fn load_cached_colors(image_path: &str, send: bool) -> Option<Vec<(u8, u8, u8)>> {
    let cache_path = get_cache_path(image_path, send);
    let content = read_to_string(&cache_path).ok()?;

    let colors: Vec<(u8, u8, u8)> = content
        .lines()
        .filter(|line| line.starts_with('#') && line.len() >= 7)
        .filter_map(|hex| {
            u32::from_str_radix(&hex[1..7], 16)
                .ok()
                .map(|v| ((v >> 16) as u8, (v >> 8 & 0xFF) as u8, (v & 0xFF) as u8))
        })
        .collect();

    if colors.len() >= 16 {
        Some(colors)
    } else {
        None
    }
}

pub fn save_to_cache(image_path: &str, colors: &[(u8, u8, u8)], send: bool) {
    let cache_path = get_cache_path(image_path, send);
    if let Some(parent) = cache_path.parent() {
        let _ = create_dir_all(parent);
    }

    let content: String = colors
        .iter()
        .map(|(r, g, b)| format!("#{r:02x}{g:02x}{b:02x}"))
        .collect::<Vec<_>>()
        .join("\n");

    let _ = write(cache_path, content);
}
