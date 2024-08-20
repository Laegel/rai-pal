use rai_pal_proc_macros::serializable_struct;

use crate::{
	game_engines::{
		game_engine::{EngineBrand, EngineVersionNumbers},
		unity::UnityScriptingBackend,
	},
	mod_loaders::mod_loader::ModLoaderId,
};

#[serializable_struct]
pub struct EngineVersionRange {
	pub minimum: Option<EngineVersionNumbers>,
	pub maximum: Option<EngineVersionNumbers>,
}

#[serializable_struct]
pub struct CommonModData {
	pub id: String,
	pub engine: Option<EngineBrand>,
	pub unity_backend: Option<UnityScriptingBackend>,
	pub engine_version_range: Option<EngineVersionRange>,
	pub loader_id: ModLoaderId,
}
