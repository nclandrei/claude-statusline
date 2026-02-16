use serde::Deserialize;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct Input {
    model: Option<Model>,
    workspace: Option<Workspace>,
    context_window: Option<ContextWindow>,
}

#[derive(Deserialize)]
struct Model {
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct Workspace {
    current_dir: Option<String>,
}

#[derive(Deserialize)]
struct ContextWindow {
    used_percentage: Option<f64>,
}

#[derive(Deserialize, serde::Serialize)]
struct GitCache {
    branch: String,
    dir: String,
    ts: u64,
}

const CACHE_PATH: &str = "/tmp/claude-statusline-git-cache";
const CACHE_TTL: u64 = 5;

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn git_branch_cached(dir: &str) -> String {
    let now = now_secs();

    // Try reading cache
    if let Ok(data) = fs::read_to_string(CACHE_PATH) {
        if let Ok(cache) = serde_json::from_str::<GitCache>(&data) {
            if cache.dir == dir && now.saturating_sub(cache.ts) < CACHE_TTL {
                return cache.branch;
            }
        }
    }

    let branch = git_branch(dir);

    // Update cache (best-effort)
    let cache = GitCache {
        branch: branch.clone(),
        dir: dir.to_string(),
        ts: now,
    };
    let _ = fs::write(CACHE_PATH, serde_json::to_string(&cache).unwrap_or_default());

    branch
}

fn git_branch(dir: &str) -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(dir)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let b = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if b.is_empty() {
                "detached".to_string()
            } else {
                b
            }
        }
        _ => "no-git".to_string(),
    }
}

fn dir_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
}

fn main() {
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() || buf.trim().is_empty() {
        return;
    }

    let input: Input = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(_) => return,
    };

    let model = input
        .model
        .and_then(|m| m.display_name)
        .unwrap_or_default();

    let current_dir = input
        .workspace
        .and_then(|w| w.current_dir)
        .unwrap_or_default();

    let pct = input
        .context_window
        .and_then(|c| c.used_percentage)
        .unwrap_or(0.0) as u32;

    let dir = dir_name(&current_dir);
    let branch = if current_dir.is_empty() {
        "no-git".to_string()
    } else {
        git_branch_cached(&current_dir)
    };

    print!("{model} ◦ {dir} ◦ {pct}% ◦ {branch}");
}
