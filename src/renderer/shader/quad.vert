uniform mat4 u_Transform;
uniform float u_Scale;

in vec4 i_color;
in vec2 i_pos;
in vec2 i_size;

in vec2 i_inst_pos;

out vec4 v_Color;
out vec2 v_Pos;
out vec2 v_Scale;

void main() {
    vec2 q_Pos = i_inst_pos;
    vec2 p_Pos = i_pos * u_Scale;
    vec2 p_Scale = i_size * u_Scale;

    mat4 i_Transform = mat4(
        vec4(p_Scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4(0.0, p_Scale.y + 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(p_Pos + (vec2(p_Scale.x * 0.5, p_Scale.y * 0.5)), 0.0, 1.0)
    );

    v_Color = i_color;
    v_Pos = i_pos * u_Scale;
    v_Scale = p_Scale;

    // projection * model (position)
    gl_Position = u_Transform * i_Transform * vec4(q_Pos, 0.0, 1.0);
}