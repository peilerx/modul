#version 450
// Fallback hard-coded triangle — no vertex attributes, no push constants.
// Used by PipelineTriangleHandled (cmd_draw 3 without VB).

void main() {
    // Fullscreen-ish NDC triangle covering the viewport.
    vec2 pos = vec2(
        float((gl_VertexIndex << 1) & 2),
        float(gl_VertexIndex & 2)
    );
    gl_Position = vec4(pos * 2.0 - 1.0, 0.0, 1.0);
}
