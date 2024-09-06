// #ifdef GL_ES
// #ifdef GL_FRAGMENT_PRECISION_HIGH
// precision highp float;
// #else
// precision mediump float;
// #endif
// #endif

// #ifdef HIGHER_THAN_300
// out vec4 fragColor;
// #define gl_FragColor fragColor
// #endif

in vec4 v_color;
in vec2 v_pos;
in vec2 v_scale;
in vec4 v_border_color;
in vec4 v_border_radius;
in float v_border_width;

float rounded_box_sdf(vec2 to_center, vec2 size, float radius) {
    return length(max(abs(to_center) - size + vec2(radius), vec2(0.0))) - radius;
}

float fDistance(
    vec2 frag_pos, 
    vec2 position,
    vec2 size, 
    float radius
) {
    vec2 inner_half_size = (size - vec2(radius) * 2.0) / 2.0;
    vec2 top_left = position + vec2(radius);
    return rounded_box_sdf(frag_pos - top_left - inner_half_size, inner_half_size, 0.0);
}

float select_border_radius(vec4 radi, vec2 position, vec2 center)
{
    float rx = radi.x;
    float ry = radi.y;
    rx = position.x > center.x ? radi.y : radi.x;
    ry = position.x > center.x ? radi.z : radi.w;
    rx = position.y > center.y ? ry : rx;
    return rx;
}

void main() {
    vec4 mixed_color = v_color;
    vec2 frag_pos = gl_FragCoord.xy;
    
    float border_radius = select_border_radius(
        v_border_radius,
        frag_pos,
        (v_pos + v_scale * 0.5).xy
    );

    if(v_border_width > 0.0) {
        float internal_border = max(border_radius - v_border_width, 0.0);

        float internal_distance = fDistance(
            frag_pos,
            v_pos + vec2(v_border_width),
            v_scale - vec2(v_border_width * 2.0),
            internal_border
        );

        float border_mix = smoothstep(
            max(internal_border - 0.5, 0.0),
            internal_border + 0.5,
            internal_distance
        );

        mixed_color = mix(v_color, v_border_color, vec4(border_mix));
    }

    // todo: shadows
    float dist = fDistance(
        frag_pos,
        v_pos,
        v_scale,
        border_radius
    );

    float radius_alpha = 1.0 - smoothstep(
        max(border_radius - 0.5, 0.0), 
        border_radius + 0.5, 
        dist
    );

    gl_FragColor = vec4(mixed_color.xyz, mixed_color.w * radius_alpha);
}