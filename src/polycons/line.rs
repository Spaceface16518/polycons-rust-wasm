use std::convert::TryInto;
use std::num::NonZeroU8;
use std::ops::{Div, Mul, Sub};

use nalgebra::{distance_squared, Point2, RealField, Scalar, distance};
use num_traits::{CheckedSub, Zero};
use num_traits::{FromPrimitive, ToPrimitive};
use wasm_bindgen::UnwrapThrowExt;

use super::node::Node;

#[derive(Clone, Debug, Hash)]
pub struct Line<Coord: Scalar> {
    start: Point2<Coord>,
    end: Point2<Coord>,
    strength: NonZeroU8,
}

impl<Coord: Scalar> Line<Coord> {
    pub fn start(&self) -> &Point2<Coord> {
        &self.start
    }
    pub fn end(&self) -> &Point2<Coord> {
        &self.end
    }

    pub fn endpoints(&self) -> (&Point2<Coord>, &Point2<Coord>) {
        (&self.start(), &self.end())
    }
    pub fn get_strength(&self) -> NonZeroU8 {
        self.strength
    }
}

impl<Coord: Scalar> Line<Coord>
where
    Coord: Mul<Coord, Output = Coord>
        + Div<Coord, Output = Coord>
        + PartialOrd
        + ToPrimitive
        + From<u8>
        + Zero
        + Sub<Coord, Output = Coord>
        + RealField,
{
    /// `max` is the maximum strength of the line. Lower this to lower the rate of change of the alpha(?)
    ///
    /// `threshold` is the distance at which the line no longer exists. Lower this to make connections happen at shorter distances and vise versa.
    ///
    /// `strength_alpha` returns either a `Some(u8)` value which represents the alpha (from 0..256).
    /// (it is actually a `NonZeroU8` for optimization purposes, since an alpha of zero would return `None`)
    pub fn strength_alpha(strength: Coord, max: Coord, threshold: Coord) -> Option<NonZeroU8> {
        // Balance in between max and threshold (and cutoff below threshold)
        Some(max - (max * strength / threshold))
            // Return None if below threshold (nodes are too far apart)
            .and_then(|c| if c >= Coord::zero() { Some(c) } else { None })
            .map(|adjusted| {
                // Clamp below max
                if adjusted <= max {
                    adjusted
                } else {
                    max
                }
            })
            .map(|adjusted| {
                // FIXME: should this be 255 or 256
                // Apply to scale of 0..255
                adjusted * Coord::from(255u8) / max
            })
            .as_ref()
            .map(Coord::to_u8)
            .map(|r| r.expect_throw("Error casting Coord to u8"))
            .and_then(NonZeroU8::new)
    }

    pub fn try_new(
        start: &Node<Coord>,
        end: &Node<Coord>,
        max_strength: Coord,
        distance_threshold: Coord,
    ) -> Option<Self> {
        if start.position() == end.position() {
            return None;
        }
        let alpha = Line::strength_alpha(distance(start.position(), end.position()), max_strength, distance_threshold)?;
        Some(Line {
            start: start.get_position(),
            end: end.get_position(),
            strength: alpha,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use std::num::NonZeroU8;

    #[test]
    fn test_strength_alpha_halfway() {
        let strength = 50.0;
        let max = 100.0;
        let threshold = 100.0;

        let expected = NonZeroU8::new((255 / 2) as u8);

        let actual = Line::strength_alpha(strength, max, threshold);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_strength_alpha_upper_quarter() {
        let strength = 75.0;
        let max = 100.0;
        let threshold = 100.0;

        let expected = NonZeroU8::new((255 / 4) as u8);

        let actual = Line::strength_alpha(strength, max, threshold);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_strength_alpha_lower_quarter() {
        let strength = 25.0;
        let max = 100.0;
        let threshold = 100.0;

        let expected = NonZeroU8::new((255 * 3 / 4) as u8);

        let actual = Line::strength_alpha(strength, max, threshold);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_strength_alpha_upper_edge() {
        let strength = 100.0;
        let max = 100.0;
        let threshold = 100.0;

        let expected = None;

        let actual = Line::strength_alpha(strength, max, threshold);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_strength_alpha_lower_edge() {
        let strength = 0.0;
        let max = 100.0;
        let threshold = 100.0;

        let expected = NonZeroU8::new(255u8);

        let actual = Line::strength_alpha(strength, max, threshold);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_strength_alpha_above_threshold() {
        let strength = 150.0;
        let max = 100.0;
        let threshold = 100.0;

        let expected = None; // Should be above threshold

        let actual = Line::strength_alpha(strength, max, threshold);

        assert_eq!(actual, expected)
    }
}
