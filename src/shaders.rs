pub const VERTEX_SHADER: &str = "
                #version 140

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                in vec3 color;
                in vec3 normal;
                out vec3 v_position;
                out vec3 v_color;
                out vec3 v_normal;

                void main() {
                    v_position = position;
                    v_color = color;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position, 1.0);
                }
            ";

#[allow(dead_code)]
pub const FRAGMENT_SHADER_FLAT: &str = "
                #version 140

                in vec3 v_color;
                out vec4 f_color;

                void main() {
                    vec3 color = v_color;
                    f_color = vec4(color, 1.0);
                }
            ";

#[allow(dead_code)]
pub const FRAGMENT_SHADER_SHADED: &str = "
                #version 140

                const vec3 light_position = vec3(10.0, 10.0, 10.0);

                in vec3 v_color;
                in vec3 v_normal;
                in vec3 v_position;
                out vec4 f_color;

                void main() {
                    vec3 light_dir = normalize(v_position - light_position);
                    float lum = max(dot(light_dir, v_normal), 0.0);
                    lum = max(lum, 0.2);
                    vec3 color = v_color * lum;
                    f_color = vec4(color, 1.0);
                }
            ";
