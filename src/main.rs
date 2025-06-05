mod flow_particle;
mod flow_vector;
mod model;
mod renderer;
mod lut_manager;

use egui_wgpu::wgpu;
use egui_winit::egui;
use log::info;
use model::{update, Model, enums::RedrawBackground};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use glam::Vec2;
use crate::model::constants::DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT;
use crate::renderer::Renderer;
use std::sync::Arc;
struct App {
    model: Model,
    egui_renderer: egui_wgpu::Renderer,
    egui_state: egui_winit::State,
    egui_ctx: egui::Context,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    renderer: Renderer,
    egui_shapes: Vec<egui::epaint::ClippedShape>,
    egui_textures_delta: egui::TexturesDelta,
    window: Arc<winit::window::Window>,
    should_clear_screen: bool,
    is_paused: bool,
}

impl App {
    async fn new(window: Arc<winit::window::Window>, event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Self {
        let size = window.inner_size();
        
        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });
        let surface = instance.create_surface(window.clone()).unwrap();
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Initialize EGUI
        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1);
        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &event_loop,
            Some(window.scale_factor() as f32),
            None,
        );
        
        // Use default EGUI visuals (with proper gray backgrounds)
        egui_ctx.set_visuals(egui::Visuals::default());
        
        // Initialize your Model
        let model = Model::new(Vec2::new(size.width as f32, size.height as f32));

        // Initialize renderer
        let renderer = Renderer::new(&device, surface_format, (size.width, size.height));

        Self {
            model,
            egui_renderer,
            egui_state,
            egui_ctx,
            surface,
            device,
            queue,
            config,
            size,
            renderer,
            egui_shapes: Vec::new(),
            egui_textures_delta: egui::TexturesDelta::default(),
            window,
            should_clear_screen: true,
            is_paused: false,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Update model's window rect
            self.model.window_rect = crate::model::SimpleRect::from_w_h(
                new_size.width as f32, 
                new_size.height as f32
            );
            
            // Resize trail texture
            self.renderer.resize(&self.device, (new_size.width, new_size.height));
            
            use crate::model::update::resized;
            resized(&mut self.model);
        }
    }

    fn input(&mut self, window: &winit::window::Window, event: &WindowEvent) -> bool {
        self.egui_state.on_window_event(window, event).consumed
    }

    fn update(&mut self) {
        if !self.is_paused {
            update(&mut self.model);
        }
    }

    fn handle_keyboard_input(&mut self, event: &winit::event::KeyEvent) {
        if event.state == winit::event::ElementState::Pressed {
            match &event.logical_key {
                winit::keyboard::Key::Character(c) if c == "c" => {
                    self.should_clear_screen = true;
                }
                _ => {
                    use crate::model::update::key_pressed;
                    key_pressed(&mut self.model, event);
                }
            }
        }
    }

    fn handle_mouse_input(&mut self, button: winit::event::MouseButton, state: winit::event::ElementState) {
        use crate::model::update::{mouse_pressed, mouse_released};
        match state {
            winit::event::ElementState::Pressed => mouse_pressed(&mut self.model, button, state),
            winit::event::ElementState::Released => mouse_released(&mut self.model, button),
        }
    }

    fn handle_mouse_moved(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        use crate::model::update::mouse_moved;
        
        // Convert screen coordinates to model coordinates 
        let model_pos = Vec2::new(
            position.x as f32 - self.size.width as f32 / 2.0,
            self.size.height as f32 / 2.0 - position.y as f32,
        );
        mouse_moved(&mut self.model, model_pos);
    }

    fn render_ui(&mut self, window: &winit::window::Window) {
        let raw_input = self.egui_state.take_egui_input(window);
        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            if self.model.show_ui {
                egui::Window::new("Controls")
                    .default_pos([10.0, 10.0])
                    .default_size([300.0, 600.0])
                    .collapsible(true)
                    .show(ctx, |ui| {
                        if ui.button("Hide Controls").clicked() {
                            self.model.show_ui = false;
                            info!("Controls hidden, press \"/\" to show them again");
                        }

                        ui.add_space(10.0);

                        let mut noise_seed = self.model.noise_seed as i32;
                        ui.label("Noise Seed");
                        if ui.add(egui::DragValue::new(&mut noise_seed)
                            .speed(1)
                            .clamp_range(0..=100_000))
                            .changed()
                        {
                            self.model.noise_seed = noise_seed as u32;
                            self.model.regen_flow_vectors();
                        }

                        ui.add_space(10.0);

                        // Noise Type Selection
                        let current_noise_type = self.model.flow_vector_field_builder_type.clone();
                        egui::ComboBox::from_label("Noise Type")
                            .selected_text(format!("{:?}", current_noise_type.clone()))
                            .show_ui(ui, |ui| {
                                let types = [
                                    "RightHandCurve", "BasicMulti", "Billow", "TerracedBillow",
                                    "Fbm", "HybridMulti", "OpenSimplex", "Value", "Worley"
                                ];
                                for noise_type in types {
                                    if ui.selectable_label(
                                        format!("{:?}", current_noise_type.clone()) == noise_type,
                                        noise_type,
                                    ).clicked() {
                                        self.model.flow_vector_field_builder_type = match noise_type {
                                            "RightHandCurve" => crate::flow_vector::FlowVectorFieldBuilder::RightHandCurve,
                                            "BasicMulti" => crate::flow_vector::FlowVectorFieldBuilder::BasicMulti,
                                            "Billow" => crate::flow_vector::FlowVectorFieldBuilder::Billow,
                                            "TerracedBillow" => crate::flow_vector::FlowVectorFieldBuilder::TerracedBillow,
                                            "Fbm" => crate::flow_vector::FlowVectorFieldBuilder::Fbm,
                                            "HybridMulti" => crate::flow_vector::FlowVectorFieldBuilder::HybridMulti,
                                            "OpenSimplex" => crate::flow_vector::FlowVectorFieldBuilder::OpenSimplex,
                                            "Value" => crate::flow_vector::FlowVectorFieldBuilder::Value,
                                            "Worley" => crate::flow_vector::FlowVectorFieldBuilder::Worley,
                                            _ => current_noise_type.clone(),
                                        };
                                        self.model.new_flow_vector_fn = self.model.flow_vector_field_builder_type.as_fn();
                                        self.model.regen_flow_vectors();
                                    }
                                }
                            });

                        ui.add_space(10.0);

                        let mut noise_scale = self.model.noise_scale as f32;
                        if ui.add(egui::Slider::new(&mut noise_scale, 0.001..=1.0).text("Noise Scale")).changed() {
                            self.model.noise_scale = noise_scale as f64;
                            self.model.regen_flow_vectors();
                        }

                        ui.add_space(10.0);

                        let mut particle_lifetime = self.model.particle_lifetime;
                        if ui.add(egui::Slider::new(&mut particle_lifetime, 10.0..=200.0).text("Particle Lifetime")).changed() {
                            self.model.particle_lifetime = particle_lifetime;
                        }

                        ui.add_space(10.0);

                        // Particle Size Controls
                        ui.heading("Particle Size");
                        let mut min_size = self.model.particle_min_weight;
                        let mut max_size = self.model.particle_max_weight;
                        
                        // Min size slider - clamped by max size
                        if ui.add(egui::Slider::new(&mut min_size, 0.1..=max_size).text("Min Size")).changed() {
                            self.model.particle_min_weight = min_size;
                        }
                        
                        // Max size slider - clamped by min size
                        if ui.add(egui::Slider::new(&mut max_size, min_size..=100.0).text("Max Size")).changed() {
                            self.model.particle_max_weight = max_size;
                        }

                        ui.add_space(10.0);

                        let mut particle_step_length = self.model.particle_step_length;
                        if ui.add(egui::Slider::new(&mut particle_step_length, 0.0001..=1.0).text("Particle Speed")).changed() {
                            self.model.particle_step_length = particle_step_length;
                        }

                        ui.add_space(10.0);

                        // Background Selection
                        let current_background = self.model.background;
                        egui::ComboBox::from_label("Background")
                            .selected_text(format!("{}", current_background))
                            .show_ui(ui, |ui| {
                                let backgrounds = ["Black", "White", "Vector Field"];
                                for bg in backgrounds {
                                    if ui.selectable_label(
                                        format!("{}", current_background) == bg,
                                        bg,
                                    ).clicked() {
                                        self.model.background = match bg {
                                            "Black" => crate::model::enums::Background::Black,
                                            "White" => crate::model::enums::Background::White,
                                            "Vector Field" => crate::model::enums::Background::Vectors,
                                            _ => current_background,
                                        };
                                        self.model.redraw_background = RedrawBackground::Pending;
                                    }
                                }
                            });

                        ui.add_space(10.0);

                        // LUT Selection
                        let available_luts = self.model.lut_manager.get_available_luts();
                        let current_lut = self.model.current_lut.clone();
                        
                        // Use a fixed width that should accommodate most LUT names
                        let preview_width = 150.0;
                        let text_width = 200.0; // Fixed width for text
                        let padding = 20.0;
                        let total_width = preview_width + text_width + padding;
                        
                        egui::ComboBox::from_label("Color LUT")
                            .selected_text(&current_lut)
                            .width(total_width)
                            .show_ui(ui, |ui| {
                                for lut_name in available_luts {
                                    let is_selected = lut_name == current_lut;
                                    let response = ui.selectable_label(is_selected, "");
                                    
                                    // Draw the preview and label in the response rect
                                    let rect = response.rect;
                                    let painter = ui.painter();
                                    
                                    // Create a color strip preview
                                    let lut_data = self.model.lut_manager.load_lut(&lut_name).unwrap();
                                    let preview_height = 20.0;
                                    
                                    // Draw the color strip
                                    for i in 0..256 {
                                        let x = rect.min.x + (i as f32 / 255.0) * preview_width;
                                        let color = egui::Color32::from_rgb(
                                            lut_data.red[i],
                                            lut_data.green[i],
                                            lut_data.blue[i]
                                        );
                                        painter.rect_filled(
                                            egui::Rect::from_min_size(
                                                egui::pos2(x, rect.min.y),
                                                egui::vec2(preview_width / 255.0, preview_height)
                                            ),
                                            0.0,
                                            color
                                        );
                                    }
                                    
                                    // Draw the LUT name
                                    painter.text(
                                        egui::pos2(rect.min.x + preview_width + 10.0, rect.min.y + preview_height / 2.0),
                                        egui::Align2::LEFT_CENTER,
                                        &lut_name,
                                        egui::FontId::default(),
                                        if is_selected { egui::Color32::WHITE } else { egui::Color32::GRAY }
                                    );
                                    
                                    if response.clicked() {
                                        self.model.current_lut = lut_name;
                                    }
                                }
                            });

                        ui.add_space(10.0);

                        // Particle Shape Selection
                        let current_shape = self.model.particle_shape;
                        egui::ComboBox::from_label("Particle Shape")
                            .selected_text(format!("{}", current_shape))
                            .show_ui(ui, |ui| {
                                let shapes = [
                                    "Circle", "Square", "Triangle", "Star", "Diamond"
                                ];
                                for shape in shapes {
                                    if ui.selectable_label(
                                        format!("{}", current_shape) == shape,
                                        shape,
                                    ).clicked() {
                                        self.model.particle_shape = match shape {
                                            "Circle" => crate::model::enums::ParticleShape::Circle,
                                            "Square" => crate::model::enums::ParticleShape::Square,
                                            "Triangle" => crate::model::enums::ParticleShape::Triangle,
                                            "Star" => crate::model::enums::ParticleShape::Star,
                                            "Diamond" => crate::model::enums::ParticleShape::Diamond,
                                            _ => current_shape,
                                        };
                                    }
                                }
                            });

                        ui.add_space(10.0);

                        if ui.button(format!("Kill {} Particles", self.model.flow_particles.len()))
                            .clicked()
                        {
                            self.model.flow_particles = Vec::with_capacity(DEFAULT_AUTO_SPAWN_PARTICLE_COUNT_LIMIT);
                        }

                        ui.add_space(10.0);
                        ui.separator();
                        ui.heading("Quick Actions");

                        if ui.button(if self.is_paused { "Resume" } else { "Pause" }).clicked() {
                            self.is_paused = !self.is_paused;
                        }

                        if ui.button("Clear Screen").clicked() {
                            self.should_clear_screen = true;
                        }

                        if ui.button("New Random Seed").clicked() {
                            self.model.noise_seed = rand::random();
                            self.model.new_flow_vector_fn = self.model.flow_vector_field_builder_type.as_fn();
                            self.model.regen_flow_vectors();
                        }

                        let auto_spawn = self.model.automatically_spawn_particles;
                        if ui.checkbox(&mut self.model.automatically_spawn_particles, "Auto Spawn Particles").changed() {
                            self.model.automatically_spawn_particles = !auto_spawn;
                        }

                        ui.add_space(20.0);
                        ui.heading("Controls");
                        ui.label("Left Click  - Spawn a new particle where you clicked");
                        ui.label("Right Click - \"Draw\" new particles where you click and drag");
                        ui.label("Space       - Spawn new particle in a random location");
                        ui.label("C           - Clear screen and change background");

                        ui.label("/           - Show/hide this UI");
                    });
            }
        });
        self.egui_state.handle_platform_output(window, full_output.platform_output);
        self.egui_shapes = full_output.shapes;
        self.egui_textures_delta = full_output.textures_delta;
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Render particles with trail accumulation
        let should_clear = self.should_clear_screen || self.model.redraw_background == crate::model::enums::RedrawBackground::Pending;
        self.renderer.render(
            &self.queue,
            &mut encoder,
            &view,
            self.model.background,
            &self.model.flow_vectors,
            &self.model.flow_particles,
            (self.size.width, self.size.height),
            should_clear,
        );
        
        // After first frame, don't clear to create trails
        if self.should_clear_screen {
            self.should_clear_screen = false;
        }

        // EGUI render pass - render UI on top of particles without clearing
        {
            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [self.size.width, self.size.height],
                pixels_per_point: self.window.scale_factor() as f32,
            };
            let paint_jobs = self.egui_ctx.tessellate(std::mem::take(&mut self.egui_shapes), self.egui_ctx.pixels_per_point());
            
            // Handle texture updates for EGUI
            for (id, image_delta) in &self.egui_textures_delta.set {
                self.egui_renderer.update_texture(&self.device, &self.queue, *id, image_delta);
            }
            
            // Update EGUI buffers
            self.egui_renderer.update_buffers(
                &self.device,
                &self.queue,
                &mut encoder,
                &paint_jobs,
                &screen_descriptor,
            );
            
            // Create EGUI render pass that preserves background
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("EGUI Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load, // Don't clear - preserve particles underneath
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                
                // Render EGUI with alpha blending
                self.egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
            }
            
            // Free textures that are no longer needed (after render pass is dropped)
            for id in &self.egui_textures_delta.free {
                self.egui_renderer.free_texture(id);
            }
            
            // Clear texture delta for next frame
            self.egui_textures_delta.clear();
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    info!("Starting up the flow field...");

    let event_loop = EventLoop::new().unwrap();
    
    // Get the primary monitor's size
    let monitor = event_loop.primary_monitor().unwrap();
    let monitor_size = monitor.size();
    
    let window = Arc::new(WindowBuilder::new()
        .with_title("Flow Field")
        .with_inner_size(monitor_size)
        .with_maximized(true)
        .build(&event_loop)
        .unwrap());

    let mut app = pollster::block_on(App::new(window.clone(), &event_loop));

    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent { ref event, .. } => {
                if !app.input(&window, event) {
                    match event {
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::Resized(physical_size) => {
                            app.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            app.resize(app.size);
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            app.handle_keyboard_input(event);
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            app.handle_mouse_input(*button, *state);
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            app.handle_mouse_moved(*position);
                        }
                        WindowEvent::RedrawRequested => {
                            app.update();
                            app.render_ui(&window);
                            match app.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => app.resize(app.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                app.window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
