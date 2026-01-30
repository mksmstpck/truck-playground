mod app;
mod geometry;
mod renderer;
pub mod sketch;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("CAD Viewer"),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "CAD Viewer",
        options,
        Box::new(|cc| Ok(Box::new(app::CadApp::new(cc)))),
    )
}
