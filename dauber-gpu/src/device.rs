use crate::types::{Globals, Vertex};
use dauber_core::color::Color;
use dauber_core::device::Device;
use dauber_core::geom::point::Point;
use dauber_core::image_info::ImageInfo;
use dauber_core::paint::{Paint, PaintStyle};
use dauber_core::path;
use dauber_core::path::{Path, PathSegment};
use lyon::tessellation;
use lyon::tessellation::VertexBuffers;
use std::num::NonZeroU64;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupLayoutDescriptor, BufferDescriptor, DeviceDescriptor, Features, IndexFormat,
    InstanceDescriptor, Limits, PowerPreference, RequestAdapterOptions, TextureViewDescriptor,
};

#[derive(Debug)]
pub struct Wgpu {
    info: ImageInfo,
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_texture: wgpu::Texture,
    surface_texture_view: wgpu::TextureView,
    msaa_texture: wgpu::Texture,
    msaa_texture_view: wgpu::TextureView,

    globals_buffer: wgpu::Buffer,

    output_buffer: wgpu::Buffer,

    render_pipeline: wgpu::RenderPipeline,
    msaa_render_pipeline: wgpu::RenderPipeline,

    bind_group: wgpu::BindGroup,

    clear: Option<Color>,
}

impl Wgpu {
    pub fn flush(&mut self) {
        if let Some(_) = &self.clear {
            self.draw_path(&Path::new(), &Paint::new());
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.clear = Some(color);
    }

    pub fn encode_to_png(&mut self) -> Vec<u8> {
        self.flush();

        let mut v = Vec::new();
        {
            let slice = self.output_buffer.slice(..);
            let (tx, rx) = std::sync::mpsc::sync_channel(1);
            slice.map_async(wgpu::MapMode::Read, move |res| {
                tx.send(res).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.recv().unwrap().unwrap();
            let data = slice.get_mapped_range();

            let buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
                self.info.width,
                self.info.height,
                data,
            )
            .unwrap();

            let mut buf = std::io::Cursor::new(&mut v);

            buffer
                .write_to(&mut buf, image::ImageOutputFormat::Png)
                .unwrap();
        }

        self.output_buffer.unmap();

        v
    }
}

const U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

impl Device for Wgpu {
    fn new(info: ImageInfo) -> Self {
        let ImageInfo { width, height } = info;

        let instance = wgpu::Instance::new(InstanceDescriptor::default());

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::default(),
            },
            None,
        ))
        .unwrap();

