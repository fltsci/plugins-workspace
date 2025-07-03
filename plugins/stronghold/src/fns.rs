use std::{path::PathBuf, time::Duration};

use crate::{
    models::{BytesDto, ProcedureDto},
    stronghold::{Error, Result, Stronghold},
};
use iota_stronghold::{procedures::StrongholdProcedure, Client, Location};
use tauri::State;
use zeroize::{Zeroize, Zeroizing};

use crate::{models::PasswordHashFunction, StrongholdCollection};

pub(crate) fn initialize(
    collection: State<'_, StrongholdCollection>,
    hash_function: State<'_, PasswordHashFunction>,
    snapshot_path: PathBuf,
    mut password: String,
) -> Result<()> {
    let hash = (hash_function.0)(&password);
    password.zeroize();
    let stronghold = Stronghold::new(snapshot_path.clone(), hash)?;

    collection
        .0
        .lock()
        .unwrap()
        .insert(snapshot_path, stronghold);

    Ok(())
}

pub(crate) fn destroy(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
) -> Result<()> {
    let mut collection = collection.0.lock().unwrap();
    if let Some(stronghold) = collection.remove(&snapshot_path) {
        if let Err(e) = stronghold.save() {
            collection.insert(snapshot_path, stronghold);
            return Err(e);
        }
    }
    Ok(())
}

pub(crate) fn save(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
) -> Result<()> {
    let collection = collection.0.lock().unwrap();
    if let Some(stronghold) = collection.get(&snapshot_path) {
        stronghold.save()?;
    }
    Ok(())
}

pub(crate) fn create_client(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
) -> Result<()> {
    let stronghold = get_stronghold(collection, snapshot_path)?;
    stronghold.create_client(client)?;
    Ok(())
}

pub(crate) fn load_client(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
) -> Result<()> {
    let stronghold = get_stronghold(collection, snapshot_path)?;
    stronghold.load_client(client)?;
    Ok(())
}

pub(crate) fn get_store_record(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    key: String,
) -> Result<Option<Vec<u8>>> {
    let client = get_client(collection, snapshot_path, client)?;
    client.store().get(key.as_ref()).map_err(Into::into)
}

pub(crate) fn save_store_record(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    key: String,
    value: Vec<u8>,
    lifetime: Option<Duration>,
) -> Result<Option<Vec<u8>>> {
    let client = get_client(collection, snapshot_path, client)?;
    client
        .store()
        .insert(key.as_bytes().to_vec(), value, lifetime)
        .map_err(Into::into)
}

pub(crate) fn remove_store_record(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    key: String,
) -> Result<Option<Vec<u8>>> {
    let client = get_client(collection, snapshot_path, client)?;
    client.store().delete(key.as_ref()).map_err(Into::into)
}

pub(crate) fn save_secret(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    vault: BytesDto,
    record_path: BytesDto,
    secret: Vec<u8>,
) -> Result<()> {
    let client = get_client(collection, snapshot_path, client)?;
    client
        .vault(&vault)
        .write_secret(
            Location::generic(vault, record_path),
            Zeroizing::new(secret),
        )
        .map_err(Into::into)
}

pub(crate) fn remove_secret(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    vault: BytesDto,
    record_path: BytesDto,
) -> Result<()> {
    let client = get_client(collection, snapshot_path, client)?;
    client
        .vault(vault)
        .delete_secret(record_path)
        .map(|_| ())
        .map_err(Into::into)
}

pub(crate) fn execute_procedure(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
    procedure: ProcedureDto,
) -> Result<Vec<u8>> {
    let client = get_client(collection, snapshot_path, client)?;
    client
        .execute_procedure(StrongholdProcedure::from(procedure))
        .map(Into::into)
        .map_err(Into::into)
}

pub(crate) fn get_stronghold(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
) -> Result<iota_stronghold::Stronghold> {
    let collection = collection.0.lock().unwrap();
    if let Some(stronghold) = collection.get(&snapshot_path) {
        Ok(stronghold.inner().clone())
    } else {
        Err(Error::StrongholdNotInitialized)
    }
}

pub(crate) fn get_client(
    collection: State<'_, StrongholdCollection>,
    snapshot_path: PathBuf,
    client: BytesDto,
) -> Result<Client> {
    let collection = collection.0.lock().unwrap();
    if let Some(stronghold) = collection.get(&snapshot_path) {
        stronghold.get_client(client).map_err(Into::into)
    } else {
        Err(Error::StrongholdNotInitialized)
    }
}
