use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;
use crate::player::components::FacingComponent;
use crate::player::events::Moving;

pub fn update_system(mut controllers: Query<(&mut KinematicCharacterController, &FacingComponent), With<Moving>>) {
	for (mut controller, facing) in controllers.iter_mut() {
		let vel = facing.0.to_vec() / 5.;
		controller.translation = Some(vel);
	}
}
