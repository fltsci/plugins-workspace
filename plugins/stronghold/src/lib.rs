// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Store secrets and keys using the [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs) encrypted database and secure runtime.

#![doc(
    html_logo_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png",
    html_favicon_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png"
)]

use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(feature = "kdf")]
pub mod kdf;

mod commands;
mod fns;
pub mod models;

pub mod ext;
pub mod stronghold;

pub use ext::StrongholdExt;

use commands::*;
use models::*;

pub struct Builder {
    password_hash_function: PasswordHashFunctionKind,
}

impl Builder {
    pub fn new<F: Fn(&str) -> Vec<u8> + Send + Sync + 'static>(password_hash_function: F) -> Self {
        Self {
            password_hash_function: PasswordHashFunctionKind::Custom(Box::new(
                password_hash_function,
            )),
        }
    }

    /// Initializes [`Self`] with argon2 as password hash function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tauri::Manager;
    /// tauri::Builder::default()
    ///     .setup(|app| {
    ///         let salt_path = app
    ///             .path()
    ///             .app_local_data_dir()
    ///             .expect("could not resolve app local data path")
    ///             .join("salt.txt");
    ///         app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
    ///         Ok(())
    ///     });
    /// ```
    #[cfg(feature = "kdf")]
    pub fn with_argon2(salt_path: &std::path::Path) -> Self {
        Self {
            password_hash_function: PasswordHashFunctionKind::Argon2(salt_path.to_owned()),
        }
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        let password_hash_function = self.password_hash_function;

        let plugin_builder = PluginBuilder::new("stronghold").setup(move |app, _api| {
            app.manage(StrongholdCollection::default());
            app.manage(PasswordHashFunction(match password_hash_function {
                #[cfg(feature = "kdf")]
                PasswordHashFunctionKind::Argon2(path) => {
                    Box::new(move |p| kdf::KeyDerivation::argon2(p, &path))
                }
                PasswordHashFunctionKind::Custom(f) => f,
            }));
            let app_stronghold = ext::init(app)?;
            app.manage(app_stronghold);
            Ok(())
        });

        Builder::invoke_stronghold_handlers_and_build(plugin_builder)
    }

    fn invoke_stronghold_handlers_and_build<R: Runtime>(
        builder: PluginBuilder<R>,
    ) -> TauriPlugin<R> {
        builder
            .invoke_handler(tauri::generate_handler![
                initialize,
                destroy,
                save,
                create_client,
                load_client,
                get_store_record,
                save_store_record,
                remove_store_record,
                save_secret,
                remove_secret,
                execute_procedure,
            ])
            .build()
    }
}
