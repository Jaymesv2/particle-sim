use std::ops::Mul;

pub struct Pos<T, const N: usize> {
    elems: [T; N],
}

impl<const N: usize> Vector<N> {}

impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Vector<N>;
    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in 0..N {
            self.elems[i] *= rhs;
        }
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize> {
    elems: [f32; N],
}

impl<const N: usize> Vector<N> {
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        let mut sum = 0.0;
        // I think this should loop unroll up to a certain point
        for i in 0..N {
            sum += self.elems[i] * self.elems[i];
        }
        sum
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }
}

impl Vector<1> {
    pub fn x(&self) -> f32 {
        self.elems[0]
    }
}

impl Vector<2> {
    pub fn x(&self) -> f32 {
        self.elems[0]
    }
    pub fn y(&self) -> f32 {
        self.elems[1]
    }
}

impl Vector<3> {
    pub fn x(&self) -> f32 {
        self.elems[0]
    }

    pub fn y(&self) -> f32 {
        self.elems[1]
    }

    pub fn z(&self) -> f32 {
        self.elems[2]
    }
}
