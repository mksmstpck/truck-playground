use eframe::egui;
use eframe::wgpu;

// Import RenderState properly
use eframe::egui_wgpu::RenderState;

pub struct CadApp {
    renderer: crate::renderer::Renderer,
    render_texture: Option<RenderTexture>,
}

struct RenderTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    egui_texture_id: egui::TextureId,
    size: (u32, u32),
}

impl CadApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let wgpu_state = cc.wgpu_render_state.as_ref().expect("wgpu required");

        let renderer =
            crate::renderer::Renderer::new(&wgpu_state.device, wgpu_state.target_format, 800, 600);

        // Load test geometry
        let solid = crate::geometry::create_test_solid();
        let mesh = crate::renderer::mesh::GpuMesh::from_solid(&solid, 0.0001);
        let mut renderer = renderer;
        renderer.set_mesh(&wgpu_state.device, &mesh);

        Self {
            renderer,
            render_texture: None,
        }
    }

    fn ensure_render_texture(&mut self, wgpu_state: &RenderState, width: u32, height: u32) {
        let needs_recreate = match &self.render_texture {
            None => true,
            Some(rt) => rt.size != (width, height),
        };

        if needs_recreate && width > 0 && height > 0 {
            // Remove old texture from egui
            if let Some(old) = self.render_texture.take() {
                wgpu_state
                    .renderer
                    .write()
                    .free_texture(&old.egui_texture_id);
            }

            // Create new texture
            let texture = wgpu_state.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Viewport Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu_state.target_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Register with egui for display
            let egui_texture_id = wgpu_state.renderer.write().register_native_texture(
                &wgpu_state.device,
                &view,
                wgpu::FilterMode::Linear,
            );

            // Update depth texture in our renderer
            self.renderer.resize(&wgpu_state.device, width, height);

            self.render_texture = Some(RenderTexture {
                texture,
                view,
                egui_texture_id,
                size: (width, height),
            });
        }
    }
}

impl eframe::App for CadApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Get wgpu state from frame
        let wgpu_state = frame.wgpu_render_state().expect("wgpu required");

        // Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.label("CAD Viewer - Drag to rotate, scroll to zoom");
        });

        // 3D viewport
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE) // Use NONE instead of none()
            .show(ctx, |ui| {
                let available = ui.available_size();
                let width = available.x as u32;
                let height = available.y as u32;

                // Ensure render texture exists and is correct size
                self.ensure_render_texture(wgpu_state, width, height);

                // Handle input
                let (rect, response) =
                    ui.allocate_exact_size(available, egui::Sense::click_and_drag());

                if response.dragged() {
                    let delta = response.drag_delta();
                    self.renderer.camera.orbit(delta.x, delta.y);
                }

                if response.hovered() {
                    let scroll = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll != 0.0 {
                        self.renderer.camera.zoom(scroll * 0.01);
                    }
                }

                // Render to our texture
                if let Some(rt) = &self.render_texture {
                    let mut encoder =
                        wgpu_state
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("CAD Encoder"),
                            });

                    self.renderer
                        .render(&mut encoder, &rt.view, &wgpu_state.queue, width, height);

                    wgpu_state.queue.submit(std::iter::once(encoder.finish()));

                    // Display texture
                    ui.painter().image(
                        rt.egui_texture_id,
                        rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
            });

        ctx.request_repaint();
    }
}
