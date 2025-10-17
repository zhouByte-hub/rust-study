// 基础内容
// var == let mut; let = let

/**
*   var<private> - 私有变量，只能在单个着色器阶段内使用
*   var<uniform> - 统一变量，可以从 CPU 传递数据到 GPU
*   var<storage> - 存储变量，用于在多个着色器阶段之间传递数据（用于读写缓冲区）
*   var<workgroup> - 工作组变量，用于计算着色器中的工作组内共享
*/
var<private> GLOBAL: i32 = 10;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@binding(0) @group(0) var<uniform> frame : u32;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // 使用 switch 语句根据 vertex_index 选择顶点位置和颜色
    var pos: vec2<f32>;
    var color: vec3<f32>;
    
    switch vertex_index {
        case 0u: {
            pos = vec2<f32>(0.0, 0.5);
            color = vec3<f32>(1.0, 0.0, 0.0);  // 红
            let a = mat4x4<f32>(
                vec4<f32>(1.0, 0.0, 0.0, 0.0),
                vec4<f32>(0.0, 1.0, 0.0, 0.0),
                vec4<f32>(0.0, 0.0, 1.0, 0.0),
                vec4<f32>(0.0, 0.0, 0.0, 1.0)
            );
        }
        case 1u: {
            pos = vec2<f32>(-0.5, -0.5);
            color = vec3<f32>(0.0, 1.0, 0.0);  // 绿
        }
        default: {
            pos = vec2<f32>(0.5, -0.5);
            color = vec3<f32>(0.0, 0.0, 1.0);  // 蓝
        }
    }
    
    output.clip_position = vec4<f32>(pos, 0.0, 1.0);
    output.color = color;
    
    return output;
}

@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}

@workgroup_size(8, 8, 1)
@compute
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // 计算逻辑
}