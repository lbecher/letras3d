use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{
    object::Object,
    orthographic_view::{Orientation, OrthographicView},
    perspective_view::PerspectiveView,
    types::{
        FaceType,
        ShaderVertex,
    }
};

pub const SIDEBAR_WIDTH: u32 = 320;
pub const VIEWS_SPACING: u32 = 10;

#[derive(Debug, PartialEq)]
enum Visualization {
    All,
    Front,
    Side,
    Top,
    Perspective,
}

pub struct Application {
    // wgpu
    instance: wgpu::Instance,
    surface_config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // vistas
    front_view: OrthographicView,
    side_view: OrthographicView,
    top_view: OrthographicView,
    pub perspective_view: PerspectiveView,
    // lista de objetos
    objects: Vec<Object>,
    // buffers
    vertices_buffer: wgpu::Buffer,
    lines_indices_buffer: wgpu::Buffer,
    lines_indices_len: u32,
    // egui
    platform: egui_winit_platform::Platform,
    render_pass: egui_wgpu_backend::RenderPass,
    start_time: std::time::Instant,
    previous_frame_time: Option<f32>,
    // gui state
    visualization: Visualization,
    pub selected: Option<usize>,
    extrusion: bool,
    extrusion_string: String,
    extrusion_string_parsing_error: bool,
    x_scale_string: String,
    y_scale_string: String,
    z_scale_string: String,
    scale_string_parsing_error: bool,
    x_position_string: String,
    y_position_string: String,
    z_position_string: String,
    position_string_parsing_error: bool,
    x_rotation: f64,
    y_rotation: f64,
    z_rotation: f64,
}

