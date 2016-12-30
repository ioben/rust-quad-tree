use std::ops::{Div, Mul, Deref, DerefMut};
use ::tree::Locatable;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point([f32; ::D]);

impl Point {
    pub fn new(point: [f32; ::D]) -> Self {
        Point(point)
    }

    pub fn zero() -> Self {
        Self::from(0f32)
    }

    pub fn from(default: f32) -> Self {
        Point([default; ::D])
    }

    // pub fn iter(self) -> PointIterator {
        // PointIterator {
            // point: self,
            // pos: 0,
        // }
    // }
}

impl Deref for Point {
    type Target = [f32; ::D];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        let mut result = Point::zero();
        for i in 0..::D {
            result[i] = self[i] / rhs[i];
        }

        result
    }
}

impl<'a, 'b> Div<&'b Point> for &'a Point {
    type Output = Point;

    fn div(self, rhs: &'b Point) -> Self::Output {
        let mut result = Point::zero();
        for i in 0..::D {
            result[i] = self[i] / rhs[i];
        }

        result
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Point::zero();
        for i in 0..::D {
            result[i] = self[i] / rhs;
        }

        result
    }
}

impl<'a> Div<f32> for &'a Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Point::zero();
        for i in 0..::D {
            result[i] = self[i] / rhs;
        }

        result
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Point::zero();
        for i in 0..::D {
            result[i] = self[i] * rhs;
        }

        result
    }
}

impl Locatable for Point {
    fn position(&self) -> &Point {
        &self
    }
}

// struct PointIterator {
    // point: Point,
    // pos: usize,
// }

// impl Iterator for PointIterator {
    // type Item = f32;

    // fn next(&mut self) -> Option<Self::Item> {

    // }
// }

#[cfg(test)]
mod tests {
    use super::Point;

    // Point tests
    #[test]
    fn test_creating_point() {
        assert!(Point::zero()[0] == 0f32);
        assert!(Point::zero()[::D - 1] == 0f32);

        assert!(Point::zero() == Point::from(0f32));

        assert!(Point::from(10f32)[0] == 10f32);
        assert!(Point::from(10f32)[::D - 1] == 10f32);
    }

    #[test]
    fn test_deref_point() {
        let mut point = Point::from(10f32);

        assert!((*point)[0] == 10f32);
        (*point)[0] = 15f32;
        assert!((*point)[0] == 15f32);
    }

    #[test]
    fn test_dividing_point() {
        assert!(&Point::from(10f32) / &Point::from(5f32) == Point::from(2f32));
        assert!(&Point::from(10f32) / 5f32 == Point::from(2f32));
    }
}
