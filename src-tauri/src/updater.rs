use semver::Version;
use serde::Deserialize;

use crate::models::AppUpdateInfo;

const GITHUB_REPO: &str = "chiconghvan/mkvn-proxy-manager";
const GITHUB_API: &str = "https://api.github.com/repos";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    body: Option<String>,
    published_at: String,
    html_url: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

pub fn current_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

pub async fn check_for_updates(http: &reqwest::Client) -> Result<AppUpdateInfo, String> {
    let current_ver = current_version();
    let current = parse_version(&current_ver)?;

    let url = format!("{GITHUB_API}/{GITHUB_REPO}/releases?per_page=20");
    let releases: Vec<GitHubRelease> = http
        .get(&url)
        .header("User-Agent", "mkvn-proxy-manager")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch releases: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse releases: {e}"))?;

    let latest = releases
        .iter()
        .filter(|r| r.tag_name.starts_with('v'))
        .filter_map(|r| {
            let ver = parse_version(&r.tag_name).ok()?;
            Some((ver, r))
        })
        .max_by(|(a, _), (b, _)| a.cmp(b));

    match latest {
        Some((latest_ver, release)) if latest_ver > current => {
            let asset = release
                .assets
                .iter()
                .find(|a| a.name.ends_with("-x64-setup.exe") || a.name.ends_with(".msi") || a.name.ends_with(".zip"))
                .or_else(|| release.assets.first());

            let download_url = asset
                .map(|a| a.browser_download_url.clone())
                .unwrap_or_default();

            Ok(AppUpdateInfo {
                current_version: current_ver.clone(),
                new_version: release.tag_name.clone(),
                release_notes: release.body.clone().unwrap_or_default(),
                download_url,
                published_at: release.published_at.clone(),
                release_page_url: Some(release.html_url.clone()),
                update_available: true,
            })
        }
        _ => Ok(AppUpdateInfo {
            current_version: current_ver.clone(),
            new_version: current_ver,
            release_notes: String::new(),
            download_url: String::new(),
            published_at: String::new(),
            release_page_url: None,
            update_available: false,
        }),
    }
}

fn parse_version(tag: &str) -> Result<Version, String> {
    let v = tag.strip_prefix('v').unwrap_or(tag);
    Version::parse(v).map_err(|e| format!("Invalid version tag '{tag}': {e}"))
}
