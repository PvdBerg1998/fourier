use crate::math::*;
use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window};
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Timer};

const STARTING_N: isize = 20;
const TIME_STEPS: usize = 500;
const LINE_WIDTH: u16 = 2;

pub struct Visualizer {
    mesh: Mesh,
    n: isize,
}

impl Visualizer {
    fn recalculate(&mut self, width: f32, height: f32) {
        let coefficients = (-self.n..self.n)
            .map(|n| (n, calculate_fourier_coefficient(step, n)))
            .collect::<Vec<_>>();

        let normalize_pos = move |pos: Point| {
            // x: [0, 1] => [0, width]
            // y: [-1, +1] => [0, height]
            // * 0.6 scaling + 0.2*2 centering padding
            Point::new(
                (pos.x * width) * 0.6 + width * 0.2,
                (pos.y * height / -2.0 + height / 2.0) * 0.6 + height * 0.2,
            )
        };

        self.mesh = Mesh::new();
        let points = (0..TIME_STEPS)
            .map(|t| 1.0 / TIME_STEPS as f64 * t as f64)
            .map(|t| superposition(&coefficients, t))
            .map(|c| Point::new(c.re as f32, c.im as f32))
            .map(normalize_pos)
            .collect::<Vec<_>>();

        self.mesh
            .stroke(Shape::Polyline { points }, Color::WHITE, LINE_WIDTH);
    }
}

impl Game for Visualizer {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Visualizer> {
        let width = window.width();
        let height = window.height();
        Task::new(move || {
            let mut v = Visualizer {
                mesh: Mesh::new(),
                n: STARTING_N,
            };
            v.recalculate(width, height);
            v
        })
    }

    fn interact(&mut self, input: &mut Self::Input, window: &mut Window) {
        let width = window.width();
        let height = window.height();

        if input.was_key_released(KeyCode::Up) {
            self.n += 5;
            println!("Increased n to {}", self.n);
            self.recalculate(width, height);
        } else if input.was_key_released(KeyCode::Down) {
            if self.n > 5 {
                self.n -= 5;
                println!("Decreased n to {}", self.n);
                self.recalculate(width, height);
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let mut target = frame.as_target();
        self.mesh.draw(&mut target);
    }
}
