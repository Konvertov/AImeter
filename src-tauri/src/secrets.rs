//! Reading sign-ins that other programs already saved, and keeping the
//! API keys the user typed into AImeter. Nothing here is ever logged.

use crate::providers::Info;


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
    stored(info.id).map(|key| key.trim().to_string())
}

/// Where AImeter keeps the secrets the user gave it (API keys, the Gemini
/// session).
///
/// macOS: a file only this user can read, in the app's own folder. The
/// Keychain is not used there on purpose: for an app without a paid Apple
/// signature the system asks for the password again after every update.
///
/// Windows and others: the system password store.
#[cfg(target_os = "macos")]
mod store {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    fn file() -> Option<PathBuf> {
        Some(dirs::config_dir()?.join("AImeter").join("secrets.json"))
    }

    fn load() -> BTreeMap<String, String> {
        file()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    fn save(all: &BTreeMap<String, String>) -> Result<(), String> {
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        let path = file().ok_or("no settings folder")?;
        let folder = path.parent().ok_or("no settings folder")?;
        std::fs::create_dir_all(folder).map_err(|e| e.to_string())?;
        let _ = std::fs::set_permissions(folder, std::fs::Permissions::from_mode(0o700));
        // Write next to the file and swap in, so a crash never leaves half a file.
        let draft = path.with_extension("json.new");
        let text = serde_json::to_string_pretty(all).map_err(|e| e.to_string())?;
        {
            use std::io::Write;
            let mut out = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&draft)
                .map_err(|e| e.to_string())?;
            out.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
        }
        let _ = std::fs::set_permissions(&draft, std::fs::Permissions::from_mode(0o600));
        std::fs::rename(&draft, &path).map_err(|e| e.to_string())
    }

    pub fn get(id: &str) -> Option<String> {
        load().remove(id)
    }

    pub fn set(id: &str, value: &str) -> Result<(), String> {
        let mut all = load();
        all.insert(id.to_string(), value.to_string());
        save(&all)
    }

    pub fn remove(id: &str) -> Result<(), String> {
        let mut all = load();
        if all.remove(id).is_some() {
            save(&all)?;
        }
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
mod store {
    const VAULT: &str = "AImeter";
    /// Windows keeps at most about 1280 characters per saved secret, and a
    /// browser session is longer, so long values are saved in pieces: the
    /// main entry says "pieces:N" and entries "id#0".."id#N-1" hold the text.
    const PIECE: usize = 1000;
    const PIECES_MARK: &str = "pieces:";

    fn entry(name: &str) -> Option<keyring::Entry> {
        keyring::Entry::new(VAULT, name).ok()
    }

    fn drop_entry(name: &str) -> Result<(), String> {
        match entry(name).ok_or("password store unavailable")?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn pieces_of(head: &str) -> Option<usize> {
        head.strip_prefix(PIECES_MARK).and_then(|n| n.parse().ok())
    }

    pub fn get(id: &str) -> Option<String> {
        let head = entry(id)?.get_password().ok()?;
        match pieces_of(&head) {
            Some(count) => {
                let mut whole = String::new();
                for index in 0..count {
                    whole.push_str(&entry(&format!("{id}#{index}"))?.get_password().ok()?);
                }
                Some(whole)
            }
            None => Some(head),
        }
    }

    pub fn remove(id: &str) -> Result<(), String> {
        if let Some(count) = entry(id).and_then(|e| e.get_password().ok()).and_then(|head| pieces_of(&head)) {
            for index in 0..count {
                drop_entry(&format!("{id}#{index}"))?;
            }
        }
        drop_entry(id)
    }

    pub fn set(id: &str, value: &str) -> Result<(), String> {
        remove(id)?;
        let write = |name: &str, text: &str| -> Result<(), String> {
            entry(name).ok_or("password store unavailable")?.set_password(text).map_err(|e| e.to_string())
        };
        let letters: Vec<char> = value.chars().collect();
        if letters.len() <= PIECE {
            return write(id, value);
        }
        let pieces: Vec<String> = letters.chunks(PIECE).map(|part| part.iter().collect()).collect();
        for (index, piece) in pieces.iter().enumerate() {
            write(&format!("{id}#{index}"), piece)?;
        }
        write(id, &format!("{PIECES_MARK}{}", pieces.len()))
    }
}

/// What was already read since the app started, so the store is asked once
/// per secret and not on every refresh.
static REMEMBERED: std::sync::Mutex<Option<std::collections::HashMap<String, Option<String>>>> =
    std::sync::Mutex::new(None);

/// A secret AImeter itself saved earlier under this service id.
pub fn stored(id: &str) -> Option<String> {
    if let Ok(guard) = REMEMBERED.lock() {
        if let Some(known) = guard.as_ref().and_then(|map| map.get(id)) {
            return known.clone();
        }
    }
    let value = store::get(id).filter(|value| !value.trim().is_empty());
    if let Ok(mut guard) = REMEMBERED.lock() {
        guard.get_or_insert_with(Default::default).insert(id.to_string(), value.clone());
    }
    value
}

/// Save (or, with an empty value, remove) a secret under this service id.
pub fn save_api_key(id: &str, key: &str) -> Result<(), String> {
    let key = key.trim();
    let result = if key.is_empty() { store::remove(id) } else { store::set(id, key) };
    if let Ok(mut guard) = REMEMBERED.lock() {
        let fresh = (result.is_ok() && !key.is_empty()).then(|| key.to_string());
        guard.get_or_insert_with(Default::default).insert(id.to_string(), fresh);
    }
    result
}
