use http::Uri;
use http::uri::Scheme;
use crate::constants::{AZURE_BLOB_IDENTIFIERS, AZURE_KEY_VAULT_IDENTIFIERS};

pub fn is_azure_blob_storage_uri(uri: Uri) -> bool {
    if uri.scheme() != Some(&Scheme::HTTP) && uri.scheme() != Some(&Scheme::HTTPS) {
        return false;
    }

    let host = uri.host().unwrap().to_lowercase();
    return AZURE_BLOB_IDENTIFIERS.iter().any(|&s| host.to_string().contains(&s));
}

pub fn is_azure_key_vault_uri(uri: Uri) -> bool {
    if uri.scheme() != Some(&Scheme::HTTP) && uri.scheme() != Some(&Scheme::HTTPS) {
        return false;
    }

    let host = uri.host().unwrap().to_lowercase();
    return AZURE_KEY_VAULT_IDENTIFIERS.iter().any(|&s| host.to_string().contains(&s));
}
