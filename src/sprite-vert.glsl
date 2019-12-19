
#version 450

layout(location = 1) uniform vec2 scale;
layout(location = 2) uniform vec2 offset;

layout(location = 0) in vec4 attr_rect;
layout(location = 1) in uint attr_tex_index;

smooth out vec3 tcoords;

void main() {
    vec2[4] coord_tab = vec2[4] (
        vec2(attr_rect[0], attr_rect[1]),
        vec2(attr_rect[0], attr_rect[3]),
        vec2(attr_rect[2], attr_rect[3]),
        vec2(attr_rect[2], attr_rect[1])
    );

    vec2[4] tcoord_tab = vec2[4] (
        vec2(0.0, 1.0),
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0)
    );

    vec2 coords = scale * (coord_tab[gl_VertexID] + offset);
    gl_Position = vec4(coords, -0.5, 1.0);
    tcoords = vec3(tcoord_tab[gl_VertexID], float(attr_tex_index));
}

