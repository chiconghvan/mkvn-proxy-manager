use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

static HOST_PORT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([a-z0-9][a-z0-9.-]*|\[[0-9a-f:]+\]|\d{1,3}(?:\.\d{1,3}){3}):(\d{2,5})").expect("valid regex")
});

pub fn extract_host_port(proxy: &str) -> Option<(String, u16)> {
    let trimmed = proxy.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(url) = Url::parse(trimmed) {
        if let (Some(host), Some(port)) = (url.host_str(), url.port()) {
            return Some((host.trim_matches(['[', ']']).to_ascii_lowercase(), port));
        }
    }

    if let Some(caps) = HOST_PORT_RE.captures(trimmed) {
        let host = caps.get(1)?.as_str().trim_matches(['[', ']']).to_ascii_lowercase();
        let port = caps.get(2)?.as_str().parse::<u16>().ok()?;
        return Some((host, port));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::extract_host_port;

    #[test]
    fn parses_supported_formats() {
        assert_eq!(extract_host_port("1.2.3.4:8080:user:pass"), Some(("1.2.3.4".into(), 8080)));
        assert_eq!(extract_host_port("1.2.3.4:8080"), Some(("1.2.3.4".into(), 8080)));
        assert_eq!(extract_host_port("socks5://user:pass@1.2.3.4:1080"), Some(("1.2.3.4".into(), 1080)));
        assert_eq!(extract_host_port("http://example.com:3128"), Some(("example.com".into(), 3128)));
        assert_eq!(extract_host_port(""), None);
    }
}
