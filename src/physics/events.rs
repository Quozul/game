use crate::physics::collisions::collision::Collision;
use bevy::ecs::query::{QueryData, QueryFilter, ROQueryItem};
use bevy::prelude::*;

#[derive(Event)]
pub struct CollisionEvent {
    pub first: Entity,
    pub second: Entity,
    pub collision: Collision,
}

impl CollisionEvent {
    pub fn get_from_query<'a, D: QueryData, F: QueryFilter>(
        &self,
        query: &'a Query<D, F>,
    ) -> Option<ROQueryItem<'a, D>> {
        query
            .get(self.first)
            .or_else(|_| query.get(self.second))
            .ok()
    }

    pub fn get_entity<D: QueryData, F: QueryFilter>(&self, query: &Query<D, F>) -> Option<Entity> {
        if query.contains(self.first) {
            Some(self.first)
        } else if query.contains(self.second) {
            Some(self.second)
        } else {
            None
        }
    }

    pub fn get_mut_from_query<'a, D: QueryData, F: QueryFilter>(
        &self,
        query: &'a mut Query<D, F>,
    ) -> Option<D::Item<'a>> {
        if query.contains(self.first) {
            query.get_mut(self.first).ok()
        } else if query.contains(self.second) {
            query.get_mut(self.second).ok()
        } else {
            None
        }
    }
}
