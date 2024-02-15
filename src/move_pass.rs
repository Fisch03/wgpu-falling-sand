use crate::world_view::WorldView;
use std::borrow::Cow;

pub struct MovePass {
    pipeline: wgpu::ComputePipeline,

    view_bind_group: wgpu::BindGroup,
}

impl MovePass {
    pub fn new(view: &WorldView, device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("move.wgsl"))),
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Move Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        let view_bind_group = view.make_bind_group(device, &pipeline.get_bind_group_layout(0));

        Self {
            pipeline,
            view_bind_group,
        }
    }

    pub fn execute(&mut self, view: &WorldView, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.view_bind_group, &[]);

        let work_group_dim = (u32::div_ceil(view.size.0, 8), u32::div_ceil(view.size.1, 8));
        pass.dispatch_workgroups(work_group_dim.0, work_group_dim.1, 1);
    }
}
