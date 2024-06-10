use eframe::run_native;
use egui::ViewportBuilder;
use crate::pom::Pom;

mod pom;

fn main() {
    let app = "Pom";
    let option = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("Pom")
            .with_resizable(false)
            .with_taskbar(true)
            .with_decorations(true)
            .with_inner_size([682.0, 782.0]),
        ..Default::default()
    };

    let _ = run_native(app, option, Box::new(|_cc| Box::new(Pom::new())));
}
