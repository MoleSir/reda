// Copyright (c) 2018-2020 Thomas Kramer.
// SPDX-FileCopyrightText: 2018-2022 Thomas Kramer
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Data structures and functions for 3x3 matrices.

use crate::traits::CoordType;

/// 3x3 matrix of the form.
/// [[ m11, m12, m13 ],
///  [ m21, m22, m23 ],
///  [ m31, m32, m33 ]]
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Matrix3<T: CoordType> {
    pub m11: T,
    pub m12: T,
    pub m13: T,
    pub m21: T,
    pub m22: T,
    pub m23: T,
    pub m31: T,
    pub m32: T,
    pub m33: T,
}

impl<T> Matrix3<T>
where
    T: CoordType,
{
    /// Create a new 3x3 matrix with entries of the form:
    /// [[ m11, m12, m13 ],
    ///  [ m21, m22, m23 ],
    ///  [ m31, m32, m33 ]]
    pub fn new([[m11, m12, m13], [m21, m22, m23], [m31, m32, m33]]: [[T; 3]; 3]) -> Self {
        Matrix3 {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        }
    }

    /// Create a new 3x3 matrix with entries of the form:
    /// [[ m11, m12, m13 ],
    ///  [ m21, m22, m23 ],
    ///  [ m31, m32, m33 ]]
    fn new_from_flat([m11, m12, m13, m21, m22, m23, m31, m32, m33]: [T; 9]) -> Self {
        Matrix3 {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        }
    }

    pub fn identity() -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        Self::new_from_flat([_1, _0, _0, _0, _1, _0, _0, _0, _1])
    }

    pub fn scalar_mul(&self, rhs: T) -> Self {
        Matrix3::new_from_flat([
            self.m11 * rhs,
            self.m12 * rhs,
            self.m13 * rhs,
            self.m21 * rhs,
            self.m22 * rhs,
            self.m23 * rhs,
            self.m31 * rhs,
            self.m32 * rhs,
            self.m33 * rhs,
        ])
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Matrix3::new_from_flat([
            self.m11 + rhs.m11,
            self.m12 + rhs.m12,
            self.m13 + rhs.m13,
            self.m21 + rhs.m21,
            self.m22 + rhs.m22,
            self.m23 + rhs.m23,
            self.m31 + rhs.m31,
            self.m32 + rhs.m32,
            self.m33 + rhs.m33,
        ])
    }

    pub fn mul_vec(&self, rhs: &(T, T, T)) -> (T, T, T) {
        (
            self.m11 * rhs.0 + self.m12 * rhs.1 + self.m13 * rhs.2,
            self.m21 * rhs.0 + self.m22 * rhs.1 + self.m23 * rhs.2,
            self.m31 * rhs.0 + self.m32 * rhs.1 + self.m33 * rhs.2,
        )
    }

    pub fn mat_mul(&self, rhs: &Self) -> Self {
        let a = self;
        let b = rhs;
        let c11 = a.m11 * b.m11 + a.m12 * b.m21 + a.m13 * b.m31;
        let c12 = a.m11 * b.m12 + a.m12 * b.m22 + a.m13 * b.m32;
        let c13 = a.m11 * b.m13 + a.m12 * b.m23 + a.m13 * b.m33;
        let c21 = a.m21 * b.m11 + a.m22 * b.m21 + a.m23 * b.m31;
        let c22 = a.m21 * b.m12 + a.m22 * b.m22 + a.m23 * b.m32;
        let c23 = a.m21 * b.m13 + a.m22 * b.m23 + a.m23 * b.m33;
        let c31 = a.m31 * b.m11 + a.m32 * b.m21 + a.m33 * b.m31;
        let c32 = a.m31 * b.m12 + a.m32 * b.m22 + a.m33 * b.m32;
        let c33 = a.m31 * b.m13 + a.m32 * b.m23 + a.m33 * b.m33;
        Self::new_from_flat([c11, c12, c13, c21, c22, c23, c31, c32, c33])
    }

    pub fn transpose(&self) -> Self {
        Self::new_from_flat([
            self.m11, self.m21, self.m31, self.m12, self.m22, self.m32, self.m13, self.m23,
            self.m33,
        ])
    }

    pub fn is_identity(&self) -> bool {
        self == &Self::identity()
    }

    pub fn is_unitary(&self) -> bool {
        self.mat_mul(&self.transpose()).is_identity()
    }

    pub fn determinant(&self) -> T {
        let a = self;
        a.m11 * a.m22 * a.m33 - a.m11 * a.m23 * a.m32 - a.m12 * a.m21 * a.m33
            + a.m12 * a.m23 * a.m31
            + a.m13 * a.m21 * a.m32
            - a.m13 * a.m22 * a.m31
    }

    pub fn try_inverse(&self) -> Option<Self> {
        let a = self;
        let det = a.determinant();
        if !det.is_zero() {
            Some(Self::new_from_flat([
                (a.m22 * a.m33 - a.m23 * a.m32) / det,
                (a.m13 * a.m32 - a.m12 * a.m33) / det,
                (a.m12 * a.m23 - a.m13 * a.m22) / det,
                (a.m23 * a.m31 - a.m21 * a.m33) / det,
                (a.m11 * a.m33 - a.m13 * a.m31) / det,
                (a.m13 * a.m21 - a.m11 * a.m23) / det,
                (a.m21 * a.m32 - a.m22 * a.m31) / det,
                (a.m12 * a.m31 - a.m11 * a.m32) / det,
                (a.m11 * a.m22 - a.m12 * a.m21) / det,
            ]))
        } else {
            None
        }
    }
}

impl<T: CoordType> Default for Matrix3<T> {
    fn default() -> Self {
        Self::identity()
    }
}

#[test]
fn test_matrix_multiplication() {
    let id: Matrix3<i32> = Matrix3::identity();
    assert_eq!(id.mat_mul(&id), id);
}

#[test]
fn test_mul_vec() {
    let id: Matrix3<i32> = Matrix3::identity();
    let v = (1, 2, 3);
    assert_eq!(id.mul_vec(&v), v);
}

#[test]
fn test_inverse() {
    let m = Matrix3::new([[1.0, 4.0, 2.0], [1.0, 2.0, 4.0], [2.0, 0.0, 4.0]]);
    let i = m.try_inverse().unwrap();
    assert_eq!(m.mat_mul(&i), Matrix3::identity());
    assert_eq!(i.mat_mul(&m), Matrix3::identity());
}
