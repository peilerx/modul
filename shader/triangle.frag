#version 450
// Fallback solid color — no push constants.

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(0.2, 0.55, 0.85, 1.0);
}
