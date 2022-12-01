#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 proj;
    uint frame_index;
    float time_in_seconds;
} ubo;

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
} pcs;

layout(location = 0) in vec3 inPosition;
layout(location = 2) in vec3 inColor;

layout(location = 1) out vec3 fragColor;

void main() {
    gl_Position = ubo.proj * pcs.viewmodel * vec4(inPosition, 1.0);
    fragColor = inColor;
}
