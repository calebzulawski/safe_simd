macro_rules! arithmetic_ops {
    {
        @new $type:ty, $trait:ident, $func:ident, $op:expr
    } => {
        impl core::ops::$trait<$type> for $type {
            type Output = Self;
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(self, rhs: Self) -> Self {
                Self(unsafe { $op(self.0, rhs.0) })
            }
        }
    };
    {
        @assign $type:ty, $trait:ident, $func:ident, $op:expr
    } => {
        impl core::ops::$trait<$type> for $type {
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(&mut self, rhs: Self) {
                self.0 = unsafe { $op(self.0, rhs.0) };
            }
        }
    };
    {
        for $type:ty:
            add -> $add_expr:expr,
            sub -> $sub_expr:expr,
            mul -> $mul_expr:expr,
            div -> $div_expr:expr
    } => {
        arithmetic_ops!{@new $type, Add, add, $add_expr}
        arithmetic_ops!{@new $type, Sub, sub, $sub_expr}
        arithmetic_ops!{@new $type, Mul, mul, $mul_expr}
        arithmetic_ops!{@new $type, Div, div, $div_expr}
        arithmetic_ops!{@assign $type, AddAssign, add_assign, $add_expr}
        arithmetic_ops!{@assign $type, SubAssign, sub_assign, $sub_expr}
        arithmetic_ops!{@assign $type, MulAssign, mul_assign, $mul_expr}
        arithmetic_ops!{@assign $type, DivAssign, div_assign, $div_expr}
    };
}

macro_rules! as_slice {
    {
        $type:ty
    } => {
        impl AsRef<[<$type as crate::vector::Vector>::Scalar]> for $type {
            fn as_ref(&self) -> &[<$type as crate::vector::Vector>::Scalar] {
                use crate::vector::Vector;
                self.as_slice()
            }
        }

        impl AsMut<[<$type as crate::vector::Vector>::Scalar]> for $type {
            fn as_mut(&mut self) -> &mut [<$type as crate::vector::Vector>::Scalar] {
                use crate::vector::Vector;
                self.as_slice_mut()
            }
        }

        impl core::ops::Deref for $type {
            type Target = [<Self as crate::vector::Vector>::Scalar];
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl core::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
                self.as_slice_mut()
            }
        }
    }
}
