#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
  fn from(n: f64) -> Self {
    if n >= 1.0 {
      Q7(127)
    } else if n <= -1.0 {
      Q7(-128)
    } else {
      Q7((n * 128.0) as i8)
    }
  }
}

impl From<Q7> for f64 {
  fn from(n: Q7) -> f64 {
    (n.0 as f64) * 2_f64.powf(-7.0)
  }
}

impl From<f32> for Q7 {
  fn from(n: f32) -> Self {
    // Converting f32 to f64 is save because every bit can be represented
    Q7::from(n as f64)
  }
}

impl From<Q7> for f32 {
  fn from(n: Q7) -> f32 {
    // Converting from f64 to f32 may lead to loss of precision
    f64::from(n) as f32
  }
}

#[cfg(test)]
mod tests {
  // Defining sub module
  use super::*; // Bring parent modile to sub module scope
  #[test]
  fn out_of_bounds() {
    assert_eq!(Q7::from(10.0), Q7::from(1.0));
    assert_eq!(Q7::from(-10.0), Q7::from(-1.0));
  }

  #[test]
  fn f32_to_q7() {
    let n1:f32 = 0.1;
    let q1  = Q7::from(n1);

    let n2:f64 = -0.4;
    let q2 = Q7::from(n2);

    let n3:f64 = 123.0;
    let q3 = Q7::from(n3);

    assert_eq!(q1, Q7(89));
    assert_eq!(q2, Q7(-51));
    assert_eq!(q3, Q7(127)); 
  }
  
}
