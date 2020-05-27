use arch_types::Features;
use num_complex::{Complex, ComplexDistribution};
use num_traits::Num;
use rand::distributions::Standard;
use rand::prelude::*;
use safe_simd::vector::{Handle, Signed, Vector};

#[inline]
fn unary_op_impl<T, D, V, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    mut vector: V,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    V: Vector<Scalar = T> + Signed<T>,
    VFunc: Fn(V) -> V,
    SFunc: Fn(T) -> T,
{
    let mut rng = rand::thread_rng();
    for x in vector.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(vector);
    for i in 0..V::WIDTH {
        assert_eq!(output[i], sfunc(vector[i]))
    }
}

#[inline]
fn binary_op_impl<T, D, V, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    (mut a, mut b): (V, V),
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    V: Vector<Scalar = T> + Signed<T>,
    VFunc: Fn(V, V) -> V,
    SFunc: Fn(T, T) -> T,
{
    let mut rng = rand::thread_rng();
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(a, b);
    for i in 0..V::WIDTH {
        assert_eq!(output[i], sfunc(a[i], b[i]))
    }
}

#[inline]
fn binary_scalar_op_impl<T, D, V, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    mut a: V,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    V: Vector<Scalar = T> + Signed<T>,
    VFunc: Fn(V, T) -> V,
    SFunc: Fn(T, T) -> T,
{
    let mut rng = rand::thread_rng();
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let output = vfunc(a, b);
    for i in 0..V::WIDTH {
        assert_eq!(output[i], sfunc(a[i], b))
    }
}

#[inline]
fn assign_op_impl<T, D, V, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    (mut a, mut b): (V, V),
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    V: Vector<Scalar = T> + Signed<T>,
    VFunc: Fn(&mut V, V),
    SFunc: Fn(&mut T, T),
{
    let mut rng = rand::thread_rng();
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }
    for x in b.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output = a.clone();
    vfunc(&mut output, b);
    for i in 0..V::WIDTH {
        sfunc(&mut a[i], b[i]);
        assert_eq!(output[i], a[i])
    }
}

#[inline]
fn assign_scalar_op_impl<T, D, V, VFunc, SFunc>(
    _tag: T,
    distribution: D,
    mut a: V,
    vfunc: VFunc,
    sfunc: SFunc,
) where
    T: Num + core::ops::Neg<Output = T> + core::fmt::Debug + Copy,
    D: rand::distributions::Distribution<T> + Copy,
    V: Vector<Scalar = T> + Signed<T>,
    VFunc: Fn(&mut V, T),
    SFunc: Fn(&mut T, T),
{
    let mut rng = rand::thread_rng();
    let b = rng.sample(distribution);
    for x in a.as_slice_mut() {
        *x = rng.sample(distribution);
    }

    let mut output = a.clone();
    vfunc(&mut output, b);
    for i in 0..V::WIDTH {
        sfunc(&mut a[i], b);
        assert_eq!(output[i], a[i])
    }
}

macro_rules! ops_test {
    {
        $name:ident, $handle:path, $handleinit:expr
    } => {
        #[test]
        fn $name() {
            if let Some(handle) = $handleinit {
                ops_test!{ @impl binary_op_impl, handle, core::ops::Add::add }
                ops_test!{ @impl binary_op_impl, handle, core::ops::Sub::sub }
                ops_test!{ @impl binary_op_impl, handle, core::ops::Mul::mul }
                ops_test!{ @impl binary_op_impl, handle, core::ops::Div::div }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Add::add }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Sub::sub }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Mul::mul }
                ops_test!{ @impl binary_scalar_op_impl, handle, core::ops::Div::div }
                ops_test!{ @impl assign_op_impl, handle, core::ops::AddAssign::add_assign }
                ops_test!{ @impl assign_op_impl, handle, core::ops::SubAssign::sub_assign }
                ops_test!{ @impl assign_op_impl, handle, core::ops::MulAssign::mul_assign }
                ops_test!{ @impl assign_op_impl, handle, core::ops::DivAssign::div_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::AddAssign::add_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::SubAssign::sub_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::MulAssign::mul_assign }
                ops_test!{ @impl assign_scalar_op_impl, handle, core::ops::DivAssign::div_assign }
                ops_test!{ @impl unary_op_impl, handle, core::ops::Neg::neg }
            }
        }
    };
    {
        @impl $test:ident, $handle:ident, $func:path
    } => {
        ops_test!{@types $test, $handle, zeroed_native, $func}
        ops_test!{@types $test, $handle, zeroed1, $func}
        ops_test!{@types $test, $handle, zeroed2, $func}
        ops_test!{@types $test, $handle, zeroed4, $func}
        ops_test!{@types $test, $handle, zeroed8, $func}
    };
    {
        @types $test:ident, $handle:ident, $init:ident, $func:path
    } => {
        $test(0f32, Standard, ops_test!(@init $test, f32, $handle, $init), $func, $func);
        $test(0f64, Standard, ops_test!(@init $test, f64, $handle, $init), $func, $func);
        $test(Complex::<f32>::default(), ComplexDistribution::new(Standard, Standard), ops_test!(@init $test, Complex<f32>, $handle, $init), $func, $func);
        $test(Complex::<f64>::default(), ComplexDistribution::new(Standard, Standard), ops_test!(@init $test, Complex<f64>, $handle, $init), $func, $func);
    };
    {
        @init unary_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        Handle::<$type>::$init($handle)
    };
    {
        @init binary_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        (Handle::<$type>::$init($handle), Handle::<$type>::$init($handle))
    };
    {
        @init binary_scalar_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        Handle::<$type>::$init($handle)
    };
    {
        @init assign_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        (Handle::<$type>::$init($handle), Handle::<$type>::$init($handle))
    };
    {
        @init assign_scalar_op_impl, $type:ty, $handle:ident, $init:ident
    } => {
        Handle::<$type>::$init($handle)
    };
}

ops_test! { ops_generic, safe_simd::generic::Generic, safe_simd::generic::Generic::new() }
ops_test! { ops_sse, safe_simd::x86::Sse, safe_simd::x86::Sse::new() }
ops_test! { ops_avx, safe_simd::x86::Avx, safe_simd::x86::Avx::new() }
