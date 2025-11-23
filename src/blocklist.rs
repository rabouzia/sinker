use serde::Deserialize;
use std::{any, collections::HashSet};

#[derive(Debug, Deserialize)]
pub struct Blocklist {
    domains: HashSet<String>,
}

impl Blocklist {
    pub fn new(path: &str) -> Self {
        let mut domains = HashSet::new();
        if let Ok(contents) = std::fs::read_to_string(path) {
            for line in contents.lines() {
                let domain = line.trim();
                if !domain.is_empty() && !domain.starts_with('#') {
                    domains.insert(domain.to_string());
                }
            }
        }
        Blocklist { domains }
    }

    pub fn is_blocked(&self, domain: &str) -> bool {
        self.domains.contains(domain)
    }

    pub fn block_check(item: &str, blocklist: HashSet<String>) -> bool {
        blocklist.contains(item)
    }
}
