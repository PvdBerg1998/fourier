mod definitions;
mod fourier;

use crate::math::{self, Complex, Real};
use coffee::{
    graphics::*,
    input::{keyboard::KeyCode, KeyboardAndMouse},
    load::Task,
    Game, Timer,
};
use definitions::*;
use fourier::*;

pub struct Visualizer {
    fourier: Fourier,
    progress: Real,
    progress_increase: Real,
    drawing_completed: bool,
    zoom_factor: u16,
}

impl Game for Visualizer {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    const TICKS_PER_SECOND: u16 = UPS;

    fn load(_window: &Window) -> Task<Visualizer> {
        Task::succeed(|| Visualizer {
            fourier: Fourier::new(DEFAULT_FN, DEFAULT_N),
            progress: 0.0,
            progress_increase: DEFAULT_SPEED,
            drawing_completed: false,
            zoom_factor: 1,
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        // Change N
        if input.was_key_released(KeyCode::Up) {
            self.fourier.change_n(N_CHANGE);
        } else if input.was_key_released(KeyCode::Down) {
            self.fourier.change_n(-N_CHANGE);
        }

        // Zoom
        if input.was_key_released(KeyCode::Right) {
            self.zoom_factor *= 2;
        } else if input.was_key_released(KeyCode::Left) {
            if self.zoom_factor > 1 {
                self.zoom_factor /= 2;
            }
        }

        // Speed
        if input.was_key_released(KeyCode::PageUp) {
            self.progress_increase *= 2.0;
        } else if input.was_key_released(KeyCode::PageDown) {
            self.progress_increase /= 2.0;
        }

        // Function
        if input.was_key_released(KeyCode::Space) {
            self.fourier.next_f();
            self.drawing_completed = false;
            self.progress = 0.0;
        }
    }

    fn update(&mut self, _window: &Window) {
        self.progress += self.progress_increase;
        if self.progress > 1.0 {
            self.drawing_completed = true;
            self.progress -= 1.0;
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let path_mesh = self.fourier.into_path_mesh(
            if self.drawing_completed {
                1.0
            } else {
                self.progress
            },
            1.0 / self.zoom_factor as f32,
        );

        let (last_vector, vector_meshes) = self
            .fourier
            .into_vector_meshes(self.progress, 1.0 / self.zoom_factor as f32);

        // Scale and translate the math output space to the frame
        // Many thanks to Héctor Ramón for this linear algebra
        let scale = self.zoom_factor as f32 * frame.width() / math::functions::FULL_SPACE as f32;
        let center_shift = Vector::new(frame.width() / 2.0, frame.height() / 2.0);
        let mut transform = Transformation::translate(center_shift)
            * Transformation::scale(scale)
            * Transformation::nonuniform_scale(1.0, -1.0);
        if self.zoom_factor > 1 {
            transform = transform * Transformation::translate(-last_vector);
        }

        frame.clear(BACKGROUND_COLOR);
        let mut target = frame.as_target();
        let mut target = target.transform(transform);

        path_mesh.draw(&mut target);
        for vector in vector_meshes {
            vector.draw(&mut target);
        }
    }
}

pub trait ComplexToNalgebra {
    fn into_point(self) -> Point;
    fn into_vector(self) -> Vector;
}

impl ComplexToNalgebra for Complex {
    fn into_point(self) -> Point {
        Point::new(self.re as f32, self.im as f32)
    }

    fn into_vector(self) -> Vector {
        Vector::new(self.re as f32, self.im as f32)
    }
}
