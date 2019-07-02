pub mod math;
pub mod visualize;

use coffee::graphics::WindowSettings;
use coffee::Game;

fn main() {
    visualize::Visualizer::run(WindowSettings {
        title: String::from("Fourier coefficients"),
        size: (1024, 1024),
        resizable: false,
        fullscreen: false,
    })
    .unwrap();
}
