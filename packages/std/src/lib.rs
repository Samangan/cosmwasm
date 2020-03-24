// Exposed on all platforms

mod encoding;
mod errors;
mod serde;
mod storage;
mod traits;
mod types;

pub use crate::encoding::Binary;
pub use crate::errors::{
    contract_err, dyn_contract_err, invalid, unauthorized, Error, NotFound, NullPointer, ParseErr,
    Result, SerializeErr,
};
pub use crate::serde::{from_slice, to_vec};
pub use crate::storage::{
    transactional, transactional_deps, MemoryStorage, RepLog, StorageTransaction,
};
pub use crate::traits::{Api, Extern, ReadonlyStorage, Storage};
pub use crate::types::{
    coin, log, CanonicalAddr, ContractResult, CosmosMsg, Env, HumanAddr, QueryResult, Response,
};

// Exposed in wasm build only

#[cfg(target_arch = "wasm32")]
mod exports;
#[cfg(target_arch = "wasm32")]
mod imports;
#[cfg(target_arch = "wasm32")]
mod memory; // used by exports and imports only

#[cfg(target_arch = "wasm32")]
pub use crate::exports::{allocate, deallocate, do_handle, do_init, do_query};
#[cfg(target_arch = "wasm32")]
pub use crate::imports::{ExternalApi, ExternalStorage};

// Exposed for testing only
// Both unit tests and integration tests are compiled to native code, so everything in here does not need to compile to Wasm.

#[cfg(not(target_arch = "wasm32"))]
mod mock;
#[cfg(not(target_arch = "wasm32"))]
pub mod testing {
    pub use crate::mock::{mock_dependencies, mock_env, MockApi, MockStorage};
}

// Not exposed

mod demo;