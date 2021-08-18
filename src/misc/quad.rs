const QUAD_EPS: f64 = 1E-8;

#[derive(Debug, Clone)]
pub(crate) struct QuadConfig<'a> {
    pub max_depth: u32,
    pub err_tol: f64,
    pub seed_points: Option<&'a Vec<f64>>,
}

impl<'a> Default for QuadConfig<'a> {
    fn default() -> Self {
        QuadConfig {
            max_depth: 12,
            err_tol: 1e-16,
            seed_points: None,
        }
    }
}

#[inline]
fn simpsons_rule<F>(
    func: &F,
    a: f64,
    fa: f64,
    b: f64,
    fb: f64,
) -> (f64, f64, f64)
where
    F: Fn(f64) -> f64,
{
    let c = (a + b) / 2.0;
    let h3 = (b - a).abs() / 6.0;
    let fc = func(c);
    (c, fc, h3 * (4.0_f64.mul_add(fc, fa) + fb))
}

#[allow(clippy::too_many_arguments)]
fn recursive_asr<F>(
    func: &F,
    a: f64,
    fa: f64,
    b: f64,
    fb: f64,
    eps: f64,
    whole: f64,
    c: f64,
    fc: f64,
) -> f64
where
    F: Fn(f64) -> f64,
{
    let (cl, fcl, left) = simpsons_rule(&func, a, fa, c, fc);
    let (cr, fcr, right) = simpsons_rule(&func, c, fc, b, fb);
    if (left + right - whole).abs() <= 15.0 * eps {
        left + right + (left + right - whole) / 15.0
    } else {
        recursive_asr(func, a, fa, c, fc, eps / 2.0, left, cl, fcl)
            + recursive_asr(func, c, fc, b, fb, eps / 2.0, right, cr, fcr)
    }
}

/// Adaptive Simpson's quadrature with user supplied error tolerance
///
/// # Example
///
/// Integrate f: x<sup>2</sup> over the interval [0, 1].
///
/// ```
/// use rv::misc::quad_eps;
///
/// let func = |x: f64| x.powi(2);
/// let q = quad_eps(func, 0.0, 1.0, Some(1E-10));
///
/// assert!((q - 1.0/3.0).abs() < 1E-10);
/// ```
pub fn quad_eps<F>(func: F, a: f64, b: f64, eps_opt: Option<f64>) -> f64
where
    F: Fn(f64) -> f64,
{
    let eps = eps_opt.unwrap_or(QUAD_EPS);
    let fa = func(a);
    let fb = func(b);
    let (c, fc, whole) = simpsons_rule(&func, a, fa, b, fb);
    recursive_asr(&func, a, fa, b, fb, eps, whole, c, fc)
}

/// Adaptive Simpson's quadrature
///
/// # Example
///
/// Integrate f: x<sup>2</sup> over the interval [0, 1].
///
/// ```
/// use rv::misc::quad;
///
/// let func = |x: f64| x.powi(2);
/// let q = quad(func, 0.0, 1.0);
///
/// assert!((q - 1.0/3.0).abs() < 1E-8);
/// ```
pub fn quad<F>(func: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    quad_eps(func, a, b, None)
}

//------------------------------------------
#[inline]
fn try_simpsons_rule<F, E>(
    func: &F,
    a: f64,
    fa: f64,
    b: f64,
    fb: f64,
) -> Result<(f64, f64, f64), E>
where
    F: Fn(f64) -> Result<f64, E>,
{
    let c = (a + b) / 2.0;
    let fc = func(c)?;
    let h3 = (b - a).abs() / 6.0;
    Ok((c, fc, h3 * (4.0_f64.mul_add(fc, fa) + fb)))
}

