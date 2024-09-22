use crate::animation::systems::execute_animations;
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_animations);
    }
}
