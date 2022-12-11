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

layout(location = 0) in vec3 fragNormal;
layout(location = 1) in vec4 fragColor;

layout(location = 0) out vec4 outColor;

void main() {
    vec3 light_color = ubo.ambient_light;

    vec3 normal = normalize(fragNormal);
    float directional_amt = max(dot(normal, -ubo.directional_light_direction), 0.0);
    light_color += ubo.directional_light_color * directional_amt;

    //Diagnose lights
    // outColor = vec4(light_color, 1.0);

    //Diagnose normals
    // outColor = vec4((normal.x + 1.0) / 2.0, (normal.y + 1.0) / 2.0, (normal.z + 1.0) / 2.0, 1.0);

    outColor = vec4(fragColor.rgb * light_color, 1.0);
}
