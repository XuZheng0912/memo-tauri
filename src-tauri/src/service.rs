mod note;

#[tauri::command]
pub fn get_value_by_key(key: &str) -> String {
    "1111".to_string()
}