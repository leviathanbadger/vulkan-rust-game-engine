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

layout(binding = 1) uniform sampler2D tex[2];

layout(push_constant) uniform PushConstants {
    mat4 viewmodel;
    mat4 normal_viewmodel;
    mat4 previous_viewmodel;
} pcs;

layout(location = 0) in vec4 currentFragPositionClipSpace;
layout(location = 1) in vec4 previousFragPositionClipSpace;
layout(location = 2) in vec3 fragNormal;
layout(location = 3) in vec3 fragColor;
layout(location = 4) in vec2 fragUv;

layout(location = 0) out vec4 outColor;
layout(location = 1) out vec2 outMotionVector;

void main() {
    vec3 light_color = ubo.ambient_light;

    float directional_amt = max(dot(fragNormal, -ubo.directional_light_direction), 0.0);
    light_color += ubo.directional_light_color * directional_amt;

    //Diagnose lights
    // outColor = vec4(light_color, 1.0);

    //Diagnose normals
    // outColor = vec4((fragNormal.x + 1.0) / 2.0, (fragNormal.y + 1.0) / 2.0, (fragNormal.z + 1.0) / 2.0, 1.0);

    outColor = vec4(fragColor * texture(tex[0], fragUv).rgb * light_color, 1.0);

    vec3 current_ndc = (currentFragPositionClipSpace / currentFragPositionClipSpace.w).xyz;
    vec3 prev_ndc = (previousFragPositionClipSpace / previousFragPositionClipSpace.w).xyz;
    outMotionVector = (current_ndc - prev_ndc).xy;

    //Diagnose motion vector
    // float mag_mult = 50.0;
    // outColor = vec4(max((outMotionVector.r * mag_mult), 0.0), max((outMotionVector.r * -mag_mult), 0.0), max((outMotionVector.g * mag_mult), 0.0), 1.0);
}
