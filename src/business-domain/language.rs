use serde::Serialize;
use uuid::Uuid;
use strum_macros::{EnumString, Display};


pub enum LanguageState {
    NotStarted,
    CompileStarted,
    CompileInProgress,
    Compiled,
    Paused,
    Failed
}

#[derive(Serialize)]
pub struct Language {
    pub user_uuid: String,
    pub language_uuid: String,
    pub language_type: String,
    pub state: LanguageState,
    pub source: String,
    pub result_file: Option<String>, 

}

impl Language {
    pub fn new(user_uuid: String, language_type: String, source_file: String) -> Language {
        Language {
            user_uuid, language_uuid: Uuuid::new_v4().to_string(),
            language_type, state: LanguageState::NotStarted,
            source_file, result_file: None
        }
    }
    pub fn get_global_id(&self) -> String {
        format!("{}_{}", self.user_uuid, self.language_uuid);
    }
}
