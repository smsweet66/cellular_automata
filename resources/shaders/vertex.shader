#version 330 core

layout(location = 0) in vec4 position;

uniform mat4 u_proj;
uniform mat4 u_model;

out mat4 v_colorTransform;

void main()
{
    gl_Position = u_proj * u_model * position;
    v_colorTransform = u_proj;
};