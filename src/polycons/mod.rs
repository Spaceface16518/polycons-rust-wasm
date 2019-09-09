use std::iter::repeat;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use alga::general::ClosedNeg;
use nalgebra::{RealField, Scalar};
use num_traits::{CheckedSub, ToPrimitive, Zero, NumCast};
use rand::distributions::uniform::SampleUniform;
use rand::Rng;

use config::WorldConfig;
pub use line::Line;
pub use node::Node;

pub mod config;
pub mod line;
pub mod node;

pub struct World<Coord: Scalar> {
    nodes: Vec<Node<Coord>>,
    dimensions: (Coord, Coord),
    config: WorldConfig<Coord>,
}

impl<Coord: Scalar> World<Coord> {
    pub fn new(
        nodes: Vec<Node<Coord>>,
        dimensions: (Coord, Coord),
        config: WorldConfig<Coord>,
    ) -> Self {
        World {
            nodes,
            dimensions,
            config,
        }
    }

    pub fn random(
        rng: &mut (impl Rng + ?Sized),
        num_nodes: usize,
        dimensions: (Coord, Coord),
        config: WorldConfig<Coord>,
        min_max_v: (Coord, Coord),
        min_max_radius: (Coord, Coord),
    ) -> Self
        where
            Coord: Zero + SampleUniform,
    {
        let nodes: Vec<Node<Coord>> = (0..num_nodes)
            .into_iter()
            .map(|_| Node::random(rng, &dimensions, &min_max_v, &min_max_radius))
            .collect();
        World::new(nodes, dimensions, config)
    }

    pub fn step_nodes(&mut self, dt: f64)
        where
            Coord: Add<Coord, Output=Coord>
            + AddAssign<Coord>
            + PartialOrd
            + Mul<Coord, Output=Coord>
            + Sub<Coord, Output=Coord>
            + ClosedNeg
            + NumCast
    {
        let dimensions = self.dimensions;
        self.nodes.iter_mut().for_each(|n| n.step(dt, dimensions))
    }

    pub fn calculate_lines(&self) -> Vec<Line<Coord>>
        where
            Coord: Mul<Coord, Output=Coord>
            + Div<Coord, Output=Coord>
            + ToPrimitive
            + From<u8>
            + Zero
            + RealField
            + Sub<Coord, Output=Coord>,
    {
        self.nodes
            .iter()
            .flat_map(|i| self.nodes.iter().zip(repeat(i)))
            .filter_map(|(start, end)| {
                Line::try_new(
                    start,
                    end,
                    self.config.max_strength,
                    self.config.line_threshold,
                )
            })
            .collect()
    }

    pub fn nodes(&self) -> &Vec<Node<Coord>> {
        &self.nodes
    }
}
