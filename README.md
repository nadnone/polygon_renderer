# Rust 3D polygons engine :)

1. Work with the Phong model shader
2. Only with wavefront obj files
3. Don't use directly OpenGL, Vulkan or others 3D libraries
----------------------
## TODO :D
----------------------
- [x] Drawing a cube
- [ ] Load Wavefront files
    - [ ] Loading all data
    - [x] Specular, ambient, diffuse
    - [x] Vertices
    - [x] Normals

- [x] Shading with Phong model
- [ ] Using GPU instead of CPU
- [ ] Moving through the world
- [ ] Texture mapping
----------------------

| First shader | Specular shading |
| ----------------- | -------------------- |
| <img height="300" width="300" src="./res/cube_first.gif"> | <img height="300" width="300" src="./res/cube_specular.gif"> |

| Airplane failed test | Cat test success |
| --------------------- | ---------------- |
| <img height="300" width="300" src="./res/airplane.gif"> | <img height="300" width="300" src="./res/cat_shader_success.gif"> |
