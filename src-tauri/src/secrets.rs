//! Reading sign-ins that other programs already saved, and keeping the
//! API keys the user typed into AImeter. Nothing here is ever logged.

use crate::providers::Info;

const VAULT: &str = "AImeter";

/// A secret another program put into the system password store.
/// macOS: Keychain item by service name. Windows: Credential Manager target.
#[cfg(target_os = "macos")]
pub fn system_secret(name: &str) -> Option<String> {
    let out = std::process::Command::new("/usr/bin/security")
        .args(["find-generic-password", "-s", name, "-w"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
}

#[cfg(windows)]
pub fn system_secret(name: &str) -> Option<String> {
    use windows_sys::Win32::Security::Credentials::{
        CredFree, CredReadW, CREDENTIALW, CRED_TYPE_GENERIC,
    };
    let target: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut found: *mut CREDENTIALW = std::ptr::null_mut();
    // SAFETY: `target` is NUL-terminated, `found` is freed with CredFree below.
    let bytes = unsafe {
        if CredReadW(target.as_ptr(), CRED_TYPE_GENERIC, 0, &mut found) == 0 || found.is_null() {
            return None;
        }
        let entry = &*found;
        let bytes = if entry.CredentialBlob.is_null() {
            Vec::new()
        } else {
            std::slice::from_raw_parts(entry.CredentialBlob, entry.CredentialBlobSize as usize)
                .to_vec()
        };
        CredFree(found as *const _);
        bytes
    };
    let text = match String::from_utf8(bytes.clone()) {
        Ok(text) if !text.contains('\0') => text,
        _ => {
            let wide: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect();
            String::from_utf16(&wide).ok()?
        }
    };
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
}

#[cfg(not(any(target_os = "macos", windows)))]
pub fn system_secret(_name: &str) -> Option<String> {
    None
}

/// The API key for a key-based service: environment variable first,
/// then the key saved through AImeter's settings.
pub fn api_key(info: &Info) -> Option<String> {
    let variable = info.key_env?;
    if let Ok(value) = std::env::var(variable) {
        let value = value.trim().to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }
    keyring::Entry::new(VAULT, info.id)
        .ok()?
        .get_password()
        .ok()
        .map(|key| key.trim().to_string())
        .filter(|key| !key.is_empty())
}

pub fn save_api_key(id: &str, key: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(VAULT, id).map_err(|e| e.to_string())?;
    if key.trim().is_empty() {
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        entry.set_password(key.trim()).map_err(|e| e.to_string())
    }
}
