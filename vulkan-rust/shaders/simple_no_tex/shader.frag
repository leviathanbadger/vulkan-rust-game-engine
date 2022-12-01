#version 450

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
} pcs;

layout(location = 1) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(fragColor, 1.0);
}
