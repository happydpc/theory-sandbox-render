use super::num::Zero;
use super::vector::*;
use super::vector3::Vector3;
use super::vector4::Vector4;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct Matrix4 {
  v: [f32; 4 * 4],
}

impl Matrix4 {
  pub fn new(array: [f32; 4 * 4]) -> Matrix4 {
    // row-major
    Matrix4 { v: array }
  }

  pub fn unit() -> Matrix4 {
    Matrix4::new([
      1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
  }

  pub fn translate(v: Vector3) -> Matrix4 {
    Matrix4::new([
      1.0, 0.0, 0.0, v.x, 0.0, 1.0, 0.0, v.y, 0.0, 0.0, 1.0, v.z, 0.0, 0.0, 0.0, 1.0,
    ])
  }

  pub fn scale(v: Vector3) -> Matrix4 {
    Matrix4::new([
      v.x, 0.0, 0.0, 0.0, 0.0, v.y, 0.0, 0.0, 0.0, 0.0, v.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
  }

  pub fn axis_angle(a: Vector3, t: f32) -> Matrix4 {
    // ロドリゲスの回転公式 (Rodrigues' rotation formula)
    let c = t.cos();
    let s = t.sin();
    Matrix4::new([
      c + a.x * a.x * (1.0 - c),
      a.x * a.y * (1.0 - c) - a.z * s,
      a.x * a.z * (1.0 - c) + a.y * s,
      0.0,
      a.y * a.x * (1.0 - c) + a.z * s,
      c + a.y * a.y * (1.0 - c),
      a.y * a.z * (1.0 - c) - a.x * s,
      0.0,
      a.z * a.x * (1.0 - c) - a.y * s,
      a.z * a.y * (1.0 - c) + a.x * s,
      c + a.z * a.z * (1.0 - c),
      0.0,
      0.0,
      0.0,
      0.0,
      1.0,
    ])
  }

  /**
   * z負方向がtargetを向くような基底を生成する
   */
  pub fn look_at(origin: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
    let za = -(target - origin).normalize();
    let xa = up.cross(za).normalize();
    let ya = za.cross(xa);
    &Matrix4::translate(origin) * &[xa, ya, za].into()
  }

  pub fn map_col<F>(&self, f: F) -> Vector4
  where
    F: Fn(Vector4) -> f32,
  {
    let mut out = [0f32; 4];
    for (i, o) in out.iter_mut().enumerate() {
      let v = Vector4::new(
        self.v[i * 4 + 0],
        self.v[i * 4 + 1],
        self.v[i * 4 + 2],
        self.v[i * 4 + 3],
      );
      *o = f(v)
    }
    out.into()
  }

  pub fn transpose(&self) -> Matrix4 {
    let mut out = Matrix4::zero();
    for (i, o) in out.v.iter_mut().enumerate() {
      let x = i % 4;
      let y = i / 4;
      *o = self.v[x * 4 + y]
    }
    out
  }
}

impl Zero for Matrix4 {
  fn zero() -> Matrix4 {
    Matrix4::new([0.0; 4 * 4])
  }
}

impl<'a> Neg for &'a Matrix4 {
  type Output = Matrix4;

  fn neg(self) -> Matrix4 {
    let mut out = Matrix4::zero();
    for (v, o) in self.v.iter().zip(out.v.iter_mut()) {
      *o = -v
    }
    out
  }
}

impl<'a> Add for &'a Matrix4 {
  type Output = Matrix4;

  fn add(self, rhs: &Matrix4) -> Matrix4 {
    let mut out = Matrix4::zero();
    for ((r, l), o) in self.v.iter().zip(rhs.v.iter()).zip(out.v.iter_mut()) {
      *o = r + l
    }
    out
  }
}

impl<'a> Sub for &'a Matrix4 {
  type Output = Matrix4;

  fn sub(self, rhs: &Matrix4) -> Matrix4 {
    let mut out = Matrix4::zero();
    for ((r, l), o) in self.v.iter().zip(rhs.v.iter()).zip(out.v.iter_mut()) {
      *o = r - l
    }
    out
  }
}

impl<'a> Mul for &'a Matrix4 {
  type Output = Matrix4;

  fn mul(self, rhs: &Matrix4) -> Matrix4 {
    let mut out = Matrix4::zero();
    for (i, o) in out.v.iter_mut().enumerate() {
      let x = i % 4;
      let y = i / 4;
      *o = (0..4).map(|j| self.v[y * 4 + j] * rhs.v[j * 4 + x]).sum()
    }
    out
  }
}

impl<'a> Mul<Vector3> for &'a Matrix4 {
  type Output = Vector3;

  fn mul(self, rhs: Vector3) -> Vector3 {
    let rhs_homo: Vector4 = rhs.into();
    let out = self.map_col(|row| row.dot(rhs_homo));
    out.into()
  }
}

impl From<[Vector3; 3]> for Matrix4 {
  fn from(v: [Vector3; 3]) -> Self {
    Matrix4::new([
      v[0].x, v[1].x, v[2].x, 0.0, v[0].y, v[1].y, v[2].y, 0.0, v[0].z, v[1].z, v[2].z, 0.0, 0.0,
      0.0, 0.0, 1.0,
    ])
  }
}

impl fmt::Display for Matrix4 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "/ {:>12} {:>12} {:>12} {:>12} \\\n| {:>12} {:>12} {:>12} {:>12} |\n| {:>12} {:>12} {:>12} {:>12} |\n\\ {:>12} {:>12} {:>12} {:>12} /",
      self.v[0],
      self.v[1],
      self.v[2],
      self.v[3],
      self.v[4],
      self.v[5],
      self.v[6],
      self.v[7],
      self.v[8],
      self.v[9],
      self.v[10],
      self.v[11],
      self.v[12],
      self.v[13],
      self.v[14],
      self.v[15]
    )
  }
}
