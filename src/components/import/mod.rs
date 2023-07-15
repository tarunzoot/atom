mod update;
mod view;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AtomImport {
    pub is_sequential: bool,
    pub import_file: String,
    pub download_path: String,
}
