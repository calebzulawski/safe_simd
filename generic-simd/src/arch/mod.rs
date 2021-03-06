//! Architecture-specific types.

/// Indicates support for a particular CPU feature.
///
/// # Safety
/// Implementing `Token` for a type indicates that the type is only constructible when the
/// associated CPU features are supported.
pub unsafe trait Token: Copy + From<Self> + Into<Self> {
    /// Detects whether the required CPU features are supported.
    fn new() -> Option<Self>;

    /// Creates the token without detecting if the CPU features are supported.
    ///
    /// # Safety
    /// Calling this function causes undefined behavior if the required CPU features are not
    /// supported.
    unsafe fn new_unchecked() -> Self;
}

#[allow(unused_macros)]
macro_rules! impl_token {
    { $name:ident => $($features:tt),+ } => {
        unsafe impl $crate::arch::Token for $name {
            #[inline]
            fn new() -> Option<Self> {
                if multiversion::are_cpu_features_detected!($($features),*) {
                    Some(Self(()))
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn new_unchecked() -> Self {
                Self(())
            }
        }

        impl core::convert::From<$name> for $crate::arch::generic::Generic {
            #[inline]
            fn from(_: $name) -> Self {
                Self
            }
        }
    }
}

pub mod generic;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

#[cfg(all(feature = "nightly", target_arch = "aarch64"))]
pub mod arm;

#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    feature = "nightly",
))]
pub mod wasm;

/// Invokes a macro with the supported token types.
///
/// Invokes the macro with the list of [`Token`] types as arguments in priority order, delimited
/// by commas (including a trailing comma).
///
/// The following example creates a `SupportedScalar` supertrait that implements [`ScalarExt`] for
/// each token:
/// ```
/// use generic_simd::{call_macro_with_tokens, scalar::ScalarExt};
///
/// macro_rules! supported_scalars {
///     { $($token:ty,)+ } => {
///         trait SupportedScalar: Copy $(+ ScalarExt<$token>)* {}
///     }
/// }
///
/// call_macro_with_tokens!{ supported_scalars }
/// ```
///
/// [`Token`]: arch/trait.Token.html
/// [`ScalarExt`]: scalar/trait.ScalarExt.html
#[macro_export]
macro_rules! call_macro_with_tokens {
    { $mac:ident } => { $crate::call_macro_with_tokens_impl! { $mac } }
}

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "aarch64", feature = "nightly"),
    all(
        target_arch = "wasm32",
        target_feature = "simd128",
        feature = "nightly",
    ),
)))]
#[doc(hidden)]
#[macro_export]
macro_rules! call_macro_with_tokens_impl {
    { $mac:ident } => {
        $mac! {
            $crate::arch::generic::Generic,
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[doc(hidden)]
#[macro_export]
macro_rules! call_macro_with_tokens_impl {
    { $mac:ident } => {
        $mac! {
            $crate::arch::x86::Avx,
            $crate::arch::x86::Sse,
            $crate::arch::generic::Generic,
        }
    }
}

#[cfg(all(feature = "nightly", target_arch = "aarch64"))]
#[doc(hidden)]
#[macro_export]
macro_rules! call_macro_with_tokens_impl {
    { $mac:ident } => {
        $mac! {
            $crate::arch::arm::Neon,
            $crate::arch::generic::Generic,
        }
    }
}

#[cfg(all(
    target_arch = "wasm32",
    target_feature = "simd128",
    feature = "nightly",
))]
#[doc(hidden)]
#[macro_export]
macro_rules! call_macro_with_tokens_impl {
    { $mac:ident } => {
        $mac! {
            $crate::arch::wasm::Simd128,
            $crate::arch::generic::Generic,
        }
    }
}
