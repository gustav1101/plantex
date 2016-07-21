use std::fmt;
use world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS};
use super::{AxialType, DefaultFloat, Vector2f};
use math::cgmath::Zero;
use std::ops::{Add, Index, IndexMut, Mul};
use math::cgmath::prelude::Array;
use std::cmp;

/// A 2-dimensional vector in axial coordinates. See [here][hex-blog] for more
/// information.
///
/// [hex-blog]: http://www.redblobgames.com/grids/hexagons/#coordinates
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, packed)]
pub struct AxialVector {
    pub q: AxialType,
    pub r: AxialType,
}

// TODO: implement cgmath::Zero
// TODO: implement cgmath::Array
// TODO: implement cgmath::MatricSpace
// TODO: implement cgmath::VectorSpace
// TODO: implement cgmath::InnerSpace
// TODO: implement ops::{ ... }
// TODO: Add `unit_q()` and `unit_r()` (see `Vector2::unit_x()` for reference)
// For all of the above, see
// http://bjz.github.io/cgmath/cgmath/struct.Vector2.html
//

impl AxialVector {
    pub fn new(q: AxialType, r: AxialType) -> Self {
        AxialVector { q: q, r: r }
    }

    /// Returns the position of the hexagons center in the standard coordinate
    /// system using `world::{HEX_INNER_RADIUS, HEX_OUTER_RADIUS}`.
    pub fn to_real(&self) -> Vector2f {
        Vector2f {
            x: ((2 * self.q + self.r) as DefaultFloat) * HEX_INNER_RADIUS,
            y: (self.r as DefaultFloat) * (3.0 / 2.0) * HEX_OUTER_RADIUS,
        }
    }

    /// Returns the `s` component of corresponding cube coordinates. In cube
    /// coordinates 'q + r + s = 0', so saving `s` is redundant and can be
    /// calculated on the fly when needed.
    pub fn s(&self) -> AxialType {
        -self.q - self.r
    }
}

impl fmt::Debug for AxialVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.q)
            .field(&self.r)
            .finish()
    }
}


impl Zero for AxialVector {
    fn zero() -> AxialVector {
        AxialVector { q: 0, r: 0 }
    }

    fn is_zero(&self) -> bool {
        self.q == 0 && self.r == 0
    }
}

impl Add<AxialVector> for AxialVector {
    type Output = AxialVector;

    fn add(self, arg2: AxialVector) -> AxialVector {
        AxialVector {
            r: self.r + arg2.r,
            q: self.q + arg2.q,
        }
    }
}

impl Mul<AxialType> for AxialVector {
    type Output = AxialVector;

    fn mul(self, arg2: AxialType) -> AxialVector {
        AxialVector {
            r: self.r * arg2,
            q: self.q * arg2,
        }
    }
}

impl Array for AxialVector {
    type Element = AxialType;

    fn from_value(value: Self::Element) -> Self {
        AxialVector::new(value, value)
    }

    fn sum(self) -> Self::Element {
        self.q + self.r
    }

    fn product(self) -> Self::Element {
        self.q * self.r
    }

    fn min(self) -> Self::Element {
        cmp::min(self.q, self.r)
    }

    fn max(self) -> Self::Element {
        cmp::max(self.q, self.r)
    }
}

impl Index<usize> for AxialVector {
    type Output = AxialType;

    fn index(&self, index: usize) -> &Self::Output {
        let ret: &AxialType = match index {
            0 => &self.q,
            1 => &self.r,
            _ => panic!("Illegal Index Argument: was {:?}", index),
        };
        ret
    }
}

impl IndexMut<usize> for AxialVector {
    fn index_mut(&mut self, index: usize) -> &mut AxialType {
        match index {
            0 => &mut self.q,
            1 => &mut self.r,
            _ => panic!("Illegal Index Argument: was {:?}", index),
        }
    }
}
