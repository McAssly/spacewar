#version 330

in vec2 fragTexCoord;
out vec4 fragColor;

uniform sampler2D _texture;
uniform vec4 colDiffuse;

out vec4 outColor;

const float PI = 3.14;
const vec4 OVERLAY = vec4(0.9, 1.0, 0.7, 1);

uniform float time;

void main() {
    vec2 uv = 0.5 + (fragTexCoord - 0.5)*0.7;

    vec4 color = texture(_texture, fragTexCoord);
    vec4 prime = vec4(color * color * 1.2);

    color = clamp(prime, 0.1, 1.0);
    color *= 0.5 + 0.5 * 18.0 * uv.x * uv.y * (1.0 - uv.x)*(1.0 - uv.y);
    color *= OVERLAY;
    color *= 0.9 + 0.1*sin(5.0*time + uv.y*1000.0);
    color *= 0.9;

    fragColor = color;
}
