use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use alga::general::ClosedNeg;
use nalgebra::{Point2, Scalar, Vector2};
use num_traits::{FromPrimitive, NumCast, ToPrimitive, Zero};
use num_traits::cast;
use rand::distributions::uniform::SampleUniform;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn debug_log(s: &str);
}

// TODO: Translation2 instead of Vector2 (more specific)
#[derive(Clone, Debug, Hash)]
pub struct Node<Coord: Scalar> {
    position: Point2<Coord>,
    velocity: Vector2<Coord>,
    radius: Coord,
}

impl<Coord: Scalar> Node<Coord> {
    pub fn new(position: Point2<Coord>, velocity: Vector2<Coord>, radius: Coord) -> Self {
        Node {
            position,
            velocity,
            radius,
        }
    }

    pub fn random(
        rng: &mut (impl Rng + ?Sized),
        dimensions: &(Coord, Coord),
        min_max_v: &(Coord, Coord),
        min_max_radius: &(Coord, Coord),
    ) -> Self
        where
            Coord: Zero + SampleUniform,
    {
        let position = Point2::from([
            rng.gen_range(Coord::zero(), dimensions.0),
            rng.gen_range(Coord::zero(), dimensions.1),
        ]);
        let velocity = Vector2::from([rng.gen_range(min_max_v.0, min_max_v.1), rng.gen_range(min_max_v.0, min_max_v.1)]);
        let radius = rng.gen_range(min_max_radius.0, min_max_radius.1);

        Node::new(position, velocity, radius)
    }

    pub fn step(&mut self, dt: f64, dimensions: (Coord, Coord))
        where
            Coord: PartialOrd
            + Add<Coord, Output=Coord>
            + Sub<Coord, Output=Coord>
            + Mul<Coord, Output=Coord>
            + ClosedNeg + NumCast,
            Point2<Coord>: AddAssign<Vector2<Coord>>,
            Vector2<Coord>: Neg<Output=Vector2<Coord>>,
    {
        let x = self.x();
        if x <= self.radius || x > dimensions.0 - self.radius {
            self.velocity[0] = -self.velocity[0];
        }

        let y = self.y();
        if y <= self.radius || y > dimensions.1 - self.radius {
            self.velocity[1] = -self.velocity[1];
        }

        self.position += self
            .velocity
            .map(|coord| coord * cast::<f64, Coord>(dt).expect("Couldn't cast Coord type to u64"));
    }

    pub fn x(&self) -> Coord {
        self.position[0]
    }

    pub fn y(&self) -> Coord {
        self.position[1]
    }

    pub fn xy(&self) -> (Coord, Coord) {
        (self.x(), self.y())
    }
}

impl<Coord: Scalar + Clone> Node<Coord> {
    pub fn get_position(&self) -> Point2<Coord> {
        self.position.clone()
    }

    pub fn get_radius(&self) -> Coord {
        self.radius.clone()
    }
}

impl<Coord: Scalar> Node<Coord> {
    pub fn position(&self) -> &Point2<Coord> {
        &self.position
    }

    pub fn radius(&self) -> &Coord {
        &self.radius
    }
}

#[cfg(test)]
mod edge_case_tests {}

#[cfg(test)]
mod behavior_tests {}
