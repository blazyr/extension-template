use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    std_types::{
        RBoxError,
        ROption::{RNone, RSome},
        RResult::{self, RErr, ROk},
        RVec,
    },
};
use spotlight_extension::{Plugin, Plugin_Ref, REntity};

#[export_root_module]
fn instantiate_root_module() -> Plugin_Ref {
    Plugin {
        entities,
        on_entity_action,
        on_mount,
        on_dispose,
    }
    .leak_into_prefix() // Converts the `MinMod` into `MinMod_Ref` and leaks it
}

#[sabi_extern_fn]
pub fn entities() -> RResult<RVec<REntity>, RBoxError> {
    ROk(vec![
        REntity {
            id: 0,
            name: "Firefox".into(),
            description: RSome("Browser".into()),
            alias: RNone,
        },
        REntity {
            id: 1,
            name: "Chrome".into(),
            description: RSome("Browser".into()),
            alias: RNone,
        },
        REntity {
            id: 2,
            name: "Vivaldi".into(),
            description: RSome("Browser".into()),
            alias: RNone,
        },
    ]
    .into())
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect,
    #[error("unknown data store error")]
    Unknown,
}

#[sabi_extern_fn]
pub fn on_mount() -> RResult<(), RBoxError> {
    RErr(RBoxError::new(DataStoreError::Disconnect))
}

#[sabi_extern_fn]
pub fn on_dispose() -> RResult<(), RBoxError> {
    RErr(RBoxError::new(DataStoreError::Unknown))
}

#[sabi_extern_fn]
pub fn on_entity_action(id: u64) -> RResult<(), RBoxError> {
    println!("ENTITY ACTION: {}", id);
    ROk(())
}
