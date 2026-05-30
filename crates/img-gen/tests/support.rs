use std::{path::PathBuf, sync::OnceLock};

/// Shared cache root for downloaded fonts used by integration tests.
pub fn typography_font_cache_root() -> PathBuf {
    static CACHE_ROOT: OnceLock<PathBuf> = OnceLock::new();
    CACHE_ROOT
        .get_or_init(|| {
            // let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let target_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
            let cache_root = target_dir.join("test-font-cache");
            std::fs::create_dir_all(&cache_root).unwrap();
            cache_root
        })
        .clone()
}