#[allow(clippy::too_many_arguments)]
fn try_recursive_asr<F, E>(
    func: &F,
    a: f64,
    fa: f64,
    b: f64,
    fb: f64,
    eps: f64,
    whole: f64,
    c: f64,
    fc: f64,
) -> Result<f64, E>
where
    F: Fn(f64) -> Result<f64, E>,
{
    let (cl, fcl, left) = try_simpsons_rule(&func, a, fa, c, fc)?;
    let (cr, fcr, right) = try_simpsons_rule(&func, c, fc, b, fb)?;
    if (left + right - whole).abs() <= 15.0 * eps {
        Ok(left + right + (left + right - whole) / 15.0)
    } else {
        try_recursive_asr(func, a, fa, c, fc, eps / 2.0, left, cl, fcl)
            .and_then(|left| {
                try_recursive_asr(func, c, fc, b, fb, eps / 2.0, right, cr, fcr)
                    .map(|right| left + right)
            })
    }
}

/// Adaptive Simpson's quadrature with user supplied error tolerance over
/// functions that can fail.
///
/// # Example
///
/// Integrate f: x<sup>2</sup> over the interval [0, 1].
///
/// ```
/// use rv::misc::try_quad_eps;
///
/// let func = |x: f64| {
///     if x > 2.0 {
///         Err(String::from("> 2.0"))
///     } else {
///         Ok(x.powi(2))
///     }
/// };
/// let q = try_quad_eps(func, 0.0, 1.0, Some(1E-10)).unwrap();
///
/// assert!((q - 1.0/3.0).abs() < 1E-10);
/// ```
pub fn try_quad_eps<F, E>(
    func: F,
    a: f64,
    b: f64,
    eps_opt: Option<f64>,
) -> Result<f64, E>
where
    F: Fn(f64) -> Result<f64, E>,
{
    let eps = eps_opt.unwrap_or(QUAD_EPS);
    let fa: f64 = func(a)?;
    let fb: f64 = func(b)?;
    let (c, fc, whole) = try_simpsons_rule(&func, a, fa, b, fb)?;
    try_recursive_asr(&func, a, fa, b, fb, eps, whole, c, fc)
}

/// Adaptive Simpson's quadrature on functions that can fail.
///
/// # Example
///
/// Integrate f: x<sup>2</sup> over the interval [0, 1].
///
/// ```
/// use rv::misc::try_quad;
///
/// let func = |x: f64| {
///     if x > 2.0 {
///         Err(String::from("> 2.0"))
///     } else {
///         Ok(x.powi(2))
///     }
/// };
/// let q = try_quad(func, 0.0, 1.0).unwrap();
///
/// assert!((q - 1.0/3.0).abs() < 1E-8);
/// ```
///
/// Errors if the function to evaluate returns an error
///
/// ```
/// use rv::misc::try_quad;
///
/// let func = |x: f64| {
///     if x > 0.5 {
///         Err(String::from("whoops"))
///     } else {
///         Ok(x.powi(2))
///     }
/// };
/// let q = try_quad(func, 0.0, 1.0);
///
/// assert!(q.is_err());
/// ```
pub fn try_quad<F, E>(func: F, a: f64, b: f64) -> Result<f64, E>
where
    F: Fn(f64) -> Result<f64, E>,
{
    try_quad_eps(func, a, b, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn quad_of_x2() {
        let func = |x: f64| x * x;
        let q = quad(func, 0.0, 1.0);
        assert::close(q, 1.0 / 3.0, QUAD_EPS);
    }

    #[test]
    fn quad_of_sin() {
        let func = |x: f64| x.sin();
        let q = quad(func, 0.0, 5.0 * PI);
        assert::close(q, 2.0, QUAD_EPS);
    }

    #[test]
    fn try_quad_of_x2() {
        fn func(x: f64) -> Result<f64, u8> {
            Ok(x * x)
        }
        let q = try_quad(func, 0.0, 1.0).unwrap();
        assert::close(q, 1.0 / 3.0, QUAD_EPS);
    }

    #[test]
    fn try_quad_of_sin() {
        fn func(x: f64) -> Result<f64, u8> {
            Ok(x.sin())
        }
        let q = try_quad(func, 0.0, 5.0 * PI).unwrap();
        assert::close(q, 2.0, QUAD_EPS);
    }
}
