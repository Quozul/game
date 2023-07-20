use bevy::prelude::{Query, Res};
use bevy_rapier2d::prelude::{KinematicCharacterController, RapierConfiguration, Vect};

pub fn apply_force(controller: &mut KinematicCharacterController, force: Vect) {
    let previous_translation = match controller.translation {
        Some(translation) => translation,
        None => Vect::new(0.0, 0.0),
    };

    controller.translation = Some(previous_translation + force);
}
