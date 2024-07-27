use bevy::prelude::{Commands, Entity};

pub trait CommandsExt<T> {
    fn try_despawn(&mut self, id: T);
}

impl CommandsExt<Entity> for Commands<'_,'_> {
    fn try_despawn(&mut self, id: Entity) {
        if let Some(mut c) = self.get_entity(id) {
            c.despawn();
        }
    }
}

impl CommandsExt<Option<Entity>> for Commands<'_,'_> {
    fn try_despawn(&mut self, id: Option<Entity>) {
        if let Some(entity) = id {
            self.try_despawn(entity);
        };
    }
}

impl<Err> CommandsExt<Result<Entity,Err>> for Commands<'_,'_> {
    fn try_despawn(&mut self, id: Result<Entity, Err>) {
        if let Ok(entity) = id {
            self.try_despawn(entity);
        };
    }
}
