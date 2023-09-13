use std::path::{Path, PathBuf};

use anyhow::anyhow;
use serde::Serialize;
use specta::Type;

use crate::game_executable::{GameExecutable, UnityScriptingBackend};
use crate::Result;

#[derive(Serialize, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub name: String,
    pub scripting_backend: UnityScriptingBackend,
    path: PathBuf,
}

impl Mod {
    pub fn new(path: &Path, scripting_backend: &UnityScriptingBackend) -> Result<Self> {
        Ok(Mod {
            path: path.to_path_buf(),
            name: String::from(
                path.file_name()
                    .ok_or(anyhow!("Failed to get file name"))?
                    .to_string_lossy(),
            ),
            scripting_backend: scripting_backend.clone(),
        })
    }

    fn install(&self, game_executable: &GameExecutable) {
        todo!()
    }
}