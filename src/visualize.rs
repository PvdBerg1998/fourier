use crate::math::*;
use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window};
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Timer};
use rayon::prelude::*;

const STARTING_N: isize = 36;
const N_CHANGE: isize = 1;
const TIME_STEPS: usize = 100;
const LINE_WIDTH: u16 = 2;

pub struct Visualizer {
    coefficients: Vec<(isize, Complex)>,
    mesh: Mesh,
    n: isize,
    progress: usize,
}

impl Visualizer {
    fn recalculate_coefficients(&mut self) {
        self.coefficients = (-self.n..=self.n)
            .into_par_iter()
            .map(|n| (n, calculate_fourier_coefficient(functions::step, n)))
            .collect::<Vec<_>>();
    }

    fn recalculate_mesh(&mut self, width: f32, height: f32) {
        let normalize_pos = move |pos: Point| {
            // x: [0, 1] => [0, width]
            // y: [0, 1] => [0, height]
            // * 0.6 scaling + 0.2*2 centering padding
            Point::new(
                (pos.x * width) * 0.6 + width * 0.2,
                (pos.y * height) * 0.6 + height * 0.2,
            )
        };

        self.mesh = Mesh::new();
        let points = (0..=self.progress)
            .into_par_iter()
            .map(|t| 1.0 / TIME_STEPS as f64 * t as f64)
            .map(|t| superposition(&self.coefficients, t))
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

    fn load(_window: &Window) -> Task<Visualizer> {
        Task::new(move || {
            let mut v = Visualizer {
                coefficients: Vec::new(),
                mesh: Mesh::new(),
                n: STARTING_N,
                progress: 0,
            };
            v.recalculate_coefficients();
            v
        })
    }

    fn interact(&mut self, input: &mut Self::Input, window: &mut Window) {
        if input.was_key_released(KeyCode::Up) {
            self.n += N_CHANGE;
            self.progress = 0;
            println!("Increased n to {}", self.n);
            self.recalculate_coefficients();
        } else if input.was_key_released(KeyCode::Down) {
            if self.n > N_CHANGE {
                self.n -= N_CHANGE;
                self.progress = 0;
                println!("Decreased n to {}", self.n);
                self.recalculate_coefficients();
            }
        }

        if self.progress < TIME_STEPS {
            self.progress += 1;
            self.recalculate_mesh(window.width(), window.height())
        }
    }

    fn update(&mut self, _window: &Window) {}

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let mut target = frame.as_target();
        self.mesh.draw(&mut target);
    }
}
