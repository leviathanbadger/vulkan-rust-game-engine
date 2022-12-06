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
    mat4 normal_viewmodel;
} pcs;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inNormal;
layout(location = 2) in vec4 inColor;
layout(location = 3) in mat4 inTransform;

layout(location = 0) out vec3 fragNormal;
layout(location = 1) out vec4 fragColor;

void main() {
    gl_Position = ubo.proj * pcs.viewmodel * inTransform * vec4(inPosition, 1.0);

    fragNormal = (pcs.normal_viewmodel * transpose(inverse(inTransform)) * vec4(inNormal, 1.0)).rgb;
    fragColor = inColor;
}
