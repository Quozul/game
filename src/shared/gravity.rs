use bevy::prelude::{Query, Res};
use bevy_rapier2d::prelude::{KinematicCharacterController, RapierConfiguration};

pub fn gravity(
    rapier_configuration: Res<RapierConfiguration>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    for mut controller in &mut query {
        controller.translation = Some(rapier_configuration.gravity);
    }
}
