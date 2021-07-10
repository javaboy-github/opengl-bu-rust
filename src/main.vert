#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 perspective;
// カメラを表した行列
uniform mat4 view;
// 頂点位置を変更するための行列
uniform mat4 model;


void main() {
	mat4 modelview = view * model;
	v_normal = transpose(inverse(mat3(modelview))) * normal;
	gl_Position = perspective * modelview * vec4(position, 1.0);
	v_position = gl_Position.xyz / gl_Position.w;
}
