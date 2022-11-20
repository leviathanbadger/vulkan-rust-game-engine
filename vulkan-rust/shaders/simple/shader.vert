#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 proj;
    mat4 view;
    uint frame_index;
    float time_in_seconds;
} ubo;

// layout(push_constant) uniform PushConstants {
//     mat4 model;
// } pcs;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = ubo.proj * ubo.view * /*pcs.model * */vec4(inPosition, 1.0);
    fragColor = inColor;
}