impl Application {
    pub async fn new(window: &Window) -> Self {
        // tamanho da janela
        let size = window.inner_size();

        // wgpu::Backends::PRIMARY => Vulkan (Linux) / Metal (macOS) / DX12 (Windows)
        let backends = wgpu::Backends::PRIMARY;

        // instance é um handle para a GPU
        let instance = wgpu::Instance::new(backends);

        // obtém superfície de renderização (parte da janela que será desenhada)
        let surface = unsafe { instance.create_surface(window) };

        // obtém identificador da GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance, // escolhe GPU com mais performance
                compatible_surface: Some(&surface),
                force_fallback_adapter: false, // evita renderização via software
            })
            .await
            .unwrap();

        // lista de recursos disponíveis para a GPU selecionada
        let features = adapter.features();

        // obtém descritor do dispositivo e fila
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features,
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        // obtém formato de textura suportado
        let format = surface
            .get_preferred_format(&adapter)
            .unwrap();

        // condiguração da superfíce de renderização
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        // aplica configuração
        surface.configure(&device, &config);

        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: size.width,
                physical_height: size.height,
                scale_factor: window.scale_factor(),
                font_definitions: egui::FontDefinitions::default(),
                style: Default::default(),
            });

        let mut render_pass = egui_wgpu_backend::RenderPass::new(&device, format, 1);

        let views_width = (size.width - (SIDEBAR_WIDTH + (3 * VIEWS_SPACING))) / 2;
        let views_height = (size.height - (3 * VIEWS_SPACING)) / 2;

        let front_view = OrthographicView::new(
            &mut render_pass,
            &device,
            views_width,
            views_height,
            Orientation::Front,
        );

        let side_view = OrthographicView::new(
            &mut render_pass,
            &device,
            views_width,
            views_height,
            Orientation::Side,
        );

        let top_view = OrthographicView::new(
            &mut render_pass,
            &device,
            views_width,
            views_height,
            Orientation::Top,
        );

        let perspective_view = PerspectiveView::new(
            &mut render_pass, 
            &device,
            views_width, 
            views_height
        );

        let objects: Vec<Object> = Vec::new();

        let vertices: Vec<ShaderVertex> = Vec::new();
        let lines_indices: Vec<u32> = Vec::new();

        let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex-buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let lines_indices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line-indices-buffer"),
            contents: bytemuck::cast_slice(&lines_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            instance,
            surface_config: config,
            surface,
            adapter,
            device,
            queue,
            front_view,
            side_view,
            top_view,
            perspective_view,
            objects,
            vertices_buffer,
            lines_indices_buffer,
            lines_indices_len: 0,
            platform,
            render_pass,
            start_time: std::time::Instant::now(),
            previous_frame_time: None,
            visualization: Visualization::All,
            selected: None,
            extrusion: false,
            extrusion_string: String::new(),
            extrusion_string_parsing_error: false,
            x_position_string: String::new(),
            y_position_string: String::new(),
            z_position_string: String::new(),
            position_string_parsing_error: false,
            x_scale_string: String::new(),
            y_scale_string: String::new(),
            z_scale_string: String::new(),
            scale_string_parsing_error: false,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
        }
    }

    pub fn handle_event<T>(&mut self, winit_event: &winit::event::Event<T>) {
        self.platform.handle_event(winit_event)
    }

    pub fn captures_event<T>(&self, winit_event: &winit::event::Event<T>) -> bool {
        self.platform.captures_event(winit_event)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
        self.resize_views(width, height);
    }

    fn resize_views(&mut self, width: u32, height: u32) {
        match self.visualization {
            Visualization::All => {
                let views_width = (width - (3 * VIEWS_SPACING + SIDEBAR_WIDTH)) / 2;
                let views_height = (height - (3 * VIEWS_SPACING)) / 2;

                self.front_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    views_width, 
                    views_height,
                );

                self.side_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    views_width, 
                    views_height,
                );

                self.top_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    views_width, 
                    views_height,
                );

                self.perspective_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    views_width, 
                    views_height,
                );
            }
            Visualization::Front => {
                self.front_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    width - (2 * VIEWS_SPACING + SIDEBAR_WIDTH), 
                    height - (2 * VIEWS_SPACING),
                );
            }
            Visualization::Side => {
                self.side_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    width - (2 * VIEWS_SPACING + SIDEBAR_WIDTH), 
                    height - (2 * VIEWS_SPACING),
                );
            }
            Visualization::Top => {
                self.top_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    width - (2 * VIEWS_SPACING + SIDEBAR_WIDTH), 
                    height - (2 * VIEWS_SPACING),
                );
            }
            Visualization::Perspective => {
                self.perspective_view.resize(
                    &mut self.render_pass,
                    &self.device,
                    width - (2 * VIEWS_SPACING + SIDEBAR_WIDTH), 
                    height - (2 * VIEWS_SPACING),
                );
            }
        }
    }

    fn show(&mut self) {
        let ctx = &self.platform.context();
        egui::SidePanel::right("Painel de Controle")
            .min_width(SIDEBAR_WIDTH as f32)
            .max_width(SIDEBAR_WIDTH as f32)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = VIEWS_SPACING as f32;

                    ui.horizontal(|ui| {
                        ui.label("Vista:");
                        if egui::ComboBox::from_label("")
                            .selected_text(match self.visualization {
                                Visualization::All => "Todas",
                                Visualization::Front => "Frente",
                                Visualization::Side => "Lado",
                                Visualization::Top => "Topo",
                                Visualization::Perspective => "Perspectiva",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.visualization,
                                    Visualization::All,
                                    "Todas",
                                ).clicked() || ui.selectable_value(
                                    &mut self.visualization,
                                    Visualization::Front,
                                    "Frente",
                                ).clicked() || ui.selectable_value(
                                    &mut self.visualization,
                                    Visualization::Side,
                                    "Lado",
                                ).clicked() || ui.selectable_value(
                                    &mut self.visualization,
                                    Visualization::Top,
                                    "Topo",
                                ).clicked() || ui.selectable_value(
                                    &mut self.visualization,
                                    Visualization::Perspective,
                                    "Perspectiva",
                                ).clicked()
                            })
                            .inner
                            .unwrap_or(false)
                        {
                            self.resize_views(self.surface_config.width, self.surface_config.height);
                        }
                    });

                    if let Some(selected) = self.selected {
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            ui.checkbox(&mut self.extrusion, "Extrusão");
                            if self.extrusion {
                                ui.add(egui::TextEdit::singleline(&mut self.extrusion_string));
                                if self.extrusion_string_parsing_error {
                                    ui.label("Algo aqui não é ponto flutuante!");
                                }
                                ui.horizontal(|ui| {
                                    if ui.button("Aplicar").clicked() {
                                        if let Ok(extrusion) = self.extrusion_string.parse::<f64>() {
                                            self.objects[selected].set_extrusion(Some(extrusion));
                                            self.update();
                                            self.extrusion_string_parsing_error = false;
                                        } else {
                                            self.extrusion_string_parsing_error = true;
                                        }
                                    }
                                    if ui.button("Restaurar").clicked() {
                                        match self.objects[selected].extrusion {
                                            Some(extrusion) => {
                                                self.extrusion_string = format!(
                                                    "{}",
                                                    extrusion,
                                                );
                                            }
                                            None => {
                                                self.extrusion_string = String::new();
                                                self.extrusion = false;
                                            }
                                        }
                                    }
                                });
                            } else {
                                if self.objects[selected].extrusion.is_some() {
                                    self.objects[selected].set_extrusion(None);
                                    self.update();
                                }
                            }
                        });

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            ui.label("Posição:");
                            ui.add(egui::TextEdit::singleline(&mut self.x_position_string));
                            ui.add(egui::TextEdit::singleline(&mut self.y_position_string));
                            ui.add(egui::TextEdit::singleline(&mut self.z_position_string));
                            if self.position_string_parsing_error {
                                ui.label("Algo aqui não é ponto flutuante!");
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Aplicar").clicked() {
                                    if let (
                                        Ok(xp),
                                        Ok(yp),
                                        Ok(zp),
                                    ) = (
                                        self.x_position_string.parse::<f64>(),
                                        self.y_position_string.parse::<f64>(),
                                        self.z_position_string.parse::<f64>(),
                                    ) {
                                        self.objects[selected].set_position([
                                            xp,
                                            yp,
                                            zp,
                                            1.0,
                                        ]);
                                        self.update();
                                        self.position_string_parsing_error = false;
                                    } else {
                                        self.position_string_parsing_error = true;
                                    }
                                }
                                if ui.button("Restaurar").clicked() {
                                    self.x_position_string = format!(
                                        "{}",
                                        self.objects[selected].position[0],
                                    );
                                    self.y_position_string = format!(
                                        "{}",
                                        self.objects[selected].position[1],
                                    );
                                    self.z_position_string = format!(
                                        "{}",
                                        self.objects[selected].position[2],
                                    );
                                }
                            });
                        });

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            ui.label("Escala:");
                            ui.add(egui::TextEdit::singleline(&mut self.x_scale_string));
                            ui.add(egui::TextEdit::singleline(&mut self.y_scale_string));
                            ui.add(egui::TextEdit::singleline(&mut self.z_scale_string));
                            if self.scale_string_parsing_error {
                                ui.label("Algo aqui não é ponto flutuante!");
                            }
                            ui.horizontal(|ui| {
                                if ui.button("Aplicar").clicked() {
                                    if let (
                                        Ok(xs),
                                        Ok(ys),
                                        Ok(zs),
                                    ) = (
                                        self.x_scale_string.parse::<f64>(),
                                        self.y_scale_string.parse::<f64>(),
                                        self.z_scale_string.parse::<f64>(),
                                    ) {
                                        self.objects[selected].set_scale([
                                            xs,
                                            ys,
                                            zs,
                                        ]);
                                        self.update();
                                        self.scale_string_parsing_error = false;
                                    } else {
                                        self.scale_string_parsing_error = true;
                                    }
                                }
                                if ui.button("Restaurar").clicked() {
                                    self.x_scale_string = format!(
                                        "{}",
                                        self.objects[selected].scale[0],
                                    );
                                    self.y_scale_string = format!(
                                        "{}",
                                        self.objects[selected].scale[1],
                                    );
                                    self.z_scale_string = format!(
                                        "{}",
                                        self.objects[selected].scale[2],
                                    );
                                }
                            });
                        });

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            ui.label("Rotação:");
                            if ui.add(egui::Slider::new(
                                &mut self.x_rotation, -180.0..=180.0
                            )).changed() {
                                self.objects[selected].set_rotation([
                                    self.x_rotation,
                                    self.y_rotation,
                                    self.z_rotation,
                                ]);
                                self.update();
                            }
                            if ui.add(egui::Slider::new(
                                &mut self.y_rotation, -180.0..=180.0
                            )).changed() {
                                self.objects[selected].set_rotation([
                                    self.x_rotation,
                                    self.y_rotation,
                                    self.z_rotation,
                                ]);
                                self.update();
                            }
                            if ui.add(egui::Slider::new(
                                &mut self.z_rotation, -180.0..=180.0
                            )).changed() {
                                self.objects[selected].set_rotation([
                                    self.x_rotation,
                                    self.y_rotation,
                                    self.z_rotation,
                                ]);
                                self.update();
                            }
                        });
                    }
                });
            });

        egui::CentralPanel::default()
            .show(ctx, |ui| match self.visualization {
                Visualization::All => {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = VIEWS_SPACING as f32;
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = VIEWS_SPACING as f32;
                            self.front_view.show(ui, &self.device);
                            self.side_view.show(ui, &self.device);
                        });
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = VIEWS_SPACING as f32;
                            self.top_view.show(ui, &self.device);
                            self.perspective_view.show(ui, &self.device);
                        });
                    });
                }
                Visualization::Front => {
                    self.front_view.show(ui, &self.device);
                }
                Visualization::Side => {
                    self.side_view.show(ui, &self.device);
                }
                Visualization::Top => {
                    self.top_view.show(ui, &self.device);
                }
                Visualization::Perspective => {
                    self.perspective_view.show(ui, &self.device);
                }
            });
    }

    pub fn inc_selected(&mut self) {
        if self.selected.is_none() {
            if self.objects.len() > 0 {
                self.selected = Some(0);
            }
        } else {
            let selected = self.selected.unwrap();
            if selected + 1 < self.objects.len() {
                self.selected = Some(selected + 1);
            }
        }

        self.update();
    }

    pub fn dec_selected(&mut self) {
        if self.selected.is_none() {
            if self.objects.len() > 0 {
                self.selected = Some(0);
            }
        } else {
            let selected = self.selected.unwrap();
            if selected > 0 {
                self.selected = Some(selected - 1);
            }
        }

        self.update();
    }

    pub fn del_object(&mut self, index: usize) {
        if self.objects.len() > index {
            self.objects.remove(index);
            if let Some(selected) = self.selected {
                if self.objects.len() == 0 {
                    self.selected = None;
                } else if self.objects.len() == selected {
                    self.selected = Some(selected - 1);
                }
            }
            self.update();
        }
    }

    pub fn add_object(&mut self, faces: &[FaceType]) {
        let mut object = Object::new([
            self.objects.len() as f64,
            0.0,
            0.0,
            1.0,
        ]);

        for face in faces {
            object.add_face(*face);
        }

        self.objects.push(object);

        self.update();
    }

    fn update(&mut self) {
        match self.selected {
            Some(selected) => {
                self.extrusion = self.objects[selected as usize].extrusion.is_some();
                if self.extrusion {
                    self.extrusion_string = format!(
                        "{}",
                        self.objects[selected as usize].extrusion.unwrap(),
                    );
                } else {
                    self.extrusion_string = String::new();
                }

                self.x_position_string = format!(
                    "{}",
                    self.objects[selected].position[0],
                );
                self.y_position_string = format!(
                    "{}",
                    self.objects[selected].position[1],
                );
                self.z_position_string = format!(
                    "{}",
                    self.objects[selected].position[2],
                );

                self.x_scale_string = format!(
                    "{}",
                    self.objects[selected].scale[0],
                );
                self.y_scale_string = format!(
                    "{}",
                    self.objects[selected].scale[1],
                );
                self.z_scale_string = format!(
                    "{}",
                    self.objects[selected].scale[2],
                );

                self.x_rotation = self.objects[selected].rotation[0];
                self.y_rotation = self.objects[selected].rotation[1];
                self.z_rotation = self.objects[selected].rotation[2];
            }
            None => {
                self.extrusion_string = String::new();

                self.x_position_string = String::new();
                self.y_position_string = String::new();
                self.z_position_string = String::new();

                self.x_scale_string = String::new();
                self.y_scale_string = String::new();
                self.z_scale_string = String::new();

                self.x_rotation = 0.0;
                self.y_rotation = 0.0;
                self.z_rotation = 0.0;
            }
        }

        let mut vertices: Vec<ShaderVertex> = Vec::new();
        let mut lines_indices: Vec<u32> = Vec::new();

        let mut count: usize = 0;

        for object in self.objects.iter() {
            let vertices_len = vertices.len();

            let object_vertices = object.get_vertices_vec();
            let object_lines_indices_vec = object.get_lines_indices_vec();

            for vertex in object_vertices {
                if let Some(selected) = self.selected {
                    if count == selected {
                        vertices.push(ShaderVertex {
                            position: vertex.position,
                            color: [1.0, 0.0, 1.0],
                        });
                    } else {
                        vertices.push(vertex);
                    }
                } else {
                    vertices.push(vertex);
                }
            }

            for index in object_lines_indices_vec {
                lines_indices.push(index + (vertices_len as u32));
            }

            count += 1;
        }

        self.vertices_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex-buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.lines_indices_len = lines_indices.len() as u32;

        self.lines_indices_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line-indices-buffer"),
            contents: bytemuck::cast_slice(&lines_indices),
            usage: wgpu::BufferUsages::INDEX,
        });
    }

    pub fn render(&mut self, scale_factor: f32) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;

        let view = frame.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        self.platform.update_time(
            self.start_time.elapsed().as_secs_f64()
        );

        let start = std::time::Instant::now();
        self.platform.begin_frame();

        self.show();

        let (_, paint_commands) = self.platform.end_frame(None);
        let paint_jobs = self.platform.context().tessellate(paint_commands);

        {
            let frame_time = (std::time::Instant::now() - start).as_secs_f32();
            self.previous_frame_time = Some(frame_time);
        }

        {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });

            match self.visualization {
                Visualization::All => {
                    encoder.push_debug_group("orthographic-view-front-render");
                    self.front_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();

                    encoder.push_debug_group("orthographic-view-side-render");
                    self.side_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();

                    encoder.push_debug_group("orthographic-view-top-render");
                    self.top_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();

                    encoder.push_debug_group("orthographic-view-perspective-render");
                    self.perspective_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();
                }
                Visualization::Front => {
                    encoder.push_debug_group("orthographic-view-front-render");
                    self.front_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();
                }
                Visualization::Side => {
                    encoder.push_debug_group("orthographic-view-side-render");
                    self.side_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();
                }
                Visualization::Top => {
                    encoder.push_debug_group("orthographic-view-top-render");
                    self.top_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();
                }
                Visualization::Perspective => {
                    encoder.push_debug_group("orthographic-view-perspective-render");
                    self.perspective_view.render(
                        &mut encoder,
                        self.vertices_buffer.slice(..),
                        self.lines_indices_buffer.slice(..),
                        self.lines_indices_len as u32,
                    );
                    encoder.pop_debug_group();
                }
            }

            self.queue.submit(Some(encoder.finish()));
        }

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("gui-encoder"),
            }
        );

        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: self.surface_config.width,
            physical_height: self.surface_config.height,
            scale_factor,
        };

        self.render_pass.update_texture(
            &self.device,
            &self.queue,
            &self.platform.context().texture(),
        );

        self.render_pass.update_user_textures(&self.device, &self.queue);

        self.render_pass.update_buffers(
            &self.device,
            &self.queue,
            &paint_jobs,
            &screen_descriptor
        );

        self.render_pass.execute(
            &mut encoder, 
            &view, 
            &paint_jobs, 
            &screen_descriptor, 
            None
        ).unwrap();

        self.queue.submit(Some(encoder.finish()));

        frame.present();

        Ok(())
    }
}
