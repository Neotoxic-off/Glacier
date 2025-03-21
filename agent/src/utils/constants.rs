pub const LOG_DIRECTORY: &str = "/glacier-logs";
pub const REPORT_DIRECTORY: &str = "/glacier-reports";
pub const GLACIER_DIRECTORY: &str = "/glacier";

pub const COLLECTION_NAME_SIGNATURES: &str = "signatures";
pub const COLLECTION_NAME_CATALOG: &str = "catalog";

pub const CHUNK_SIZE: usize = 1024;

pub const CDC_WINDOW_SIZE: usize = 48;
pub const CDC_AVERAGE_CHUNK_SIZE: usize = 1024 * 4;
pub const CDC_MASK_S: u32 = 13;
pub const CDC_MASK: u32 = (1 << CDC_MASK_S) - 1;