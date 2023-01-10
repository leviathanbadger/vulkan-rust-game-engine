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
    mat4 normal_viewmodel;
} pcs;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inNormal;
layout(location = 2) in vec3 inTangent;
layout(location = 3) in vec3 inColor;
layout(location = 4) in vec2 inUv;

layout(location = 0) out vec4 currentFragPositionCameraSpace;
layout(location = 1) out vec3 fragNormal;
layout(location = 2) out vec3 fragTangent;
layout(location = 3) out vec3 fragColor;
layout(location = 4) out vec2 fragUv;

void main() {
    mat4 jitter = mat4(1.0);
    if (ubo.jitter_scale > 0) {
        float deltaWidth = 1.0 / ubo.resolution.x;
        float deltaHeight = 1.0 / ubo.resolution.y;
        jitter[3][0] += ubo.jitter.x * deltaWidth * ubo.jitter_scale;
        jitter[3][1] += ubo.jitter.y * deltaHeight * ubo.jitter_scale;
    }

    currentFragPositionCameraSpace = pcs.viewmodel * vec4(inPosition, 1.0);
    gl_Position = jitter * ubo.proj * currentFragPositionCameraSpace;

    fragNormal = normalize((pcs.normal_viewmodel * vec4(inNormal, 1.0)).xyz);
    fragTangent = normalize((pcs.normal_viewmodel * vec4(inTangent, 1.0)).xyz);
    fragColor = inColor;
    fragUv = inUv;
}
