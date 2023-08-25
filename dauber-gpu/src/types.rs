use std::mem::size_of;
use wgpu::{vertex_attr_array, VertexBufferLayout, VertexStepMode};

#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug)]
#[repr(C)]
pub struct Globals {
    pub view: [f32; 2],
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 2],
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Debug)]
#[repr(C)]
pub struct ShaderPaint {
    pub color: [f32; 4],
}

impl Vertex {
    pub const fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float32x2],
        }
    }
}
