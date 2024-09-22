use bevy::prelude::{Component, Timer, TimerMode};
use std::time::Duration;

#[derive(Component, Clone, Default)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    fps: u8,
    pub frame_timer: Timer,
    pub last_sprite_index: usize,
    mode: TimerMode,
    is_animating: bool,
}

impl AnimationConfig {
    /// Creates a new animation that will play in loop
    pub fn repeating(first: usize, last: usize, fps: u8, started: bool) -> Self {
        let mode = TimerMode::Repeating;
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            mode,
            frame_timer: Self::timer_from_fps(fps, mode),
            is_animating: started,
        }
    }

    /// Creates a new animation that needs to be triggered in order to play once
    pub fn once(first: usize, last: usize, fps: u8) -> Self {
        let mode = TimerMode::Once;
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            mode,
            frame_timer: Self::timer_from_fps(fps, mode),
            is_animating: false,
        }
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn start(&mut self) {
        self.is_animating = true;
    }

    pub fn stop(&mut self) {
        self.is_animating = false;
    }

    pub fn reset(&mut self) {
        self.frame_timer = Self::timer_from_fps(self.fps, self.mode);
    }

    fn timer_from_fps(fps: u8, mode: TimerMode) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), mode)
    }
}
