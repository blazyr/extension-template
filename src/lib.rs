use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    std_types::{
        ROption::{RNone, RSome},
        RVec,
    },
};
use spotlight_extension::{Plugin, Plugin_Ref, REntity};

#[export_root_module]
fn instantiate_root_module() -> Plugin_Ref {
    Plugin {
        entities,
        on_entity_action,
    }
    .leak_into_prefix() // Converts the `MinMod` into `MinMod_Ref` and leaks it
}

#[sabi_extern_fn]
pub fn entities() -> RVec<REntity> {
    vec![
        REntity {
            id: 0,
            name: RSome("Firefox".into()),
            description: RSome("Browser".into()),
            alias: RNone,
        },
        REntity {
            id: 1,
            name: RSome("Chrome".into()),
            description: RSome("Browser".into()),
            alias: RNone,
        },
        REntity {
            id: 2,
            name: RSome("Vivaldi".into()),
            description: RSome("Browser".into()),
            alias: RNone,
        },
    ]
    .into()
}

#[sabi_extern_fn]
pub fn on_entity_action(id: u64) {
    println!("ENTITY ACTION: {}", id);
}
