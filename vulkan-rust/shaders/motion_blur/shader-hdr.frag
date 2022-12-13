#version 450

layout(binding = 0) uniform UniformBufferObject {
    uint frame_index;
    float time_in_seconds;
    float exposure;
} ubo;

layout(binding = 1) uniform sampler2D texColor;
layout(binding = 2) uniform sampler2D texMotion;

layout(location = 0) in vec2 fragUv;

layout(location = 0) out vec4 outColor;

vec3 aces_approx(vec3 v) {
    v *= 0.6f;
    float a = 2.51f;
    float b = 0.03f;
    float c = 2.43f;
    float d = 0.59f;
    float e = 0.14f;
    return clamp((v*(a*v+b))/(v*(c*v+d)+e), 0.0f, 1.0f);
}

void main() {
    vec2 motion = texture(texMotion, fragUv).rg * -0.5;

    vec3 samples = (texture(texColor, fragUv).rgb * 1.0) +
        (texture(texColor, fragUv - motion * 0.125).rgb * 0.8) +
        (texture(texColor, fragUv - motion * 0.25).rgb * 0.6) +
        (texture(texColor, fragUv - motion * 0.375).rgb * 0.5) +
        (texture(texColor, fragUv - motion * 0.5).rgb * 0.4) +
        (texture(texColor, fragUv - motion * 0.625).rgb * 0.3) +
        (texture(texColor, fragUv - motion * 0.75).rgb * 0.225) +
        (texture(texColor, fragUv - motion * 0.875).rgb * 0.15) +
        (texture(texColor, fragUv - motion * 1.0).rgb * 0.075);
    samples /= 4.0;

    //No motion blur
    //vec3 samples = texture(texColor, fragUv).rgb;

    samples = vec3(1.0) - exp(-samples * ubo.exposure);
    samples = aces_approx(samples);
    outColor = vec4(clamp(samples, 0.0, 1.0), 1.0);
}
