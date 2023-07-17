use bevy::prelude::{Query, Res};
use bevy_rapier2d::prelude::{KinematicCharacterController, RapierConfiguration, Vect};

pub fn gravity(
    rapier_configuration: Res<RapierConfiguration>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    for mut controller in &mut query {
        apply_force(&mut controller, rapier_configuration.gravity);
    }
}

pub fn apply_force(controller: &mut KinematicCharacterController, force: Vect) {
    let previous_translation = match controller.translation {
        Some(translation) => translation,
        None => Vect::new(0.0, 0.0),
    };

    controller.translation = Some(previous_translation + force);
}
