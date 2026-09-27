use crate::traits::CoordType;
use crate::base::Vec2;

/// A 2x2 matrix of the form:
/// [[ m11, m12 ],
///  [ m21, m22 ]]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Matrix2<T: CoordType> {
    pub(crate) m11: T,
    pub(crate) m12: T,
    pub(crate) m21: T,
    pub(crate) m22: T,
}

impl<T> Matrix2<T>
where
    T: CoordType,
{
    /// Create a new 2x2 matrix with entries of the form:
    /// ```txt
    /// [[ m11, m12 ],
    ///  [ m21, m22 ]]
    /// ```
    pub fn new(m11: T, m12: T, m21: T, m22: T) -> Self {
        Matrix2 { m11, m12, m21, m22 }
    }

    pub fn identity() -> Self {
        Self::new(T::one(), T::zero(), T::zero(), T::one())
    }

    pub fn scaler_mul(&self, rhs: T) -> Self {
        Matrix2::new(
            self.m11 * rhs,
            self.m12 * rhs,
            self.m21 * rhs,
            self.m22 * rhs,
        )
    }

    pub fn mul_vec(&self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(
            rhs.x * self.m11 + rhs.y * self.m12,
            rhs.x * self.m21 + rhs.y * self.m22,
        )
    }

    pub fn mat_mul(&self, rhs: &Self) -> Self {
        let a = self;
        let b = rhs;
        let c11 = a.m11 * b.m11 + a.m12 * b.m21;
        let c12 = a.m11 * b.m12 + a.m12 * b.m22;
        let c21 = a.m21 * b.m11 + a.m22 * b.m21;
        let c22 = a.m21 * b.m12 + a.m22 * b.m22;
        Self::new(c11, c12, c21, c22)
    }

    pub fn transpose(&self) -> Self {
        Self::new(self.m11, self.m21, self.m12, self.m22)
    }

    pub fn determinant(&self) -> T {
        self.m11 * self.m22 - self.m12 * self.m21
    }

    pub fn is_identity(&self) -> bool {
        self == &Self::identity()
    }

    pub fn is_unitary(&self) -> bool {
        self.mat_mul(&self.transpose()).is_identity()
    }

    pub fn try_inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if !det.is_zero() {
            let z = T::zero();
            Some(Self::new(
                self.m22 / det,
                z - self.m12 / det,
                z - self.m21 / det,
                self.m11 / det,
            ))
        } else {
            None
        }
    }
}

impl<T: CoordType> Default for Matrix2<T> {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod test {
    use crate::base::Matrix2;

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 4.0);
        let b = Matrix2::new(5.0, 6.0, 7.0, 8.0);
        let id = Matrix2::identity();
        assert_eq!(id.mat_mul(&id), id);
        assert_eq!(b.mat_mul(&id), b);
        assert_eq!(id.mat_mul(&b), b);
        assert_eq!(
            a.mat_mul(&b),
            Matrix2::new(19.0, 22.0, 15.0 + 28.0, 18.0 + 32.0)
        );
    }
    
    #[test]
    fn test_inverse() {
        let m = Matrix2::new(2.0, 1.0, 4.0, 8.0);
        let i = m.try_inverse().unwrap();
        assert_eq!(m.mat_mul(&i), Matrix2::identity());
        assert_eq!(i.mat_mul(&m), Matrix2::identity());
    }
    
}