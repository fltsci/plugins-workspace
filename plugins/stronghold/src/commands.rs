use std::{path::PathBuf, time::Duration};

use crate::{
    models::{BytesDto, ProcedureDto},
    stronghold::Result,
};
use tauri::State;

use crate::{models::PasswordHashFunction, StrongholdCollection};

#[tauri::command]
pub async fn initialize(
    collection: State<'_, StrongholdCollection>,
    hash_function: State<'_, PasswordHashFunction>,
    snapshot_path: PathBuf,
    password: String,
) -> Result<()> {
    crate::fns::initialize(collection, hash_function, snapshot_path, password)
}

#[tauri::command]
pub async fn destroy(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
) -> Result<()> {
    crate::fns::destroy(collection, snapshot_path)
}

#[tauri::command]
pub async fn save(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
) -> Result<()> {
    crate::fns::save(collection, snapshot_path)
}

#[tauri::command]
pub async fn create_client(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
) -> Result<()> {
    crate::fns::create_client(collection, snapshot_path, client)
}

#[tauri::command]
pub async fn load_client(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
) -> Result<()> {
    crate::fns::load_client(collection, snapshot_path, client)
}

#[tauri::command]
pub async fn get_store_record(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    key: String,
) -> Result<Option<Vec<u8>>> {
    crate::fns::get_store_record(collection, snapshot_path, client, key)
}

#[tauri::command]
pub async fn save_store_record(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    key: String,
    value: Vec<u8>,
    lifetime: Option<Duration>,
) -> Result<Option<Vec<u8>>> {
    crate::fns::save_store_record(collection, snapshot_path, client, key, value, lifetime)
}

#[tauri::command]
pub async fn remove_store_record(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    key: String,
) -> Result<Option<Vec<u8>>> {
    crate::fns::remove_store_record(collection, snapshot_path, client, key)
}

#[tauri::command]
pub async fn save_secret(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    vault: BytesDto,
    record_path: BytesDto,
    secret: Vec<u8>,
) -> Result<()> {
    crate::fns::save_secret(
        collection,
        snapshot_path,
        client,
        vault,
        record_path,
        secret,
    )
}

#[tauri::command]
pub async fn remove_secret(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    vault: BytesDto,
    record_path: BytesDto,
) -> Result<()> {
    crate::fns::remove_secret(collection, snapshot_path, client, vault, record_path)
}

#[tauri::command]
pub async fn execute_procedure(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    procedure: ProcedureDto,
) -> Result<Vec<u8>> {
    crate::fns::execute_procedure(collection, snapshot_path, client, procedure)
}
