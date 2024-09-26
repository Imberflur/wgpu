@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

fn ptr_function(x: ptr<function, f32>) {}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    var vector = vec2<f32>(1.0);
    var x: f32;
    // Combination of x with default initializer, loading via OpAccessChain and storing to x, and passing ptr to a function call.
    x = vector.x;
    ptr_function(&x);
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
