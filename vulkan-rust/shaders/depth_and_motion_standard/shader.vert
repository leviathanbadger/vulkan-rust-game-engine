#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 proj;
    mat4 previous_proj;
    vec3 ambient_light;
    vec3 directional_light_direction;
    vec3 directional_light_color;
    uint frame_index;
    float time_in_seconds;
} ubo;

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
    mat4 previous_viewmodel;
} pcs;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inNormal;
layout(location = 2) in vec3 inTangent;
layout(location = 3) in vec3 inColor;
layout(location = 4) in vec2 inUv;

layout(location = 0) out vec4 currentFragPositionClipSpace;
layout(location = 1) out vec4 previousFragPositionClipSpace;

void main() {
    gl_Position = ubo.proj * pcs.viewmodel * vec4(inPosition, 1.0);
    currentFragPositionClipSpace = gl_Position;
    previousFragPositionClipSpace = (ubo.previous_proj * pcs.previous_viewmodel * vec4(inPosition, 1.0));
}
