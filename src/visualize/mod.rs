mod color;

use crate::math;
use crate::math::{Complex, Real};
use coffee::graphics::*;
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Timer};
use rayon::prelude::*;

const STARTING_N: isize = 16;
const N_CHANGE: isize = 2;

const TIME_STEPS: usize = 1500;

const PATH_WIDTH: u16 = 4;
const VECTOR_WIDTH: u16 = 2;
const VECTOR_DOT_RADIUS: f32 = 3.0;

// @todo add function change button. maybe change math::functions to enum
pub struct Visualizer {
    f: Box<dyn Fn(Real) -> Complex + Send + Sync + 'static>,
    coefficients: Vec<(isize, Complex)>,
    n: isize,
    progress: usize,
    drawing_completed: bool,
    zoom_factor: u16,
}

fn progress_as_time(progress: usize) -> f64 {
    1.0 / TIME_STEPS as f64 * progress as f64
}

impl Visualizer {
    fn recalculate_coefficients(&mut self) {
        self.coefficients = (-self.n..=self.n)
            .into_par_iter()
            .map(|n| (n, math::calculate_fourier_coefficient(|t| (self.f)(t), n)))
            .collect::<Vec<_>>();
    }

    fn path_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();
        let coefficients = self.coefficients.as_slice();
        let progress = if self.drawing_completed {
            TIME_STEPS
        } else {
            self.progress
        };
        let points = (0..=progress)
            .into_par_iter()
            .map(progress_as_time)
            .map(|t| math::superposition(coefficients, t))
            .map(|c| Point::new(c.re as f32, c.im as f32))
            .collect::<Vec<_>>();
        mesh.stroke(Shape::Polyline { points }, color::PATH_COLOR, PATH_WIDTH);
        mesh
    }

    fn vector_meshes(&self) -> (Vector, Vec<Mesh>) {
        let time = progress_as_time(self.progress);
        let vectors = self
            .coefficients
            .iter()
            .map(|(n, coefficient)| math::eval_term(*coefficient, *n, time))
            .collect::<Vec<_>>();

        // Draw vector arrows, starting at the end of the last one
        let mut last_pos = Complex::new(0.0, 0.0);
        let meshes = vectors
            .into_iter()
            .map(|vector| {
                // Add current vector to the last position
                let next_pos = last_pos + vector;
                let mut mesh = Mesh::new();

                // Vector line
                mesh.stroke(
                    Shape::Polyline {
                        points: vec![last_pos.into_point(), next_pos.into_point()],
                    },
                    color::VECTOR_COLOR,
                    VECTOR_WIDTH,
                );

                // Vector dot @todo replace with actual arrows?
                mesh.fill(
                    Shape::Circle {
                        center: next_pos.into_point(),
                        radius: VECTOR_DOT_RADIUS,
                    },
                    color::VECTOR_COLOR,
                );

                // Vector path circle
                mesh.stroke(
                    Shape::Circle {
                        center: last_pos.into_point(),
                        radius: vector.norm() as f32,
                    },
                    color::VECTOR_CIRCLE_COLOR,
                    VECTOR_WIDTH,
                );

                // Continue the next vector at the tip of this one
                last_pos = next_pos;

                mesh
            })
            .collect();
        (last_pos.into_vector(), meshes)
    }
}

impl Game for Visualizer {
    const TICKS_PER_SECOND: u16 = 60;
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Visualizer> {
        Task::new(|| {
            let mut v = Visualizer {
                f: Box::new(math::functions::step),
                coefficients: Vec::new(),
                n: STARTING_N,
                progress: 0,
                drawing_completed: false,
                zoom_factor: 1,
            };
            v.recalculate_coefficients();
            v
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        // Change N
        if input.was_key_released(KeyCode::Up) {
            self.n += N_CHANGE;
            self.progress = 0;
            self.drawing_completed = false;
            println!("Increased n to {}", self.n);
            self.recalculate_coefficients();
        } else if input.was_key_released(KeyCode::Down) {
            if self.n > N_CHANGE {
                self.n -= N_CHANGE;
                self.progress = 0;
                self.drawing_completed = false;
                println!("Decreased n to {}", self.n);
                self.recalculate_coefficients();
            }
        }

        // Zoom
        if input.was_key_released(KeyCode::Right) {
            self.zoom_factor += 1;
        } else if input.was_key_released(KeyCode::Left) {
            if self.zoom_factor > 1 {
                self.zoom_factor -= 1;
            }
        }
    }

    fn update(&mut self, _window: &Window) {
        if self.progress < TIME_STEPS {
            self.progress += 1;
            if self.progress == TIME_STEPS {
                self.drawing_completed = true;
                self.progress = 0;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let path_mesh = self.path_mesh();
        let (last_vector, vector_meshes) = self.vector_meshes();

        let transform = if self.zoom_factor == 1 {
            Transformation::scale(frame.width() / math::functions::FULL_SPACE_SIZE as f32)
                * Transformation::translate(Vector::new(
                    math::functions::HALF_SPACE_SIZE as f32,
                    math::functions::HALF_SPACE_SIZE as f32,
                )) // * Transformation::nonuniform_scale(1.0, -1.0)
        } else {
            Transformation::scale(
                self.zoom_factor as f32 * frame.width() / math::functions::FULL_SPACE_SIZE as f32,
            ) * Transformation::translate(-last_vector)
                * Transformation::translate(Vector::new(
                    math::functions::HALF_SPACE_SIZE as f32,
                    math::functions::HALF_SPACE_SIZE as f32,
                )) // * Transformation::nonuniform_scale(1.0, -1.0)
        };

        frame.clear(color::BACKGROUND_COLOR);
        let mut target = frame.as_target();
        let mut target = target.transform(transform);

        path_mesh.draw(&mut target);
        for vector in vector_meshes {
            vector.draw(&mut target);
        }
    }
}

trait ComplexToCoffee {
    fn into_point(self) -> Point;
    fn into_vector(self) -> Vector;
}

impl ComplexToCoffee for Complex {
    fn into_point(self) -> Point {
        Point::new(self.re as f32, self.im as f32)
    }
    fn into_vector(self) -> Vector {
        Vector::new(self.re as f32, self.im as f32)
    }
}
