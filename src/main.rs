pub mod math;
pub mod visualize;

use coffee::{graphics::WindowSettings, Game};

fn main() {
    visualize::Visualizer::run(WindowSettings {
        title: String::from("Fourier coefficients"),
        size: (1024, 1024),
        resizable: true,
        fullscreen: false,
    })
    .unwrap();
}
