use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DynamicsSchedule;
