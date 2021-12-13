#version 400 core

layout(location = 0) out vec4 color;
in mat4 v_colorTransform;
uniform vec4 u_color;

void main()
{
    color = v_colorTransform * u_color;
    if(color.x < 0)
        color.x *= -1;
    if(color.y < 0)
        color.y *= -1;
};