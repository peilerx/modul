#version 450

// mesh solid · unit cube instances + Viewsor LOD in instance.w
// lod 0 = full 6 faces · 1 = front-facing faces only · 2 = point impostor

layout(location = 0) in vec3 inPos;
layout(location = 1) in vec3 inNormal;
layout(location = 2) in vec4 inInstancePos; // xyz center · w lod

layout(push_constant) uniform Pc {
    mat4 mvp;
    vec4 light_dir;
    vec4 base_color;
    vec4 cam_pos;
    vec4 look;
    vec4 look2;
    vec4 look3;
} pc;

layout(location = 0) out vec3 vNormal;
layout(location = 1) out vec3 vWorldPos;
layout(location = 2) out vec3 vView;

float saturate(float x) { return clamp(x, 0.0, 1.0); }
float smooth01(float x) {
    x = saturate(x);
    return x * x * (3.0 - 2.0 * x);
}

void main() {
    vec3 n = normalize(inNormal);
    vec3 base_inst = inInstancePos.xyz;
    float lod = inInstancePos.w;

    float t = pc.look3.x;
    float sep_max = pc.look3.y;
    if (sep_max < 1e-5) sep_max = 1.6;
    float y_half = max(pc.look3.z, 0.5);
    float period = max(pc.look3.w, 0.5);

    float y_n = saturate(0.5 + 0.5 * (base_inst.y / y_half));
    float g = fract(t / period);
    float open_g;
    if (g < 0.40) open_g = g / 0.40;
    else if (g < 0.50) open_g = 1.0;
    else if (g < 0.90) open_g = 1.0 - (g - 0.50) / 0.40;
    else open_g = 0.0;

    float stagger = 0.45;
    float local = open_g * (1.0 + stagger) - (1.0 - y_n) * stagger;
    float open = smooth01(local);
    vec3 inst = base_inst * (1.0 + open * sep_max);

    vec3 eye = pc.cam_pos.xyz;
    vec3 to_cam = normalize(eye - inst);
    vec3 world;
    vec3 out_n = n;

    if (lod >= 1.5) {
        vec3 right = normalize(cross(vec3(0.0, 1.0, 0.0), to_cam));
        if (length(right) < 1e-3) right = vec3(1.0, 0.0, 0.0);
        vec3 up = normalize(cross(to_cam, right));
        float s = 0.08;
        world = inst + right * (inPos.x * s) + up * (inPos.y * s);
        out_n = to_cam;
    } else if (lod >= 0.5) {
        float facing = dot(n, to_cam);
        if (facing < 0.05) {
            world = eye - to_cam * 10.0;
            out_n = n;
        } else {
            world = inPos + inst;
            out_n = n;
        }
    } else {
        world = inPos + inst;
        out_n = n;
    }

    gl_Position = pc.mvp * vec4(world, 1.0);
    vNormal = out_n;
    vWorldPos = world;
    vView = normalize(eye - world);
}
