pub fn get_storage_item(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}

pub fn set_storage_item(key: &str, value: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(key, value);
        }
    }
}

pub fn get_storage_u32(key: &str, default_val: u32) -> u32 {
    match get_storage_item(key) {
        Some(s) => s.parse::<u32>().unwrap_or(default_val),
        None => default_val,
    }
}

pub fn set_storage_u32(key: &str, value: u32) {
    set_storage_item(key, &value.to_string());
}

pub fn get_storage_bool(key: &str, default_val: bool) -> bool {
    match get_storage_item(key) {
        Some(s) => s == "true" || s == "1",
        None => default_val,
    }
}

pub fn set_storage_bool(key: &str, value: bool) {
    set_storage_item(key, if value { "true" } else { "false" });
}
