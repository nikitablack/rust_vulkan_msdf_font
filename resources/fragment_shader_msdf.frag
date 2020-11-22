#version 450

layout(set = 0, binding = 2) uniform sampler2D fontTexture;

layout(location = 0) in vec2 inUv;
layout(location = 0) out vec4 outColor;

float median(float r, float g, float b) {
    return max(min(r, g), min(max(r, g), b));
}

void main() {
    float pxRange = 10.0f;
    vec4 bgColor = vec4(0.5f, 0.0f, 0.0f, 0.0f);
    vec4 fgColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);

    vec2 msdfUnit = pxRange / vec2(textureSize(fontTexture, 0));
    vec3 smpl = texture(fontTexture, inUv.st).rgb;
    float sigDist = median(smpl.r, smpl.g, smpl.b) - 0.5;
    sigDist *= dot(msdfUnit, 0.5 / fwidth(inUv));
    float opacity = clamp(sigDist + 0.5, 0.0, 1.0);
    outColor = mix(bgColor, fgColor, opacity);
}