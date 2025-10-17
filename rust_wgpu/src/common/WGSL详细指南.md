# WebGPU 着色器语言 (WGSL) 详细指南

## 目录
1. [简介](#简介)
2. [基本语法](#基本语法)
3. [数据类型](#数据类型)
4. [变量声明](#变量声明)
5. [函数](#函数)
6. [控制结构](#控制结构)
7. [着色器阶段](#着色器阶段)
8. [资源和绑定](#资源和绑定)
9. [内置函数和操作](#内置函数和操作)
10. [与 GLSL 的对比](#与-glsl-的对比)
11. [实践示例](#实践示例)

## 简介

WebGPU 着色器语言 (WGSL) 是 WebGPU API 的标准着色器语言，用于编写在 GPU 上运行的程序，称为着色器。WGSL 设计为一种现代、类型安全的着色语言，能够跨平台工作，并最终转换为各种底层图形 API 的原生着色器语言（如 HLSL、MSL、SPIR-V 等）<mcreference link="https://www.w3.org/TR/WGSL/" index="0">0</mcreference>。

WGSL 是一种命令式语言，通过一系列语句来指定行为。这些语句可以：
- 声明常量或变量
- 修改变量的内容
- 使用结构化编程构造修改执行顺序
- 计算表达式

## 基本语法

### 注释
WGSL 支持两种类型的注释：
```wgsl
// 这是单行注释

/* 这是
   多行注释 */
```

### 标识符
标识符用于命名变量、函数、类型等。WGSL 标识符必须以字母或下划线开头，后跟字母、数字或下划线。

### 关键字
WGSL 保留了一些关键字，不能用作标识符，如：`fn`, `var`, `let`, `if`, `else`, `for`, `while`, `return`, `struct`, `type` 等。

## 数据类型

### 标量类型
WGSL 提供了以下基本标量类型<mcreference link="https://blog.csdn.net/xuejianxinokok/article/details/131192474" index="1">1</mcreference>：

| 类型 | 描述 |
|------|------|
| `i32` | 32位有符号整数 |
| `u32` | 32位无符号整数 |
| `f32` | 32位浮点数 |
| `f16` | 16位浮点数（可选功能） |
| `bool` | 布尔值 |

### 向量类型
向量是相同类型标量的有序集合：

| 类型 | 描述 |
|------|------|
| `vec2<T>` | 2分量向量 |
| `vec3<T>` | 3分量向量 |
| `vec4<T>` | 4分量向量 |

其中 T 可以是 `i32`, `u32`, `f32`, `f16`, `bool`。

示例：
```wgsl
var position: vec3<f32>;
var color: vec4<f32>;
var flags: vec4<bool>;
```

### 矩阵类型
矩阵是向量的二维数组：

| 类型 | 描述 |
|------|------|
| `mat2x2<T>` | 2×2矩阵 |
| `mat2x3<T>` | 2×3矩阵 |
| `mat2x4<T>` | 2×4矩阵 |
| `mat3x2<T>` | 3×2矩阵 |
| `mat3x3<T>` | 3×3矩阵 |
| `mat3x4<T>` | 3×4矩阵 |
| `mat4x2<T>` | 4×2矩阵 |
| `mat4x3<T>` | 4×3矩阵 |
| `mat4x4<T>` | 4×4矩阵 |

### 数组类型
数组是相同类型元素的有序集合：

```wgsl
// 固定大小数组
var fixed_array: array<f32, 10>;

// 运行时大小数组（需要存储类）
var<storage> runtime_array: array<f32>;
```

### 结构体类型
结构体允许组合不同类型的数据：

```wgsl
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
};
```

### 原子类型
原子类型用于并发操作：

```wgsl
var<workgroup> atomic_counter: atomic<u32>;
```

## 变量声明

### 变量和常量
WGSL 中使用 `var` 声明变量，`let` 声明常量：

```wgsl
var a: f32 = 1.0;  // 可变变量
let b: f32 = 2.0;  // 不可变常量
```

### 自动类型推断
类似于 TypeScript，如果不显式指定类型，变量会自动推断为右边表达式的类型<mcreference link="https://blog.csdn.net/xuejianxinokok/article/details/131192474" index="1">1</mcreference>：

```wgsl
var a = 1;      // a 是 i32
let b = 2.0;    // b 是 f32
var c = 3u;     // c 是 u32
var d = true;   // d 是 bool
```

### 存储类
变量可以指定不同的存储类，决定其生命周期和可见性：

| 存储类 | 描述 |
|--------|------|
| `function` | 函数局部变量（默认） |
| `private` | 私有模块作用域变量 |
| `workgroup` | 工作组共享变量 |
| `uniform` | 只读统一缓冲区 |
| `storage` | 读写存储缓冲区 |
| `handle` | 资源句柄（纹理、采样器等） |

示例：
```wgsl
var<private> global_counter: u32;
var<workgroup> shared_data: array<f32, 10>;
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage> buffer: StorageBuffer;
```

### 地址空间
变量还可以指定地址空间，决定其在内存中的位置：

| 地址空间 | 描述 |
|----------|------|
| `function` | 函数栈空间 |
| `private` | 私有地址空间 |
| `workgroup` | 工作组共享地址空间 |
| `uniform` | 统一缓冲区地址空间 |
| `storage` | 存储缓冲区地址空间 |

## 函数

### 函数声明
函数使用 `fn` 关键字声明，必须指定参数类型和返回类型：

```wgsl
fn add(a: f32, b: f32) -> f32 {
    return a + b;
}

fn transform_position(pos: vec3<f32>, model: mat4x4<f32>) -> vec4<f32> {
    return model * vec4<f32>(pos, 1.0);
}
```

### 函数参数
函数参数可以是值传递或引用传递：

```wgsl
// 值传递
fn increment_value(value: u32) -> u32 {
    return value + 1;
}

// 引用传递
fn increment_reference(ptr: ptr<function, u32>) {
    *ptr = *ptr + 1;
}
```

### 内置函数
WGSL 提供了许多内置函数，如数学函数、纹理函数等：

```wgsl
// 数学函数
let result = sin(x);
let distance = length(vector);
let normalized = normalize(vector);

// 纹理函数
let color = textureSample(texture, sampler, uv);
let depth = textureLoad(depth_texture, coords, 0);
```

## 控制结构

### 条件语句
```wgsl
if (condition) {
    // 条件为真时执行
} else if (another_condition) {
    // 另一个条件为真时执行
} else {
    // 所有条件都为假时执行
}
```

### 循环语句
```wgsl
// for 循环
for (var i = 0u; i < 10u; i = i + 1u) {
    // 循环体
}

// while 循环
var i = 0u;
while (i < 10u) {
    // 循环体
    i = i + 1u;
}

// loop 循环（带 break 和 continue）
loop {
    // 循环体
    if (condition) {
        break;
    }
    if (another_condition) {
        continue;
    }
}
```

### switch 语句
```wgsl
switch (value) {
    case 0: {
        // 处理情况 0
    }
    case 1, 2: {
        // 处理情况 1 和 2
    }
    default: {
        // 默认情况
    }
}
```

## 着色器阶段

WGSL 支持多种着色器阶段，每个阶段有不同的用途和输入输出：

### 顶点着色器
处理顶点数据，生成裁剪空间位置：

```wgsl
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}
```

### 片段着色器
处理像素颜色：

```wgsl
@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
```

### 计算着色器
执行通用计算：

```wgsl
@workgroup_size(8, 8, 1)
@compute
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // 计算逻辑
}
```

## 资源和绑定

### 绑定组
资源通过绑定组组织：

```wgsl
// 绑定组 0
@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(0) @binding(1) var<storage> lights: array<Light>;

// 绑定组 1
@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;
```

### 资源类型
WGSL 支持多种资源类型：

| 类型 | 描述 |
|------|------|
| `texture_1d<T>` | 1D纹理 |
| `texture_2d<T>` | 2D纹理 |
| `texture_3d<T>` | 3D纹理 |
| `texture_cube<T>` | 立方体纹理 |
| `texture_multisampled_2d<T>` | 多重采样2D纹理 |
| `sampler` | 采样器 |
| `sampler_comparison` | 比较采样器 |

### 资源访问模式
资源可以有不同的访问模式：

| 模式 | 描述 |
|------|------|
| `read` | 只读 |
| `write` | 只写 |
| `read_write` | 读写 |

## 内置函数和操作

### 数学函数
WGSL 提供了丰富的数学函数：

```wgsl
// 基本数学运算
let sum = a + b;
let difference = a - b;
let product = a * b;
let quotient = a / b;
let remainder = a % b;

// 三角函数
let sine = sin(x);
let cosine = cos(x);
let tangent = tan(x);
let arcsine = asin(x);
let arccosine = acos(x);
let arctangent = atan(x);

// 指数和对数函数
let exponent = exp(x);
let logarithm = log(x);
let power = pow(x, y);
let square_root = sqrt(x);

// 向量操作
let dot_product = dot(a, b);
let cross_product = cross(a, b);
let length = length(vector);
let normalized = normalize(vector);
let reflected = reflect(incident, normal);
```

### 纹理函数
```wgsl
// 纹理采样
let color = textureSample(texture, sampler, uv);
let color_offset = textureSampleBias(texture, sampler, uv, bias);
let color_level = textureSampleLevel(texture, sampler, uv, level);

// 纹理加载
let texel = textureLoad(texture, coords, level);
let depth = textureLoad(depth_texture, coords, level);

// 纹理存储
textureStore(storage_texture, coords, color);
```

### 数据包函数
```wgsl
// 数据包操作
let packed = pack4x8snorm(vector);
let unpacked = unpack4x8snorm(packed);

// 位操作
let shifted = a << b;
let rotated = rotateLeft(a, b);
let reversed = reverseBits(a);
```

## 与 GLSL 的对比

WGSL 与传统的 GLSL 有许多相似之处，但也有一些重要区别<mcreference link="https://blog.csdn.net/weixin_44938311/article/details/127751484" index="2">2</mcreference>：

| 特性 | WGSL | GLSL |
|------|------|------|
| 类型声明 | 必须显式指定类型 | 可以隐式推断 |
| 向量构造 | `vec3<f32>(x, y, z)` | `vec3(x, y, z)` |
| 矩阵构造 | `mat4x4<f32>(...)` | `mat4(...)` |
| 内置变量 | 使用属性标记 | 使用内置变量如 `gl_Position` |
| 布局限定符 | `@location(0)` | `layout(location = 0)` |
| 入口点 | 任意函数名 | `main()` 函数 |

## 实践示例

### 基本三角形渲染
```wgsl
// 顶点着色器
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // 定义三角形顶点位置
    let pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    
    // 定义顶点颜色
    let colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),  // 红
        vec3<f32>(0.0, 1.0, 0.0),  // 绿
        vec3<f32>(0.0, 0.0, 1.0)   // 蓝
    );
    
    output.clip_position = vec4<f32>(pos[vertex_index], 0.0, 1.0);
    output.color = colors[vertex_index];
    
    return output;
}

// 片段着色器
@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
```

### 带光照的表面着色
```wgsl
// 结构体定义
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    camera_pos: vec3<f32>,
};

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
};

// 资源绑定
@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage> lights: array<Light>;
@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

// 顶点着色器
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    let world_pos = (uniforms.model * vec4<f32>(input.position, 1.0)).xyz;
    output.clip_position = uniforms.projection * uniforms.view * vec4<f32>(world_pos, 1.0);
    output.world_pos = world_pos;
    
    // 变换法线到世界空间
    output.normal = (uniforms.model * vec4<f32>(input.normal, 0.0)).xyz;
    output.uv = input.uv;
    
    return output;
}

// 片段着色器
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // 采样纹理
    let base_color = textureSample(color_texture, texture_sampler, input.uv);
    
    // 计算光照
    let N = normalize(input.normal);
    var surface_color = vec3<f32>(0.0);
    
    // 遍历所有光源
    for (var i = 0u; i < arrayLength(&lights); i = i + 1u) {
        let light = lights[i];
        let world_to_light = light.position - input.world_pos;
        let distance = length(world_to_light);
        let light_dir = normalize(world_to_light);
        
        // 计算光照贡献
        let radiance = light.color * light.intensity / (distance * distance);
        let n_dot_l = max(dot(N, light_dir), 0.0);
        
        // 累加光照贡献
        surface_color = surface_color + base_color.rgb * radiance * n_dot_l;
    }
    
    return vec4<f32>(surface_color, base_color.a);
}
```

### 计算着色器示例
```wgsl
// 粒子系统更新
struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
};

struct SimParams {
    delta_time: f32,
    gravity: vec3<f32>,
    particle_count: u32,
};

@group(0) @binding(0) var<uniform> params: SimParams;
@group(0) @binding(1) var<storage, read_write> particles: array<Particle>;

@workgroup_size(64)
@compute
fn update_particles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    
    if (index >= params.particle_count) {
        return;
    }
    
    var particle = particles[index];
    
    // 更新速度（应用重力）
    particle.velocity = particle.velocity + params.gravity * params.delta_time;
    
    // 更新位置
    particle.position = particle.position + particle.velocity * params.delta_time;
    
    // 更新生命值
    particle.life = particle.life - params.delta_time;
    
    // 如果生命值耗尽，重置粒子
    if (particle.life <= 0.0) {
        particle.position = vec3<f32>(0.0, 0.0, 0.0);
        particle.velocity = vec3<f32>(
            (random(index * 2u) - 0.5) * 2.0,
            (random(index * 2u + 1u) - 0.5) * 2.0,
            (random(index * 2u + 2u) - 0.5) * 2.0
        );
        particle.life = 1.0;
    }
    
    particles[index] = particle;
}

// 简单的伪随机函数
fn random(seed: u32) -> f32 {
    let x = sin(f32(seed)) * 43758.5453;
    return fract(x);
}
```

## 总结

WGSL 是一种现代、类型安全的着色器语言，专为 WebGPU 设计。它提供了强大的功能，包括：

1. 强类型系统，减少错误
2. 现代语言特性，如结构化控制流
3. 灵活的资源绑定系统
4. 丰富的内置函数库
5. 跨平台兼容性

掌握 WGSL 是开发 WebGPU 应用的关键，希望本指南能帮助您理解和使用这种强大的着色器语言。