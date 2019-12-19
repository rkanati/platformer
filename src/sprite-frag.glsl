
#version 450

layout(binding = 0) uniform sampler2DArray tex;

smooth in vec3 tcoords;

out vec4 frag;

void main() {
//  frag = vec4(1.0, 1.0, 1.0, 1.0);
    frag = texture(tex, tcoords);
}

