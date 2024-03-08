pub const FILE_DOWNLOAD_RETRY_LINEAR_INTERVAL: isize = 15;                      // seconds
pub const MAX_PER_FILE_DOWNLOAD_RETRY_TIME: isize = 10 * 60;                    // seconds - 10 minutes
pub const FILE_DOWNLOAD_RETRY_INTERVAL_WITH_EXPONENTIAL_BACKOFF: isize = 2;     // seconds
pub const FILE_DOWNLOAD_RETRY_ATTEMPTS_WITH_EXPONENTIAL_BACKOFF: isize = 7;

pub const AZURE_BLOB_IDENTIFIERS: [&str; 2] = [".blob.core", ".blob.azurestack"];
pub const AZURE_KEY_VAULT_IDENTIFIERS: [&str; 4] = [".vault.azure.net", ".vault.azure.cn", ".vault.usgovcloudapi.net", ".vault.microsoftazure.de"];
