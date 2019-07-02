use crate::math::*;
use coffee::graphics::{Canvas, Color, Frame, Gpu, Mesh, Point, Quad, Shape, Vector, Window};
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Timer};
use rayon::prelude::*;

const SCALE_FACTOR: f32 = 0.4;
const OFFSET_FACTOR: f32 = (1.0 - SCALE_FACTOR) / 2.0;

const STARTING_N: isize = 4;
const N_CHANGE: isize = 1;

const TIME_STEPS: usize = 300;

const LINE_WIDTH: u16 = 2;
const VECTOR_WIDTH: u16 = 1;

pub struct Visualizer {
    coefficients: Vec<(isize, Complex)>,
    canvas: Canvas,
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

    fn draw_canvas(&mut self, gpu: &mut Gpu) {
        /*
            Normalisation/scaling helper closures
        */
        let width = self.canvas.width() as f32;
        let height = self.canvas.height() as f32;
        let scale_pos = move |pos: Point| {
            // x: [0, 1] => [0, width]
            // y: [0, 1] => [0, height]
            // * SCALE_FACTOR scaling + 0.2*2 centering padding
            Point::new(
                pos.x * width * SCALE_FACTOR + width * OFFSET_FACTOR,
                pos.y * height * SCALE_FACTOR + height * OFFSET_FACTOR,
            )
        };
        let scale_vec = move |vec: Vector| {
            Vector::new(vec.x * width * SCALE_FACTOR, vec.y * height * SCALE_FACTOR)
        };
        let progress_to_time = move |progress: usize| 1.0 / TIME_STEPS as f64 * progress as f64;

        /*
            Prepare canvas
        */
        let mut target = self.canvas.as_target(gpu);
        target.clear(Color::BLACK);

        /*
            Render fourier mesh at all previously seen points
            @todo can be optimized by not clearing a canvas
        */
        let mut fourier_mesh = Mesh::new();
        let coefficients = self.coefficients.as_slice();
        let points = (0..=self.progress)
            .into_par_iter()
            .map(progress_to_time)
            .map(|t| superposition(coefficients, t))
            .map(|c| Point::new(c.re as f32, c.im as f32))
            .map(scale_pos)
            .collect::<Vec<_>>();
        fourier_mesh.stroke(Shape::Polyline { points }, Color::WHITE, LINE_WIDTH);
        fourier_mesh.draw(&mut target);

        /*
            Render individual vectors at their current positions
        */
        let t = progress_to_time(self.progress);
        let vectors = coefficients
            .iter()
            .map(|(n, coefficient)| eval_term(*coefficient, *n, t))
            .collect::<Vec<_>>();

        // Draw vector arrows, starting at the end of the last one
        let mut last_pos = Complex::new(0.0, 0.0);
        for vector in vectors {
            // Add current vector to the last position
            let new_pos = last_pos + vector;

            // Scale them
            let vector_norm = scale_vec(Vector::new(vector.re as f32, vector.im as f32)).norm();
            let last_pos_normalized = scale_pos(Point::new(last_pos.re as f32, last_pos.im as f32));
            let new_pos_normalized = scale_pos(Point::new(new_pos.re as f32, new_pos.im as f32));

            let mut mesh = Mesh::new();
            // Vector line
            mesh.stroke(
                Shape::Polyline {
                    points: vec![last_pos_normalized, new_pos_normalized],
                },
                Color::from_rgb(0, 255, 0),
                VECTOR_WIDTH,
            );
            // Vector path circle
            mesh.stroke(
                Shape::Circle {
                    center: last_pos_normalized,
                    radius: vector_norm,
                },
                Color::from_rgb(0, 0, 255),
                VECTOR_WIDTH,
            );
            mesh.draw(&mut target);

            // Continue the next vector at the tip of this one
            last_pos = new_pos;
        }
    }
}

impl Game for Visualizer {
    const TICKS_PER_SECOND: u16 = 20;
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Visualizer> {
        let width = window.width();
        let height = window.height();
        Task::using_gpu(move |gpu| {
            let mut v = Visualizer {
                coefficients: Vec::new(),
                canvas: Canvas::new(gpu, width as u16, height as u16)?,
                n: STARTING_N,
                progress: 0,
            };
            v.recalculate_coefficients();
            Ok(v)
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
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
    }

    fn update(&mut self, _window: &Window) {
        if self.progress < TIME_STEPS {
            self.progress += 1;
            if self.progress == TIME_STEPS {
                self.progress = 0;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::from_rgb(255, 0, 0));

        self.draw_canvas(frame.gpu());
        self.canvas.draw(
            Quad {
                size: (frame.width(), frame.height()),
                ..Quad::default()
            },
            &mut frame.as_target(),
        );
    }
}
