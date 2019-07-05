use super::{definitions::*, ComplexToNalgebra};
use crate::math::{self, Complex, Real};
use coffee::graphics::{Color, Mesh, Shape, Vector};
use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

const PATH_MESH_POINTS: usize = 5000;

pub struct Fourier {
    f: math::functions::Function,
    coefficient_cache: Vec<(isize, Complex)>,
    n: isize,
}

impl Fourier {
    pub fn new(f: math::functions::Function, n: isize) -> Self {
        let mut f = Fourier {
            f,
            coefficient_cache: vec![],
            n,
        };
        f.update_cache();
        f
    }

    fn update_cache(&mut self) {
        self.coefficient_cache = (-self.n..=self.n)
            .into_par_iter()
            .map(|n| {
                (
                    n,
                    math::calculate_fourier_coefficient(|t| self.f.execute(t), n),
                )
            })
            .collect::<Vec<_>>();
    }

    pub fn set_n(&mut self, n: isize) {
        self.n = n;
        self.update_cache();
    }

    pub fn change_n(&mut self, change: isize) {
        if change < 0 {
            if self.n > change.abs() {
                self.set_n(self.n - change.abs());
            }
        } else if change > 0 {
            self.set_n(self.n + change);
        }
    }

    pub fn set_f(&mut self, f: math::functions::Function) {
        self.f = f;
        self.update_cache();
    }

    pub fn next_f(&mut self) {
        self.set_f(self.f.next());
    }

    pub fn into_path_mesh(&mut self, progress: Real, line_scale_factor: f32) -> Mesh {
        let mut mesh = Mesh::new();
        let coefficients = self.coefficient_cache.as_slice();

        let target_point = (progress * PATH_MESH_POINTS as Real) as usize;
        let mut points = (0..=target_point)
            .into_par_iter()
            .map(|t| t as Real / PATH_MESH_POINTS as Real)
            .map(|t| math::superposition(coefficients, t))
            .map(ComplexToNalgebra::into_point)
            .collect::<Vec<_>>();
        points.push(math::superposition(coefficients, progress).into_point());

        mesh.stroke(
            Shape::Polyline { points },
            PATH_COLOR,
            PATH_WIDTH * line_scale_factor,
        );
        mesh
    }

    pub fn into_vector_meshes(
        &self,
        progress: Real,
        line_scale_factor: f32,
    ) -> (Vector, Vec<Mesh>) {
        let vectors = self
            .coefficient_cache
            .iter()
            .map(|(n, coefficient)| math::eval_term(*coefficient, *n, progress))
            .collect::<Vec<_>>();

        // Draw vector arrows, starting at the end of the last one
        let mut rng = SmallRng::seed_from_u64(0);
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
                    VECTOR_COLOR,
                    VECTOR_WIDTH * line_scale_factor,
                );

                // Vector head
                let head_bottom_left = Complex::new(
                    next_pos.re
                        + VECTOR_HEAD_LENGTH_FACTOR
                            * ((last_pos.re - next_pos.re) * VECTOR_HEAD_ANGLE.cos()
                                + (last_pos.im - next_pos.im) * VECTOR_HEAD_ANGLE.sin()),
                    next_pos.im
                        + VECTOR_HEAD_LENGTH_FACTOR
                            * ((last_pos.im - next_pos.im) * VECTOR_HEAD_ANGLE.cos()
                                - (last_pos.re - next_pos.re) * VECTOR_HEAD_ANGLE.sin()),
                );
                let head_bottom_right = Complex::new(
                    next_pos.re
                        + VECTOR_HEAD_LENGTH_FACTOR
                            * ((last_pos.re - next_pos.re) * VECTOR_HEAD_ANGLE.cos()
                                - (last_pos.im - next_pos.im) * VECTOR_HEAD_ANGLE.sin()),
                    next_pos.im
                        + VECTOR_HEAD_LENGTH_FACTOR
                            * ((last_pos.im - next_pos.im) * VECTOR_HEAD_ANGLE.cos()
                                + (last_pos.re - next_pos.re) * VECTOR_HEAD_ANGLE.sin()),
                );
                mesh.fill(
                    Shape::Polyline {
                        points: vec![
                            next_pos.into_point(),
                            head_bottom_left.into_point(),
                            head_bottom_right.into_point(),
                        ],
                    },
                    VECTOR_COLOR,
                );

                // Vector path circle
                // Determine random color for this circle
                // No this is not overkill
                let circle_color = Color {
                    r: Normal::new(VECTOR_CIRCLE_COLOR.r, VECTOR_CIRCLE_COLOR_STDDEV)
                        .unwrap()
                        .sample(&mut rng),
                    g: Normal::new(VECTOR_CIRCLE_COLOR.g, VECTOR_CIRCLE_COLOR_STDDEV)
                        .unwrap()
                        .sample(&mut rng),
                    b: Normal::new(VECTOR_CIRCLE_COLOR.b, VECTOR_CIRCLE_COLOR_STDDEV)
                        .unwrap()
                        .sample(&mut rng),
                    a: VECTOR_CIRCLE_COLOR.a,
                };
                mesh.stroke(
                    Shape::Circle {
                        center: last_pos.into_point(),
                        radius: vector.norm() as f32,
                    },
                    circle_color,
                    VECTOR_CIRCLE_WIDTH * line_scale_factor,
                );

                // Continue the next vector at the tip of this one
                last_pos = next_pos;
                mesh
            })
            .collect();

        (last_pos.into_vector(), meshes)
    }
}
