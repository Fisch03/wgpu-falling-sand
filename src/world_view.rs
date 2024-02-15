use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

const PIXEL_DIVISOR: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RenderViewUniform {
    screen_size: [f32; 2],
    view_size: [f32; 2],
}

pub struct WorldView {
    pub size: (u32, u32),

    pub render_bind_group_layout: wgpu::BindGroupLayout,
    pub render_bind_group: wgpu::BindGroup,

    render_view_uniform: RenderViewUniform,
    render_view_uniform_buffer: wgpu::Buffer,

    buffer: wgpu::Buffer,
}

impl WorldView {
    pub fn get_render_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.render_bind_group_layout
    }

    fn make_render_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }

    pub fn make_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(self.buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        self.render_view_uniform_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    pub fn new(screen_size: PhysicalSize<u32>, device: &wgpu::Device) -> Self {
        let size = (
            screen_size.width / PIXEL_DIVISOR,
            screen_size.height / PIXEL_DIVISOR,
        );

        let content_size = usize::try_from(size.0 * size.1).unwrap();
        let mut buffer_content = Vec::<u32>::with_capacity(content_size);
        for _y in 0..size.1 {
            for _x in 0..size.0 {
                if _x > 50 && _x < 70 && _y > 50 && _y < 80 {
                    buffer_content.push(1);
                } else {
                    buffer_content.push(0);
                }
            }
        }

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&buffer_content),
        });

        let render_view_uniform = RenderViewUniform {
            screen_size: [screen_size.width as f32, screen_size.height as f32],
            view_size: [size.0 as f32, size.1 as f32],
        };

        let render_view_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&render_view_uniform),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let render_bind_group_layout = WorldView::make_render_bind_group_layout(device);

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        render_view_uniform_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        Self {
            size,

            render_bind_group,
            render_bind_group_layout,

            render_view_uniform,
            render_view_uniform_buffer,

            buffer,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        println!("resize not implemented yet");
    }
}
