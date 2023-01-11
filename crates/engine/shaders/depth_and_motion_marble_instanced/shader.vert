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

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
    mat4 previous_viewmodel;
} pcs;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inNormal;
layout(location = 2) in vec4 inColor;
layout(location = 3) in mat4 inTransform;

layout(location = 0) out vec4 currentFragPositionClipSpace;
layout(location = 1) out vec4 previousFragPositionClipSpace;

void main() {
    mat4 jitter = mat4(1.0);
    if (ubo.jitter_scale > 0) {
        float deltaWidth = 1.0 / ubo.resolution.x;
        float deltaHeight = 1.0 / ubo.resolution.y;
        jitter[3][0] += ubo.jitter.x * deltaWidth * ubo.jitter_scale;
        jitter[3][1] += ubo.jitter.y * deltaHeight * ubo.jitter_scale;
    }

    gl_Position = jitter * ubo.proj * pcs.viewmodel * inTransform * vec4(inPosition, 1.0);
    currentFragPositionClipSpace = gl_Position;
    previousFragPositionClipSpace = (ubo.previous_proj * pcs.previous_viewmodel * inTransform * vec4(inPosition, 1.0));
}
