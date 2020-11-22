/*#version 450

layout(set = 0, binding = 2) uniform sampler2D fontTexture;

layout(location = 0) in vec2 inUv;
layout(location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(texture(fontTexture, inUv.st).r);
}*/

#version 450

layout (binding = 2) uniform sampler2D fontTexture;

layout (location = 0) in vec2 inUv;
layout (location = 0) out vec4 outColor;

void main() 
{
    float distance = texture(fontTexture, inUv).r;
    float smoothWidth = fwidth(distance);
    float alpha = smoothstep(0.5 - smoothWidth, 0.5 + smoothWidth, distance);
    vec3 rgb = vec3(alpha);

    vec4 outlineColor = vec4(0.0, 1.0, 0.0, 1.0);
    float outlineWidth = 0.7;
    float outline = 0.0;

    if (outline > 0.0) 
    {
        float w = 1.0 - outlineWidth;
        alpha = smoothstep(w - smoothWidth, w + smoothWidth, distance);
        rgb += mix(vec3(alpha), outlineColor.rgb, alpha);
    }

    outColor = vec4(rgb, alpha);
}