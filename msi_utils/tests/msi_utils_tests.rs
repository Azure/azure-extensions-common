use http::Uri;
use msi_utils::downloader_common as dc;

const STORAGE_URI: &str = "https://validstorage.blob.core.windows.net";
const KEY_VAULT_URI: &str = "https://validkv.vault.azure.net/";
const INVALID_URI: &str = "abc://invaliduri.com";

#[test]
pub fn test_is_azure_blob_storage_uri() {
    assert!(dc::is_azure_blob_storage_uri(STORAGE_URI.parse::<Uri>().unwrap()));
    assert!(!dc::is_azure_blob_storage_uri(KEY_VAULT_URI.parse::<Uri>().unwrap()));
    assert!(!dc::is_azure_blob_storage_uri(INVALID_URI.parse::<Uri>().unwrap()));
}

#[test]
pub fn test_is_azure_key_vault_uri() {
    assert!(dc::is_azure_key_vault_uri(KEY_VAULT_URI.parse::<Uri>().unwrap()));
    assert!(!dc::is_azure_key_vault_uri(STORAGE_URI.parse::<Uri>().unwrap()));
    assert!(!dc::is_azure_key_vault_uri(INVALID_URI.parse::<Uri>().unwrap()));
}
