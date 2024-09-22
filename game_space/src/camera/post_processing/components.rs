use bevy::prelude::Component;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_graph::RenderLabel;
use bevy::render::render_resource::ShaderType;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct PostProcessLabel;

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
    pub intensity: f32,
    pub distortion: f32,
}
