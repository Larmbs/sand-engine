#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;

void main() {
    vec3 bottomColor = vec3(0.7647, 0.9608, 0.9686);
    vec3 topColor = vec3(0.4, 0.851, 0.871); // Blue
    float t = uv.y;
    gl_FragColor = vec4(mix(bottomColor, topColor, t), 1.0);
}
