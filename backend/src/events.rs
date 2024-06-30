use rai_pal_proc_macros::serializable_event;

use crate::{
	installed_game, local_mod, mod_loaders::mod_loader, owned_game, remote_game, remote_mod,
};

#[serializable_event]
pub struct FoundInstalledGame(pub installed_game::InstalledGame);

#[serializable_event]
pub struct GameAdded(pub String);

#[serializable_event]
pub struct FoundOwnedGame(pub owned_game::OwnedGame);

#[serializable_event]
pub struct FoundRemoteGame(pub remote_game::RemoteGame);

#[serializable_event]
pub struct SyncRemoteGames(pub remote_game::Map);

#[serializable_event]
pub struct SyncModLoaders(pub mod_loader::DataMap);

#[serializable_event]
pub struct SyncLocalMods(pub local_mod::Map);

#[serializable_event]
pub struct SyncRemoteMods(pub remote_mod::Map);

#[serializable_event]
pub struct ExecutedProviderCommand;

#[serializable_event]
pub struct GameRemoved(pub String);

#[serializable_event]
pub struct ErrorRaised(pub String);

pub fn collect_events() -> (
	tauri_specta::EventCollection,
	std::vec::Vec<tauri_specta::EventDataType>,
	specta::TypeMap,
) {
	tauri_specta::collect_events![
		FoundInstalledGame,
		FoundOwnedGame,
		FoundRemoteGame,
		GameAdded,
		SyncRemoteGames,
		SyncModLoaders,
		SyncLocalMods,
		SyncRemoteMods,
		ExecutedProviderCommand,
		GameRemoved,
		ErrorRaised,
	]
}