        let mut texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };

        let texture_view_desc = TextureViewDescriptor::default();

        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&texture_view_desc);

        texture_desc.sample_count = 4;
        let msaa_texture = device.create_texture(&texture_desc);
        let msaa_texture_view = msaa_texture.create_view(&texture_view_desc);

        let output_buffer_size = (U32_SIZE * width * height) as wgpu::BufferAddress;
        let output_buffer_desc = BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST
                // this tells wpgu that we want to read this buffer from the cpu
                | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(
                        NonZeroU64::new(std::mem::size_of::<Globals>() as u64).unwrap(),
                    ),
                },
                count: None,
            }],
        });

        let globals_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: std::mem::size_of::<Globals>() as u64,
            mapped_at_creation: false,
        });

        queue.write_buffer(
            &globals_buffer,
            0,
            bytemuck::bytes_of(&Globals {
                view: [width as f32, height as f32],
            }),
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &globals_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let color_targets = [Some(wgpu::ColorTargetState {
            format: texture_desc.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let mut render_pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &color_targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };

        let render_pipeline = device.create_render_pipeline(&render_pipeline_desc);

        render_pipeline_desc.multisample.count = 4;
        let msaa_render_pipeline = device.create_render_pipeline(&render_pipeline_desc);

        Self {
            info,
            instance,
            adapter,
            device,
            queue,
            surface_texture: texture,
            surface_texture_view: texture_view,
            msaa_texture,
            msaa_texture_view,
            globals_buffer,
            output_buffer,
            render_pipeline,
            msaa_render_pipeline,
            bind_group,
            clear: None,
        }
    }

    fn image_info(&self) -> &ImageInfo {
        &self.info
    }

    fn draw_path(&mut self, path: &Path, paint: &Paint) {
        let mut first_pt = None::<Point>;
        let mut last_pt = None::<Point>;

        let mut buffers = VertexBuffers::<Vertex, u16>::new();

        match paint.style() {
            PaintStyle::Fill => {
                let mut b = tessellation::BuffersBuilder::new(
                    &mut buffers,
                    |vertex: tessellation::FillVertex| Vertex {
                        pos: vertex.position().to_array(),
                    },
                );

                let iter = LyonIter {
                    iter: path.iter(),
                    last: None,
                    first_pt: &mut first_pt,
                    last_pt: &mut last_pt,
                };

                tessellation::FillTessellator::new()
                    .tessellate(iter, &tessellation::FillOptions::even_odd(), &mut b)
                    .unwrap();
            }
            PaintStyle::Stroke(width) => {
                let mut b = tessellation::BuffersBuilder::new(
                    &mut buffers,
                    |vertex: tessellation::StrokeVertex| Vertex {
                        pos: vertex.position().to_array(),
                    },
                );

                let iter = LyonIter {
                    iter: path.iter(),
                    last: None,
                    first_pt: &mut first_pt,
                    last_pt: &mut last_pt,
                };

                tessellation::StrokeTessellator::new()
                    .tessellate(
                        iter,
                        &tessellation::StrokeOptions::default().with_line_width(width),
                        &mut b,
                    )
                    .unwrap();
            }
            PaintStyle::FillAndStroke(width) => {
                let mut b = tessellation::BuffersBuilder::new(
                    &mut buffers,
                    |vertex: tessellation::FillVertex| Vertex {
                        pos: vertex.position().to_array(),
                    },
                );

                let iter = LyonIter {
                    iter: path.iter(),
                    last: None,
                    first_pt: &mut first_pt,
                    last_pt: &mut last_pt,
                };

                tessellation::FillTessellator::new()
                    .tessellate(iter, &tessellation::FillOptions::even_odd(), &mut b)
                    .unwrap();

                let mut b = tessellation::BuffersBuilder::new(
                    &mut buffers,
                    |vertex: tessellation::StrokeVertex| Vertex {
                        pos: vertex.position().to_array(),
                    },
                );

                let iter = LyonIter {
                    iter: path.iter(),
                    last: None,
                    first_pt: &mut first_pt,
                    last_pt: &mut last_pt,
                };

                tessellation::StrokeTessellator::new()
                    .tessellate(
                        iter,
                        &tessellation::StrokeOptions::default().with_line_width(width),
                        &mut b,
                    )
                    .unwrap();
            }
        }

        let indices_len = buffers.indices.len() as u32;

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&buffers.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&buffers.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let (render_pipeline, view, resolve_target) = if paint.anti_alias {
            (
                &self.msaa_render_pipeline,
                &self.msaa_texture_view,
                Some(&self.surface_texture_view),
            )
        } else {
            (&self.render_pipeline, &self.surface_texture_view, None)
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: self
                            .clear
                            .take()
                            .map(|Color { r, g, b, a }| {
                                wgpu::LoadOp::Clear(wgpu::Color {
                                    r: r as f64,
                                    g: g as f64,
                                    b: b as f64,
                                    a: a as f64,
                                })
                            })
                            .unwrap_or(wgpu::LoadOp::Load),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
            rpass.draw_indexed(0..indices_len, 0, 0..1);
        }

        let ImageInfo { width, height } = self.info;

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.surface_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(U32_SIZE * width),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit([encoder.finish()]);
    }
}

struct LyonIter<'a> {
    iter: path::Iter<'a>,
    last: Option<PathSegment>,
    first_pt: &'a mut Option<Point>,
    last_pt: &'a mut Option<Point>,
}

impl<'a> Iterator for LyonIter<'a> {
    type Item = lyon::path::PathEvent;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(seg) => {
                self.last = Some(seg.clone());
                Some(path_seg_to_lyon_event(seg, self.first_pt, self.last_pt))
            }
            None => match self.last.take() {
                Some(PathSegment::Close) | None => None,
                Some(_) => Some(lyon::path::PathEvent::End {
                    last: lyon_point((*self.last_pt)?),
                    first: lyon_point((*self.first_pt)?),
                    close: false,
                }),
            },
        }
    }
}

fn lyon_point(Point { x, y }: Point) -> lyon::math::Point {
    lyon::math::point(x, y)
}

fn path_seg_to_lyon_event(
    seg: PathSegment,
    first_pt: &mut Option<Point>,
    last_pt: &mut Option<Point>,
) -> lyon::path::PathEvent {
    match seg {
        PathSegment::Move { to } => {
            *first_pt = Some(to);
            *last_pt = Some(to);
            lyon::path::PathEvent::Begin { at: lyon_point(to) }
        }
        PathSegment::Line { from, to } => {
            *last_pt = Some(to);
            lyon::path::PathEvent::Line {
                from: lyon_point(from),
                to: lyon_point(to),
            }
        }
        PathSegment::Quadratic { from, ctrl, to } => {
            *last_pt = Some(to);
            lyon::path::PathEvent::Quadratic {
                from: lyon_point(from),
                ctrl: lyon_point(ctrl),
                to: lyon_point(to),
            }
        }
        PathSegment::Cubic {
            from,
            ctrl1,
            ctrl2,
            to,
        } => {
            *last_pt = Some(to);
            lyon::path::PathEvent::Cubic {
                from: lyon_point(from),
                ctrl1: lyon_point(ctrl1),
                ctrl2: lyon_point(ctrl2),
                to: lyon_point(to),
            }
        }
        PathSegment::Close => lyon::path::PathEvent::End {
            last: lyon_point(last_pt.unwrap()),
            first: lyon_point(first_pt.unwrap()),
            close: true,
        },
    }
}
