use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct JSONFromBrowser {
    pub headers: HashMap<String, String>,
    pub url: String,
    pub file_name: String,
    pub size: usize,
}
