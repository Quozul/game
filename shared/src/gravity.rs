use crate::FIXED_TIMESTEP;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_rapier2d::prelude::{KinematicCharacterController, TimestepMode, Vect};

pub fn apply_force(controller: &mut KinematicCharacterController, force: Vect) {
    let previous_translation = match controller.translation {
        Some(translation) => translation,
        None => Vect::new(0.0, 0.0),
    };

    controller.translation = Some(previous_translation + force);
}

pub fn get_rapier_configuration() -> RapierConfiguration {
    RapierConfiguration {
        gravity: Vect::ZERO,
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::Fixed {
            dt: FIXED_TIMESTEP,
            substeps: 1,
        },
        scaled_shape_subdivision: 10,
        force_update_from_transform_changes: false,
    }
}
