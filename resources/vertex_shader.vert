#version 450

layout(set = 0, binding = 0) readonly buffer PositionBuffer
{
    float data[];
} positionBuffer;

layout(set = 0, binding = 1) readonly buffer UvBuffer
{
    float data[];
} uvBuffer;

layout(push_constant) uniform PushConst
{
    mat4 proj;
} pushConst;

layout(location = 0) out vec2 outUv;

void main()
{
    uint ind = gl_VertexIndex * 2;
    gl_Position = pushConst.proj * vec4(positionBuffer.data[ind], positionBuffer.data[ind + 1], 0.0f, 1.0f);

    outUv = vec2(uvBuffer.data[ind], uvBuffer.data[ind + 1]);
}