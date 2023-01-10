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

layout(location = 0) in vec4 currentFragPositionCameraSpace;
layout(location = 1) in vec3 fragNormal;
layout(location = 2) in vec3 fragColor;

layout(location = 0) out vec4 outColor;

void main() {
    vec3 light_color = ubo.ambient_light;

    vec3 normal = normalize(fragNormal);

    float directional_amt = max(dot(normal, -ubo.directional_light_direction), 0.0);
    light_color += ubo.directional_light_color * directional_amt;

    vec3 mat_specular_color = vec3(7.5, 7.5, 7.5);
    float mat_shininess = 20.0;

    vec3 view_direction = normalize(currentFragPositionCameraSpace.xyz);
    float specular_factor = max(0.0, dot(reflect(-ubo.directional_light_direction, normal), view_direction));
    vec3 specular_color = mat_specular_color * pow(specular_factor, mat_shininess);
    light_color += specular_color;

    //Diagnose lights
    // outColor = vec4(light_color, 1.0);

    //Diagnose normals
    // outColor = vec4((normal.x + 1.0) / 2.0, (normal.y + 1.0) / 2.0, (normal.z + 1.0) / 2.0, 1.0);

    outColor = vec4(fragColor * light_color, 1.0);
}
