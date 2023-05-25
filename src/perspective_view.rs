use std::borrow::Cow;

use crate::{
    constants::VERTICES_BUFFER_LAYOUT,
    texture::Texture,
};

pub struct PerspectiveView {
    label: String,
    width: u32,
    height: u32,
    texture: Texture,
    texture_id: egui::TextureId,
    render_pipeline: wgpu::RenderPipeline,
}

impl PerspectiveView {
    pub fn new(
        render_pass: &mut egui_wgpu_backend::RenderPass,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Self {
        let label = "perspective-view";
        
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(&(label.to_string() + "-shader")),
            source: wgpu::ShaderSource::Wgsl(
                Cow::Borrowed(include_str!("perspective_view.wgsl"))
            ),
        });

        let dimensions = (width, height);

        let texture = Texture::new(
            &device,
            dimensions,
            Some(&(label.to_string() + "-texture")),
            wgpu::TextureFormat::Rgba8UnormSrgb,
            false,
        );

        let texture_id = render_pass.egui_texture_from_wgpu_texture(
            device,
            &texture.texture,
            wgpu::FilterMode::Linear,
        );

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&(label.to_string() + "-pipeline-layout")),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&(label.to_string() + "-render-pipeline")),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VERTICES_BUFFER_LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: texture.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Line,
                conservative: false,
                clamp_depth: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        Self {
            label: String::from(label),
            width,
            height,
            texture,
            texture_id,
            render_pipeline,
        }
    }

    pub fn resize(
        &mut self,
        render_pass: &mut egui_wgpu_backend::RenderPass,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) {
        self.width = width;
        self.height = height;

        let dimensions = (width, height);

        self.texture = Texture::new(
            device,
            dimensions,
            Some(&(self.label.to_string() + "-texture")),
            wgpu::TextureFormat::Rgba8UnormSrgb,
            false,
        );

        self.texture_id = render_pass.egui_texture_from_wgpu_texture(
            device,
            &self.texture.texture,
            wgpu::FilterMode::Linear,
        );
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _device: &wgpu::Device) {
        ui.image(
            self.texture_id,
            (self.width as f32, self.height as f32),
        );
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        vertex_buffer_slice: wgpu::BufferSlice,
        index_buffer_slice: wgpu::BufferSlice,
        index_buffer_len: u32,
    ) {
        let label = self.label.to_string() + "-render-pass";

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&label),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &self.texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer_slice);
        render_pass.set_index_buffer(index_buffer_slice, wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..index_buffer_len, 0, 0..1);
    }
}