
#version 450

layout(location = 1) uniform vec2 scale;
layout(location = 2) uniform vec2 offset;

layout(location = 0) in vec2 attr_vert;
layout(location = 1) in vec4 attr_colour;

smooth out vec4 colour;

void main() {
    vec2 coords = scale * (attr_vert + offset);
    gl_Position = vec4(coords, -0.5, 1.0);
    colour = attr_colour;
}

