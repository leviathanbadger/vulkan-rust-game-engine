#version 450

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
} pcs;

layout(binding = 1) uniform sampler2D tex;

layout(location = 0) in vec2 fragUv;
layout(location = 1) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(fragColor * texture(tex, fragUv).rgb, 1.0);
}
