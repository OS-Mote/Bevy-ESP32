use bevy::prelude::*;

#[derive(Resource)]
pub struct NavigationStack {
    entities: Vec<Entity>
}

impl NavigationStack {
    pub fn new() -> Self {
        NavigationStack { entities: Vec::new() }
    }

    pub fn push(&mut self, entity: Entity) {
        if let Some(last_entity) = self.entities.last() {
            if *last_entity == entity {
                return
            }
        }

        self.entities.push(entity);
    }

    pub fn pop(&mut self) {
        self.entities.pop();
    }

    pub fn last(&self) -> Option<&Entity> {
        self.entities.last()
    }
}