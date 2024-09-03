#ifdef GL_ES
#ifdef GL_FRAGMENT_PRECISION_HIGH
precision highp float;
#else
precision mediump float;
#endif
#endif

#ifdef HIGHER_THAN_300
out vec4 fragColor;
#define gl_FragColor fragColor
#endif

uniform float u_ScreenHeight;

in vec4 v_Color;
in vec4 v_BorderColor;
in vec2 v_Pos;
in vec2 v_Scale;
in vec4 v_BorderRadius;
in float v_BorderWidth;

float roundedBoxSdf(vec2 to_center, vec2 size, float radius) {
  return length(max(abs(to_center) - size + vec2(radius, radius), vec2(0.0, 0.0))) - radius;
}

float fDistance(
    vec2 frag_coord, 
    vec2 position,
    vec2 size, 
    float radius
) {
    vec2 inner_half_size = (size - vec2(radius, radius) * 2.0) / 2.0;
    vec2 top_left = position + vec2(radius, radius);
    return roundedBoxSdf(frag_coord - top_left - inner_half_size, inner_half_size, 0.0);
}

float selectBorderRadius(vec4 radi, vec2 position, vec2 center)
{
    float rx = radi.x;
    float ry = radi.y;
    rx = position.x > center.x ? radi.y : radi.x;
    ry = position.x > center.x ? radi.z : radi.w;
    rx = position.y > center.y ? ry : rx;
    return rx;
}

void main() {
    vec4 mixed_color = v_Color;

    vec2 fragCoord = vec2(gl_FragCoord.x, u_ScreenHeight - gl_FragCoord.y);

    float border_radius = selectBorderRadius(
        v_BorderRadius,
        fragCoord,
        (v_Pos + v_Scale * 0.5).xy
    );

    if(v_BorderWidth > 0.0) {
        float internal_border = max(border_radius - v_BorderWidth, 0.0);

        float internal_distance = fDistance(
            fragCoord,
            v_Pos + vec2(v_BorderWidth),
            v_Scale - vec2(v_BorderWidth * 2.0),
            internal_border
        );

        float border_mix = smoothstep(
            max(internal_border - 0.5, 0.0),
            internal_border + 0.5,
            internal_distance
        );

        mixed_color = mix(v_Color, v_BorderColor, border_mix);
    }

    float dist = fDistance(
        fragCoord,
        v_Pos,
        v_Scale,
        border_radius
    );

    float radius_alpha = 1.0 - smoothstep(
        max(border_radius - 0.5, 0.0),
        border_radius + 0.5, 
        dist
    );

    vec4 quad_color = vec4(mixed_color.xyz, mixed_color.w * radius_alpha);

    // todo: shadows

    gl_FragColor = quad_color;
}