#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 proj;
    mat4 previous_proj;
    vec3 ambient_light;
    vec3 directional_light_direction;
    vec3 directional_light_color;
    vec2 resolution;
    vec2 jitter;
    float jitter_scale;
    uint frame_index;
    float time_in_seconds;
} ubo;

layout(binding = 1) uniform sampler2D tex[2];

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
    mat4 previous_viewmodel;
} pcs;

layout(location = 0) in vec4 currentFragPositionClipSpace;
layout(location = 1) in vec4 previousFragPositionClipSpace;

layout(location = 0) out vec2 outMotionVector;

void main() {
    vec3 current_ndc = (currentFragPositionClipSpace / currentFragPositionClipSpace.w).xyz;
    vec3 prev_ndc = (previousFragPositionClipSpace / previousFragPositionClipSpace.w).xyz;
    outMotionVector = (current_ndc - prev_ndc).xy;
}
