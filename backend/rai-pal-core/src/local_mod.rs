use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

use rai_pal_proc_macros::serializable_struct;

use crate::{
	game_engines::{game_engine::EngineBrand, unity::UnityScriptingBackend},
	game_mod::CommonModData,
	mod_loaders::mod_loader::ModLoaderId,
	mod_manifest::{self, Manifest},
	paths::{self, open_folder_or_parent},
	result::Result,
	serializable_enum,
};

serializable_enum!(ModKind {
	Installable,
	Runnable,
});

#[serializable_struct]
pub struct LocalModData {
	pub path: PathBuf,
	pub manifest: Option<Manifest>,
}

#[serializable_struct]
pub struct LocalMod {
	pub data: LocalModData,
	pub common: CommonModData,
}

pub fn get_manifest_path(mod_path: &Path) -> PathBuf {
	mod_path.join(mod_manifest::Manifest::FILE_NAME)
}

impl LocalMod {
	pub fn new(
		loader_id: &ModLoaderId,
		path: &Path,
		engine: Option<EngineBrand>,
		unity_backend: Option<UnityScriptingBackend>,
	) -> Result<Self> {
		let manifest = mod_manifest::get(&get_manifest_path(path));

		Ok(Self {
			common: CommonModData {
				id: paths::file_name_without_extension(path)?.to_string(),
				engine,
				engine_version_range: manifest
					.as_ref()
					.and_then(|m| m.engine_version_range.clone()),
				unity_backend,
				loader_id: loader_id.clone(),
			},
			data: LocalModData {
				path: path.to_path_buf(),
				manifest,
			},
		})
	}

	pub fn open_folder(&self) -> Result {
		open_folder_or_parent(&self.data.path)
	}
}

pub type Map = HashMap<String, LocalMod>;
