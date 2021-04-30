use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Mutex;

pub fn path_str(path: &Path) -> Result<&str> {
    path.to_str().ok_or_else(|| anyhow!("path is not valid"))
}

// VIDEO_EXTS is collected from: https://en.wikipedia.org/wiki/Video_file_format
static VIDEO_EXTS: Lazy<Mutex<HashSet<&'static str>>> = Lazy::new(|| {
    let set = [
        "webm", "mkv", "flv", "flv", "vob", "ogv", "ogg", "drc", "gif", "gif", "mng", "avi", "mts",
        "m2t", "ts", "mov", "qt", "wmv", "yuv", "rm", "rmv", "viv", "asf", "amv", "mp4", "m4p",
        "m4v", "mpg", "mp2", "mpe", "mpe", "mpv", "mpg", "mpe", "m2v", "m4v", "svi", "3gp", "3g2",
        "mxf", "roq", "nsv", "flv", "f4v", "f4p", "f4a", "f4b",
    ]
    .iter()
    .cloned()
    .collect();
    Mutex::new(set)
});

pub fn is_video_file(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
        .to_lowercase();
    VIDEO_EXTS.lock().unwrap().contains(&ext as &str)
}
