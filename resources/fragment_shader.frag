#version 450

layout(set = 0, binding = 2) uniform sampler2D fontTexture;

layout(location = 0) in vec2 inUv;
layout(location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(texture(fontTexture, inUv.st).r);
}
