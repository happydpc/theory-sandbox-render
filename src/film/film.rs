use super::tonemap::Tonemap;
use math::Vector3;
use rand::Rng;
use std::path::Path;
use util::*;
use RNG;

pub struct Film<T> {
  /**
   * Row-major order
   *
   * [(0 0), (1 0), ..., (w, 0)
   *  (0 1), (1 1), ..., (w, 1)
   * ...
   *  (0 h), (1 h), ..., (w, h)]
   */
  pub data: Vec<T>,
  pub width: usize,
  pub height: usize,
}

impl<T: Clone> Film<T> {
  pub fn new(fill: T, width: usize, height: usize) -> Film<T> {
    Film {
      data: vec![fill; height * width],
      width: width,
      height: height,
    }
  }

  /**
   * (0,1) ... (1,1)
   * ...         ...
   * (0,0) ... (1,0)
   */
  pub fn uv(&self) -> impl Fn(usize) -> (f32, f32) {
    let w = self.width;
    let h = self.height;
    move |index| {
      let x = index % w;
      // flip y
      let y = h - index / w - 1;

      RNG.with(|rng| {
        // super sampling
        let su = rng.borrow_mut().gen::<f32>();
        let sv = rng.borrow_mut().gen::<f32>();

        ((x as f32 + su) / w as f32, (y as f32 + sv) / h as f32)
      })
    }
  }

  pub fn get(&self, x: usize, y: usize) -> &T {
    &self.data[y * self.width + x]
  }

  pub fn aspect(&self) -> f32 {
    return self.width as f32 / self.height as f32;
  }
}

pub trait Validate {
  fn validate(&self);
}

impl Validate for Film<Vector3> {
  fn validate(&self) {
    debug_assert!(self.data.iter().find(|v| v.is_nan()) == None, "nan");
    debug_assert!(
      self.data.iter().find(|v| v.is_infinite()) == None,
      "infinite"
    );
  }
}

pub trait Save<T>: Format {
  type Output;

  fn save<M>(&Film<T>, path: &Path, tonemap: M)
  where
    M: Tonemap<Input = T, Output = Self::Output>;
}

pub trait Format {
  fn ext() -> &'static str;
}
