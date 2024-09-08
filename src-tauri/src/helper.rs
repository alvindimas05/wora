use entity::prelude::Settings;
use crate::AppState;

#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, ()>{
    Settings::find().one(&*state.db).await.expect("Can't find settings data.")
}