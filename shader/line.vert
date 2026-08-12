#version 450
// CAD line / grid — pos only + MVP push (CadLinePushRt = 80 bytes).

layout(location = 0) in vec3 inPos;

layout(push_constant) uniform Pc {
    mat4 mvp;
    vec4 color;
} pc;

layout(location = 0) out vec4 vColor;

void main() {
    gl_Position = pc.mvp * vec4(inPos, 1.0);
    vColor = pc.color;
}
