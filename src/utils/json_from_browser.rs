use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct JSONFromBrowser {
    pub sequential: bool,
    pub method: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub url: String,
    pub file_name: String,
    pub size: usize,
}
