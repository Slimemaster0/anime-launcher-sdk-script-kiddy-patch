use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[cfg(feature = "sandbox")]
use crate::config::schema_blanks::sandbox::Sandbox;

#[cfg(feature = "components")]
use crate::components::wine::Version as WineVersion;

#[cfg(feature = "components")]
use crate::components::dxvk::Version as DxvkVersion;

pub mod launcher;
pub mod game;
pub mod patch;

#[cfg(feature = "components")]
pub mod components;

pub mod prelude {
    pub use super::launcher::prelude::*;
    pub use super::game::prelude::*;

    pub use super::launcher::*;
    pub use super::game::*;
    pub use super::patch::*;

    #[cfg(feature = "components")]
    pub use super::components::*;
}

use prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Schema {
    pub launcher: Launcher,
    pub game: Game,

    #[cfg(feature = "sandbox")]
    pub sandbox: Sandbox,

    #[cfg(feature = "components")]
    pub components: Components,

    pub patch: Patch
}

impl From<&JsonValue> for Schema {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            launcher: match value.get("launcher") {
                Some(value) => Launcher::from(value),
                None => default.launcher
            },

            game: match value.get("game") {
                Some(value) => Game::from(value),
                None => default.game
            },

            #[cfg(feature = "sandbox")]
            sandbox: match value.get("sandbox") {
                Some(value) => Sandbox::from(value),
                None => default.sandbox
            },

            #[cfg(feature = "components")]
            components: match value.get("components") {
                Some(value) => Components::from(value),
                None => default.components
            },

            patch: match value.get("patch") {
                Some(value) => Patch::from(value),
                None => default.patch
            },
        }
    }
}

impl Schema {
    #[cfg(feature = "components")]
    /// Get selected wine version
    pub fn get_selected_wine(&self) -> anyhow::Result<Option<WineVersion>> {
        match &self.game.wine.selected {
            Some(selected) => WineVersion::find_in(&self.components.path, selected),
            None => Ok(None)
        }
    }

    #[cfg(feature = "components")]
    /// Get selected dxvk version
    pub fn get_selected_dxvk(&self) -> anyhow::Result<Option<DxvkVersion>> {
        match wincompatlib::dxvk::Dxvk::get_version(&self.game.wine.prefix)? {
            Some(version) => DxvkVersion::find_in(&self.components.path, version),
            None => Ok(None)
        }
    }

    #[cfg(feature = "components")]
    /// Resolve real wine prefix path using wincompatlib
    /// 
    /// - For general wine build returns `game.wine.prefix`
    /// - For proton-like builds return `game.wine.prefix`/`pfx`
    pub fn get_wine_prefix_path(&self) -> PathBuf {
        if let Ok(Some(wine)) = self.get_selected_wine() {
            let wine = wine
                .to_wine(&self.components.path, Some(&self.game.wine.builds.join(&wine.name)))
                .with_prefix(&self.game.wine.prefix);

            let prefix = match wine {
                WincompatlibWine::Default(wine) => wine.prefix,
                WincompatlibWine::Proton(proton) => proton.proton_prefix
            };

            if let Some(prefix) = prefix {
                return prefix;
            }
        }

        self.game.wine.prefix.clone()
    }
}
