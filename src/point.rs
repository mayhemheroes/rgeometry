use num_traits::Num;
use num_traits::Zero;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use super::array::*;
use super::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point<T, const N: usize>(pub [T; N]);

// Random sampling.
impl<T, const N: usize> Distribution<Point<T, N>> for Standard
where
  Standard: Distribution<T>,
{
  // FIXME: Unify with code for Vector.
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point<T, N> {
    unsafe {
      let mut arr = MaybeUninit::uninit();
      for i in 0..N {
        (arr.as_mut_ptr() as *mut T).add(i).write(rng.gen());
      }
      Point(arr.assume_init())
    }
  }
}

// Methods on N-dimensional points.
impl<T: Clone, const N: usize> Point<T, N> {
  pub fn as_vector(&self) -> Vector<T, N> {
    Vector(self.clone().0)
  }
  pub fn cmp_distance_to(&self, p: &Point<T, N>, q: &Point<T, N>) -> Ordering
  where
    T: Zero + PartialOrd,
    for<'a> &'a T: Sub<&'a T, Output = T> + Mul<&'a T, Output = T>,
  {
    self
      .squared_euclidean_distance(p)
      .partial_cmp(&self.squared_euclidean_distance(q))
      .unwrap_or(Ordering::Equal)
  }
  pub fn squared_euclidean_distance(&self, rhs: &Point<T, N>) -> T
  where
    T: Zero,
    for<'a> &'a T: Sub<&'a T, Output = T> + Mul<&'a T, Output = T>,
  {
    self
      .0
      .iter()
      .zip(rhs.0.iter())
      .fold(T::zero(), |sum, (a, b)| {
        let diff = a - b;
        sum + &diff * &diff
      })
  }
}

// Methods on two-dimensional points.
impl<T> Point<T, 2> {
  pub fn turn(&self, q: &Point<T, 2>, r: &Point<T, 2>) -> Turn
  where
    T: Sub<T, Output = T> + Clone + Mul<T, Output = T> + PartialOrd,
    for<'a> &'a T: Sub<Output = T>,
  {
    raw_arr_turn(&self.0, &q.0, &r.0)
  }
  /// Docs?
  pub fn ccw_cmp_around(&self, p: &Point<T, 2>, q: &Point<T, 2>) -> Ordering
  where
    T: Num + Clone + PartialOrd,
    for<'a> &'a T: Neg<Output = T> + Sub<Output = T> + Mul<Output = T>,
  {
    self.ccw_cmp_around_with(&Vector([T::one(), T::zero()]), p, q)
  }
  pub fn ccw_cmp_around_with(&self, z: &Vector<T, 2>, p: &Point<T, 2>, q: &Point<T, 2>) -> Ordering
  where
    T: Num + Clone + PartialOrd,
    for<'a> &'a T: Sub<Output = T> + Mul<Output = T> + Neg<Output = T>,
  {
    ccw_cmp_around_origin_with(&z.0, &(p - self).0, &(q - self).0)
  }
}

mod add;
mod sub;
