use std::rc::Rc;

use anyhow::Result;
use spotlight_extension::{dispose, entities, entry_point, Entity, EntityType};

#[entry_point]
fn init() -> Result<()> {
    println!("Salut mon pote !");
    Ok(())
}

#[entities]
fn data() -> Result<Vec<Entity>> {
    let entity = Entity {
        name: "test".to_string(),
        description: Some("desc".to_string()),
        alias: Some("alias".to_string()),
        command: Rc::new(Box::new(|| Ok("IT WORKS !".to_string()))),
        r#type: EntityType::Command,
    };

    Ok(vec![entity])
}

#[dispose]
fn dispose() -> Result<()> {
    println!("A+ mon pote !");
    Ok(())
}
