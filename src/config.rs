//! NNTP server and article configuration types.

use serde::{Deserialize, Serialize};

/// NNTP server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Unique server identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Server hostname
    pub host: String,
    /// Server port
    pub port: u16,
    /// Use SSL/TLS
    pub ssl: bool,
    /// Verify SSL certificates
    pub ssl_verify: bool,
    /// Username for authentication
    pub username: Option<String>,
    /// Password for authentication
    pub password: Option<String>,
    /// Max simultaneous connections
    pub connections: u16,
    /// Server priority (0 = highest)
    pub priority: u8,
    /// Enable this server
    pub enabled: bool,
    /// Article retention in days (0 = unlimited)
    pub retention: u32,
    /// Number of pipelined requests per connection
    pub pipelining: u8,
    /// Server is optional (failure is non-fatal)
    pub optional: bool,
    /// Enable XFEATURE COMPRESS GZIP negotiation
    #[serde(default)]
    pub compress: bool,
    /// Optional SOCKS5 proxy URL: `socks5://[username:password@]host:port`
    #[serde(default)]
    pub proxy_url: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            host: String::new(),
            port: 563,
            ssl: true,
            ssl_verify: true,
            username: None,
            password: None,
            connections: 4,
            priority: 0,
            enabled: true,
            retention: 0,
            pipelining: 20,
            optional: false,
            compress: false,
            proxy_url: None,
        }
    }
}

/// Entry from `LIST ACTIVE` response.
///
/// Each line: `groupname last first posting_flag`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActiveEntry {
    /// Newsgroup name (e.g., "alt.binaries.test")
    pub name: String,
    /// Highest article number
    pub high: u64,
    /// Lowest article number
    pub low: u64,
    /// Posting flag (y = posting allowed, n = no posting, m = moderated)
    pub status: String,
}

/// A Usenet article segment to be downloaded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Message-ID (e.g., "abc123@example.com")
    pub message_id: String,
    /// Segment number (1-based part number)
    pub segment_number: u32,
    /// Encoded size in bytes
    pub bytes: u64,
    /// Has this article been downloaded?
    pub downloaded: bool,
    /// Byte offset in the final file (set after yEnc decode)
    pub data_begin: Option<u64>,
    /// Size of decoded data for this segment
    pub data_size: Option<u64>,
    /// CRC32 of decoded data
    pub crc32: Option<u32>,
    /// Servers that have been tried for this article
    pub tried_servers: Vec<String>,
    /// Number of fetch attempts
    pub tries: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_serde_roundtrip() {
        let article = Article {
            message_id: "abc123@example.com".to_string(),
            segment_number: 1,
            bytes: 500_000,
            downloaded: false,
            data_begin: Some(0),
            data_size: Some(499_000),
            crc32: Some(0xDEADBEEF),
            tried_servers: vec!["server1".to_string()],
            tries: 2,
        };

        let json = serde_json::to_string(&article).unwrap();
        let deserialized: Article = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.message_id, "abc123@example.com");
        assert_eq!(deserialized.segment_number, 1);
        assert_eq!(deserialized.bytes, 500_000);
        assert!(!deserialized.downloaded);
        assert_eq!(deserialized.data_begin, Some(0));
        assert_eq!(deserialized.data_size, Some(499_000));
        assert_eq!(deserialized.crc32, Some(0xDEADBEEF));
        assert_eq!(deserialized.tried_servers, vec!["server1"]);
        assert_eq!(deserialized.tries, 2);
    }

    #[test]
    fn test_server_config_serde_roundtrip() {
        let config = ServerConfig {
            id: "srv1".to_string(),
            name: "My Server".to_string(),
            host: "news.example.com".to_string(),
            port: 563,
            ssl: true,
            ssl_verify: true,
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            connections: 8,
            priority: 0,
            enabled: true,
            retention: 3000,
            pipelining: 20,
            optional: false,
            compress: true,
            proxy_url: Some("socks5://proxy:1080".to_string()),
        };

        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: ServerConfig = toml::from_str(&toml_str).unwrap();

        assert_eq!(deserialized.id, "srv1");
        assert_eq!(deserialized.host, "news.example.com");
        assert_eq!(deserialized.port, 563);
        assert!(deserialized.ssl);
        assert_eq!(deserialized.connections, 8);
        assert_eq!(deserialized.retention, 3000);
        assert!(deserialized.compress);
        assert_eq!(deserialized.proxy_url, Some("socks5://proxy:1080".to_string()));
    }

    #[test]
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 563);
        assert!(config.ssl);
        assert!(config.ssl_verify);
        assert_eq!(config.connections, 4);
        assert_eq!(config.priority, 0);
        assert!(config.enabled);
        assert_eq!(config.retention, 0);
        assert_eq!(config.pipelining, 20);
        assert!(!config.optional);
        assert!(!config.compress);
        assert!(config.proxy_url.is_none());
    }

    #[test]
    fn test_list_active_entry_serde() {
        let entry = ListActiveEntry {
            name: "alt.binaries.test".to_string(),
            high: 1_000_000,
            low: 1,
            status: "y".to_string(),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: ListActiveEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "alt.binaries.test");
        assert_eq!(deserialized.high, 1_000_000);
        assert_eq!(deserialized.low, 1);
        assert_eq!(deserialized.status, "y");
    }
}
