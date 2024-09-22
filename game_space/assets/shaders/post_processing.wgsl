#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
    distortion: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5); // center of the screen
    let distortion_intensity = settings.distortion;
    let uv = in.uv;

    // compute distance from the center and apply lens distortion effect
    let offset = (uv - center) * distortion_intensity;
    let r = length(offset);
    let distorted_uv = uv + clamp(offset * (r * r), vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, 1.0));

    if (distorted_uv.x >= 0.0 && distorted_uv.x <= 1.0 && distorted_uv.y >= 0.0 && distorted_uv.y <= 1.0) {
        // Chromatic aberration strength
        let offset_strength = settings.intensity * length(uv - center);
        return vec4<f32>(
            textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(offset_strength, -offset_strength)).r,
            textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(-offset_strength, 0.0)).g,
            textureSample(screen_texture, texture_sampler, distorted_uv + vec2<f32>(0.0, offset_strength)).b,
            1.0
        );
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}
