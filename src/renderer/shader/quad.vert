uniform mat4 u_transform;
uniform float u_scale;

in vec2 i_inst_pos;


in vec4 i_color;
in vec2 i_pos;
in vec2 i_size;
in vec4 i_border_color;
in vec4 i_border_radius;
in float i_border_width;


out vec4 v_color;
out vec2 v_pos;
out vec2 v_scale;
out vec4 v_border_color;
out vec4 v_border_radius;
out float v_border_width;


void main() {
    vec2 q_Pos = i_inst_pos;
    vec2 p_Pos = i_pos * u_scale;
    vec2 p_Scale = i_size * u_scale;

    // vec2 snap = vec2(0.0, 0.0);

    // if (i_size.x == 1.0) {
    //     snap.x = round(i_pos.x) - p_Pos.x;
    // };

    // if (i_size.y == 1.0) {
    //     snap.y = round(i_pos.y) - p_Pos.y;
    // };

    float min_border_radius = min(i_size.x, i_size.y) * 0.5;
    vec4 border_radius = vec4(
        min(i_border_radius.x, min_border_radius ),
        min(i_border_radius.y, min_border_radius ),
        min(i_border_radius.z, min_border_radius ),
        min(i_border_radius.w, min_border_radius )
    );


    v_color = i_color;
    v_pos = i_pos * u_scale;
    v_scale = i_size * u_scale;
    v_border_color = i_border_color;
    v_border_radius = border_radius * u_scale;
    v_border_width = i_border_width * u_scale;


    mat4 i_Transform = mat4(
        vec4(p_Scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4(0.0, p_Scale.y + 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(p_Pos - vec2(0.5), 0.0, 1.0)
    );

    gl_Position = u_transform * i_Transform * vec4(q_Pos, 0.0, 1.0);
}