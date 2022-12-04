#version 450

layout(binding = 0) uniform sampler2D texColor;
layout(binding = 1) uniform sampler2D texMotion;

layout(location = 0) in vec2 fragUv;

layout(location = 0) out vec4 outColor;

void main() {
    vec2 motion = texture(texMotion, fragUv).rg * -0.5;
    // outColor = vec4(
    //     max(motion.x * 5.0, 0.0),
    //     max(-motion.x * 5.0, 0.0),
    //     max(motion.y * 5.0, 0.0),
    //     1.0
    // );

    vec3 samples = (texture(texColor, fragUv).rgb * 1.0) +
        (texture(texColor, fragUv - motion * 0.125).rgb * 0.8) +
        (texture(texColor, fragUv - motion * 0.25).rgb * 0.6) +
        (texture(texColor, fragUv - motion * 0.375).rgb * 0.5) +
        (texture(texColor, fragUv - motion * 0.5).rgb * 0.4) +
        (texture(texColor, fragUv - motion * 0.625).rgb * 0.3) +
        (texture(texColor, fragUv - motion * 0.75).rgb * 0.225) +
        (texture(texColor, fragUv - motion * 0.875).rgb * 0.15) +
        (texture(texColor, fragUv - motion * 1.0).rgb * 0.075);

    outColor = vec4(samples / 4.0, 1.0);

    //No motion blur
    // outColor = vec4(texture(texColor, fragUv).rgb, 1.0);
}
