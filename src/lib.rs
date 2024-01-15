use std::rc::Rc;

use spotlight_extension::{Entity, EntityType};

#[no_mangle]
fn execute() -> Option<Entity> {
    Some(Entity {
        name: "test".to_string(),
        description: Some("desc".to_string()),
        alias: Some("alias".to_string()),
        command: Rc::new(Box::new(|| Ok("IT WORKS !".to_string()))),
        r#type: EntityType::Command,
    })
}
