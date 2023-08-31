#version 400

in vec3 position; // Vertex position input
in vec3 color; // Vertex color input

out vec3 fragColor; // Fragment color output

void main() {
    gl_Position = vec4(position, 0.5); // Set vertex position
    fragColor = color; // Pass vertex color to fragment shader
}
