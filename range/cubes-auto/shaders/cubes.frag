#version 450

// Cubes solid — multi-lobe GGX + cavity / soft contact (instanced lattice).
// look3: vertex pulse (time, sep, y_half, period) — do not
// read lighting knobs from look3 here.

layout(location = 0) in vec3 vNormal;
layout(location = 1) in vec3 vWorldPos;
layout(location = 2) in vec3 vView;
layout(location = 0) out vec4 outColor;

// 160 bytes: mat4 + 6×vec4 — layout must match vert / MeshPushRt
layout(push_constant) uniform Pc {
    mat4 mvp;
    vec4 light_dir;   // xyz key dir · w key intensity
    vec4 base_color;  // rgb albedo · w roughness
    vec4 cam_pos;     // xyz eye · w exposure
    vec4 look;        // f0, specular, env, fill
    vec4 look2;       // rim, brush, film, contrast
    vec4 look3;       // PULSE only (vert): t, sep_max, y_half, period
} pc;

float saturate(float x) { return clamp(x, 0.0, 1.0); }

vec3 aces(vec3 x) {
    const float a = 2.51, b = 0.03, c = 2.43, d = 0.59, e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

float d_ggx(float ndh, float a) {
    float a2 = a * a;
    float d = (ndh * ndh) * (a2 - 1.0) + 1.0;
    return a2 / (3.14159265 * d * d + 1e-7);
}

float softbox(vec3 R, float cx, float cy, float sx, float sy, float gain) {
    float dx = (R.x - cx) * sx;
    float dy = (R.y - cy) * sy;
    return exp(-(dx * dx + dy * dy)) * gain;
}

void main() {
    vec3 N = normalize(vNormal);
    vec3 V = normalize(vView);
    float NdV = saturate(dot(N, V));
    float one_m = 1.0 - NdV;

    float F0 = clamp(pc.look.x, 0.05, 1.0);
    if (F0 < 0.01) F0 = 0.55;
    float rough = clamp(pc.base_color.w, 0.02, 0.9);
    if (rough < 0.01) rough = 0.14;
    float spec_gain = clamp(pc.look.y, 0.0, 2.5);
    if (spec_gain < 0.01) spec_gain = 1.2;
    float env_i = clamp(pc.look.z, 0.0, 2.5);
    if (env_i < 0.01) env_i = 0.85;
    float key_i = clamp(pc.light_dir.w, 0.0, 2.5);
    if (key_i < 0.01) key_i = 1.15;
    float fill_i = clamp(pc.look.w, 0.0, 2.5);
    if (fill_i < 0.01) fill_i = 0.55;
    float rim_i = clamp(pc.look2.x, 0.0, 2.5);
    if (rim_i < 0.01) rim_i = 0.55;
    float brush_amt = clamp(pc.look2.y, 0.0, 2.5);
    float film_amt = clamp(pc.look2.z, 0.0, 2.5);
    float exposure = clamp(pc.cam_pos.w, 0.5, 2.0);
    if (exposure < 0.1) exposure = 1.08;
    float contrast = clamp(pc.look2.w, 0.5, 2.0);
    if (contrast < 0.1) contrast = 1.12;

    // Cavity / ridge / soft contact — fixed etalon knobs (look3 = pulse in vert)
    float cav_i = 1.0;
    float rid_i = 0.75;
    float sh_i = 1.15;
    float sh_soft = 2.4;

    float F = F0 + (1.0 - F0) * pow(one_m, 5.0);

    vec3 Lkey  = normalize(pc.light_dir.xyz);
    if (length(Lkey) < 0.1) Lkey = normalize(vec3(0.52, 0.82, 0.48));
    vec3 Lfill = normalize(vec3(-0.78, 0.32, 0.28));
    vec3 Lrim  = normalize(vec3(-0.18, 0.12,-0.96));
    vec3 Lsky  = normalize(vec3( 0.08, 0.99, 0.06));
    vec3 Lkick = normalize(vec3( 0.15,-0.55, 0.55));

    vec3 Hk = normalize(Lkey + V);
    vec3 Hf = normalize(Lfill + V);
    vec3 Hs = normalize(Lsky + V);
    vec3 Hkick = normalize(Lkick + V);

    float NdH = saturate(dot(N, Hk));
    float NdL = saturate(dot(N, Lkey));
    float NdLf = saturate(dot(N, Lfill));
    float NdLs = saturate(dot(N, Lsky));

    float r0 = max(rough * 1.8, 0.04);
    float r1 = max(rough * 0.55, 0.02);
    float r2 = max(rough * 0.18, 0.01);
    float r3 = max(rough * 0.06, 0.008);
    float D = d_ggx(NdH, r0) * 0.22
            + d_ggx(NdH, r1) * 0.55
            + d_ggx(NdH, r2) * 0.95
            + d_ggx(NdH, r3) * 0.70;
    float G = NdL / (NdL * 0.55 + 0.45);
    float spec_key = D * G * F * key_i;

    float ndh_f = saturate(dot(N, Hf));
    float spec_fill = (d_ggx(ndh_f, rough * 1.2) * 0.18 * F + pow(ndh_f, 48.0) * 0.12) * fill_i;
    float spec_sky  = d_ggx(saturate(dot(N, Hs)), rough * 2.0) * 0.12 * F * env_i;
    float spec_kick = pow(saturate(dot(N, Hkick)), 24.0) * 0.08 * key_i;

    vec3 up = vec3(0.0, 1.0, 0.0);
    vec3 t = cross(N, up);
    float tl = length(t);
    t = (tl > 1e-4) ? t / tl : vec3(1.0, 0.0, 0.0);
    vec3 b = cross(N, t);
    float ht = abs(dot(Hk, t));
    float hb = abs(dot(Hk, b));
    float aniso = exp(-((ht * ht) * 6.5 + (hb * hb) * 0.9)) * 1.35
                + (1.0 - ht * ht) * 0.25;
    aniso = clamp(aniso, 0.45, 1.85);
    aniso = mix(1.0, aniso, clamp(brush_amt, 0.0, 1.5) / 1.5);

    float line = sin(dot(vWorldPos.xz, t.xz) * 28.0 + vWorldPos.y * 12.0) * 0.5 + 0.5;
    float brush = mix(0.90, 1.10, line * line * clamp(brush_amt, 0.0, 1.5));

    vec3 R = reflect(-V, N);
    float elev = saturate(R.y * 0.5 + 0.5);
    vec3 zenith  = vec3(0.10, 0.16, 0.30);
    vec3 horizon = vec3(0.78, 0.84, 0.98);
    vec3 ground  = vec3(0.07, 0.065, 0.055);
    vec3 env = mix(zenith, horizon, pow(elev, 0.62));
    float floorW = saturate((-R.y + 0.06) * 3.4);
    env = mix(env, ground, floorW);
    float sb =
          softbox(R,  0.18,  0.42, 5.2, 2.4, 1.35)
        + softbox(R, -0.55,  0.20, 3.6, 2.8, 0.55)
        + softbox(R,  0.65,  0.05, 4.0, 3.2, 0.40)
        + softbox(R, -0.10,  0.75, 2.8, 1.6, 0.70)
        + softbox(R,  0.90, -0.15, 2.2, 4.0, 0.28);
    env += vec3(1.05, 1.12, 1.32) * sb;
    env += vec3(1.25, 0.85, 0.45) * softbox(R, -0.85, -0.05, 2.6, 3.5, 0.32);
    env *= saturate(NdV * 1.6 + 0.15) * env_i;

    vec3 albedo = vec3(0.70, 0.725, 0.765);
    if (length(pc.base_color.rgb) > 0.01) {
        albedo = pc.base_color.rgb;
    }
    float hemi = 0.09 + 0.14 * saturate(N.y * 0.5 + 0.5);
    // Stronger key diffuse so faces read as lit vs shaded
    vec3 metal = albedo * (hemi + NdL * 0.42 * key_i + NdLf * 0.14 * fill_i + NdLs * 0.10 * env_i);

    float rim = pow(one_m, 2.2) * (0.28 + 0.72 * abs(dot(N, Lrim))) * rim_i;
    float coat = pow(one_m, 3.8) * 0.65 * spec_gain;
    float edge_ring = pow(one_m, 6.0) * 1.1 * rim_i;

    float film = pow(one_m, 2.8) * clamp(film_amt, 0.0, 1.5);
    float phase = film * 9.5 + NdV * 2.0;
    vec3 iri = vec3(
        0.5 + 0.5 * sin(phase),
        0.5 + 0.5 * sin(phase + 2.094),
        0.5 + 0.5 * sin(phase + 4.189)
    );
    iri = mix(vec3(1.0), iri, 0.22 * film);

    vec3 spec_tint = vec3(0.92, 0.98, 1.14);
    float total_spec = (spec_key * aniso + spec_fill + spec_sky + spec_kick) * brush * spec_gain;
    vec3 col = metal
        + spec_tint * total_spec * 0.85
        + env * F * 0.85
        + vec3(0.50, 0.62, 0.98) * rim * 0.48
        + vec3(0.88, 0.94, 1.08) * coat * 0.38
        + vec3(0.75, 0.85, 1.15) * edge_ring * 0.55;
    col *= mix(vec3(1.0), iri, 0.35 * film);

    float lum = dot(col, vec3(0.2126, 0.7152, 0.0722));
    float cool = pow(1.0 - saturate(lum * 1.55), 1.45) * 0.07;
    col.r -= cool * 0.45;
    col.g -= cool * 0.08;
    col.b += cool * 1.15;
    float peak = smoothstep(0.55, 1.4, lum);
    col += vec3(0.06, 0.035, 0.01) * peak;

    // Cavity / ridge from normal derivatives (screen-space)
    float n_grad = length(dFdx(N)) + length(dFdy(N));
    float cav = saturate(n_grad * 6.0) * cav_i;
    float rid = saturate((0.35 - n_grad) * 4.0) * rid_i;
    col *= (1.0 - cav * 0.55);
    col += albedo * rid * 0.12;
    // Soft contact darkening in creases / down-facing
    float contact = saturate((-N.y * 0.5 + 0.5) * (0.4 + n_grad * sh_soft)) * sh_i;
    col *= (1.0 - contact * 0.45);
    // Directional soft shadow wrap (back faces darker)
    float wrap = saturate((NdL + 0.25) / 1.25);
    col *= mix(0.55, 1.0, wrap);

    col = (col - 0.5) * contrast + 0.5 + 0.018;
    col = aces(max(col, 0.0) * exposure);
    col = pow(clamp(col, 0.0, 1.0), vec3(0.96));

    outColor = vec4(col, 1.0);
}
