use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    std_types::{
        RBoxError, ROption,
        RResult::{self, RErr, ROk},
        RStr, RVec,
    },
};
use blazyr_extension::{Plugin, Plugin_Ref, REntity};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect,
    #[error("unknown data store error")]
    Unknown,
}

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
        REntity::builder(0, "Firefox").build(),
        REntity::builder(1, "Google Chrome").build(),
        REntity::builder(2, "Vivaldi").build(),
    ]
    .into())
}

#[sabi_extern_fn]
pub fn on_mount() -> RResult<(), RBoxError> {
    ROk(())
}

#[sabi_extern_fn]
pub fn on_dispose() -> RResult<(), RBoxError> {
    RErr(RBoxError::new(DataStoreError::Unknown))
}

#[sabi_extern_fn]
pub fn on_entity_action(id: u64, arg: ROption<RStr>) -> RResult<(), RBoxError> {
    println!("ENTITY ACTION: {}, {:?}", id, arg);
    ROk(())
}
