use std::{path::PathBuf, sync::Arc, time::Duration};

use crate::{
    models::{BytesDto, PasswordHashFunction, ProcedureDto, StrongholdCollection},
    stronghold::Result,
};

pub fn init<R: Runtime>(app: &AppHandle<R>) -> Result<AppStronghold<R>> {
    Ok(AppStronghold(Arc::new(app.clone())))
}

use tauri::{AppHandle, Manager, Runtime};

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the stronghold APIs.
pub trait StrongholdExt<R: Runtime> {
    fn stronghold(&self) -> &AppStronghold<R>;
}

impl<R: Runtime, T: Manager<R>> StrongholdExt<R> for T {
    fn stronghold(&self) -> &AppStronghold<R> {
        self.state::<AppStronghold<R>>().inner()
    }
}

#[derive(Clone)]
pub struct AppStronghold<R: Runtime>(Arc<AppHandle<R>>);

impl<R: Runtime> AppStronghold<R> {
    fn app_handle(&self) -> &AppHandle<R> {
        &self.0
    }

    pub fn initialize(&self, snapshot_path: PathBuf, password: String) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        let hash_function = self.app_handle().state::<PasswordHashFunction>();
        crate::fns::initialize(collection, hash_function, snapshot_path, password)
    }

    pub fn destroy(&self, snapshot_path: PathBuf) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::destroy(collection, snapshot_path)
    }

    pub fn save(&self, snapshot_path: PathBuf) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::save(collection, snapshot_path)
    }

    pub fn create_client(&self, snapshot_path: PathBuf, client: BytesDto) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::create_client(collection, snapshot_path, client)
    }

    pub fn load_client(&self, snapshot_path: PathBuf, client: BytesDto) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::load_client(collection, snapshot_path, client)
    }

    pub fn get_store_record(
        &self,
        snapshot_path: PathBuf,
        client: BytesDto,
        key: String,
    ) -> Result<Option<Vec<u8>>> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::get_store_record(collection, snapshot_path, client, key)
    }

    pub fn save_store_record(
        &self,
        snapshot_path: PathBuf,
        client: BytesDto,
        key: String,
        value: Vec<u8>,
        lifetime: Option<Duration>,
    ) -> Result<Option<Vec<u8>>> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::save_store_record(collection, snapshot_path, client, key, value, lifetime)
    }

    pub fn remove_store_record(
        &self,
        snapshot_path: PathBuf,
        client: BytesDto,
        key: String,
    ) -> Result<Option<Vec<u8>>> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::remove_store_record(collection, snapshot_path, client, key)
    }

    pub fn save_secret(
        &self,
        snapshot_path: PathBuf,
        client: BytesDto,
        vault: BytesDto,
        record_path: BytesDto,
        secret: Vec<u8>,
    ) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::save_secret(
            collection,
            snapshot_path,
            client,
            vault,
            record_path,
            secret,
        )
    }

    pub fn remove_secret(
        &self,
        snapshot_path: PathBuf,
        client: BytesDto,
        vault: BytesDto,
        record_path: BytesDto,
    ) -> Result<()> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::remove_secret(collection, snapshot_path, client, vault, record_path)
    }

    pub fn execute_procedure(
        &self,
        snapshot_path: PathBuf,
        client: BytesDto,
        procedure: ProcedureDto,
    ) -> Result<Vec<u8>> {
        let collection = self.app_handle().state::<StrongholdCollection>();
        crate::fns::execute_procedure(collection, snapshot_path, client, procedure)
    }
}
