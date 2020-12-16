//! mostly no-op copy of https://github.com/facebookexperimental/MIRAI/blob/master/annotations/src/lib.rs

#[macro_export]
macro_rules! debug_checked_precondition {
    ($condition:expr) => (
        debug_assert!($condition);
    );
    ($condition:expr, $message:literal) => (
        debug_assert!($condition, $message);
    );
    ($condition:expr, $($arg:tt)*) => (
        debug_assert!($condition, $($arg)*);
    );
}

#[macro_export]
macro_rules! debug_checked_postcondition {
    ($condition:expr) => (
        debug_assert!($condition);
    );
    ($condition:expr, $message:literal) => (
        debug_assert!($condition, $message);
    );
    ($condition:expr, $($arg:tt)*) => (
        debug_assert!($condition, $($arg)*);
    );
}

#[macro_export]
macro_rules! add_tag {
    ($value:expr, $tag:ty) => {};
}

#[macro_export]
macro_rules! precondition {
    ($condition:expr) => {};
    ($condition:expr, $message:literal) => {};
    ($condition:expr, $($arg:tt)*) => {};
}

#[macro_export]
macro_rules! assume {
    ($condition:expr) => {};
}

/// Provides a way to check if a value has been tagged with a type, using the add_tag! macro.
/// When compiled with an unmodified Rust compiler, this results in true.
/// When compiled with MIRAI, this will be true if all data flows into the argument of this
/// call has gone via a call to add_tag!.
#[macro_export]
macro_rules! has_tag {
    ($value:expr, $tag:ty) => {
        if cfg!(mirai) {
        } else {
            true
        }
    };
}

/// Provides a way to check if a value has *not* been tagged with a type using add_tag!.
/// When compiled with an unmodified Rust compiler, this results in true.
/// When compiled with MIRAI, this will be true if none data flows into the argument of this
/// call has gone via a call to add_tag!.
#[macro_export]
macro_rules! does_not_have_tag {
    ($value:expr, $tag:ty) => {
        if cfg!(mirai) {
        } else {
            true
        }
    };
}

/// Equivalent to a no op when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume that the preconditions of the next
/// function call have been met.
/// This is to be used when the precondition has been inferred and involves private state that
/// cannot be constrained by a normal assumption.
/// Note that it is bad style for an API to rely on preconditions that cannot be checked by the
/// caller, so this is only here for supporting legacy APIs.
#[macro_export]
macro_rules! assume_preconditions {
    () => {
        if cfg!(mirai) {}
    };
}

/// Equivalent to the standard assert! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition unless it can
/// prove it to be false.
#[macro_export]
macro_rules! checked_assume {
    ($condition:expr) => (
        if cfg!(mirai) {
        } else {
            assert!($condition);
        }
    );
    ($condition:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert!($condition, $($arg)*);
        }
    );
}

/// Equivalent to the standard assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition unless it can
/// prove it to be false.
#[macro_export]
macro_rules! checked_assume_eq {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition unless it can
/// prove it to be false.
#[macro_export]
macro_rules! checked_assume_ne {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition unless it can
/// prove it to be false.
#[macro_export]
macro_rules! debug_checked_assume {
    ($condition:expr) => (
        if cfg!(mirai) {
        } else {
            debug_assert!($condition);
        }
    );
    ($condition:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            debug_assert!($condition, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition unless it can
/// prove it to be false.
#[macro_export]
macro_rules! debug_checked_assume_eq {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            debug_assert_eq!($left, $right);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            debug_assert_eq!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition unless it can
/// prove it to be false.
#[macro_export]
macro_rules! debug_checked_assume_ne {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            debug_assert_ne!($left, $right);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            debug_assert_ne!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to a no op when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! postcondition {
    ($condition:expr) => {
        #[cfg(mirai)]
        {}
    };
    ($condition:expr, $message:literal) => {
        #[cfg(mirai)]
        {}
    };
    ($condition:expr, $($arg:tt)*) => {
        #[cfg(mirai)]
        {}
    };
}

/// Equivalent to a no op when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! assumed_postcondition {
    ($condition:expr) => {
        #[cfg(mirai)]
        {}
        #[cfg(not(mirai))]
        {
            debug_assert!($condition);
        }
    };
}

/// Equivalent to the standard assert! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! checked_postcondition {
    ($condition:expr) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            assert!($condition);
        }
    );
    ($condition:expr, $message:literal) => {
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            assert!($condition, $message);
        }
    };
    ($condition:expr, $($arg:tt)*) => {
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            assert!($condition, $($arg)*);
        }
    };
}

/// Equivalent to the standard assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! checked_postcondition_eq {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! checked_postcondition_ne {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! debug_checked_postcondition_eq {
    ($left:expr, $right:expr) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            debug_assert_eq!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            debug_assert_eq!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            debug_assert_eq!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify the condition at the
/// point where it appears in a function, but to also add it a postcondition that can
/// be assumed by the caller of the function.
#[macro_export]
macro_rules! debug_checked_postcondition_ne {
    ($left:expr, $right:expr) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            debug_assert_ne!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            debug_assert_ne!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        #[cfg(mirai)] {
        }
        #[cfg(not(mirai))] {
            debug_assert_ne!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard assert! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition at the
/// point where it appears in a function, but to also add it a precondition that must
/// be verified by the caller of the function.
#[macro_export]
macro_rules! checked_precondition {
    ($condition:expr) => (
        if cfg!(mirai) {
        } else {
            assert!($condition);
        }
    );
    ($condition:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            assert!($condition, $message);
        }
    );
    ($condition:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert!($condition, $($arg)*);
        }
    );
}

/// Equivalent to the standard assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition at the
/// point where it appears in a function, but to also add it a precondition that must
/// be verified by the caller of the function.
#[macro_export]
macro_rules! checked_precondition_eq {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert_eq!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition at the
/// point where it appears in a function, but to also add it a precondition that must
/// be verified by the caller of the function.
#[macro_export]
macro_rules! checked_precondition_ne {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            assert_ne!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition at the
/// point where it appears in a function, but to also add it a precondition that must
/// be verified by the caller of the function.
#[macro_export]
macro_rules! debug_checked_precondition_eq {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            debug_assert_eq!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            debug_assert_eq!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            debug_assert_eq!($left, $right, $($arg)*);
        }
    );
}

/// Equivalent to the standard debug_assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume the condition at the
/// point where it appears in a function, but to also add it a precondition that must
/// be verified by the caller of the function.
#[macro_export]
macro_rules! debug_checked_precondition_ne {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
        } else {
            debug_assert_ne!($left, $right);
        }
    );
    ($left:expr, $right:expr, $message:literal) => (
        if cfg!(mirai) {
        } else {
            debug_assert_ne!($left, $right, $message);
        }
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
        if cfg!(mirai) {
        } else {
            debug_assert_ne!($left, $right, $($arg)*);
        }
    );
}

/// Terminates the program with a panic that is tagged as being an unrecoverable error.
/// Use this for errors that arise in correct programs due to external factors.
/// For example, if a file that is essential for running cannot be found for some reason.
#[macro_export]
macro_rules! unrecoverable {
    ($fmt:expr) => (
        panic!(concat!("unrecoverable: ", stringify!($fmt)));
    );
    ($fmt:expr, $($arg:tt)+) => (
        panic!(concat!("unrecoverable: ", stringify!($fmt)), $($arg)+);
    );
}

/// Equivalent to a no op when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! verify {
    ($condition:expr) => {
        if cfg!(mirai) {}
    };
}

/// Equivalent to the standard assert! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! checked_verify {
    ($condition:expr) => (
            assert!($condition);
    );
    ($condition:expr, $message:literal) => {
            assert!($condition, $message);
    };
    ($condition:expr, $($arg:tt)*) => (
            assert!($condition, $($arg)*);
    );
}

/// Equivalent to the standard assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! checked_verify_eq {
    ($left:expr, $right:expr) => (
            assert_eq!($left, $right);
    );
    ($left:expr, $right:expr, $message:literal) => (
            assert_eq!($left, $right, $message);
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
            assert_eq!($left, $right, $($arg)*);
    );
}

/// Equivalent to the standard assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! checked_verify_ne {
    ($left:expr, $right:expr) => (
            assert_ne!($left, $right);
    );
    ($left:expr, $right:expr, $message:literal) => (
            assert_ne!($left, $right, $message);
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
            assert_ne!($left, $right, $($arg)*);
    );
}

/// Equivalent to the standard debug_assert! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! debug_checked_verify {
	($condition:expr) => (
			 debug_assert!($condition);
	);
    ($condition:expr, $message:literal) => {
            debug_assert!($condition, $message);
    };
    ($condition:expr, $($arg:tt)*) => (
            debug_assert!($condition, $($arg)*);
    );
}

/// Equivalent to the standard debug_assert_eq! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! debug_checked_verify_eq {
    ($left:expr, $right:expr) => (
            debug_assert_eq!($left, $right);
    );
    ($left:expr, $right:expr, $message:literal) => (
            debug_assert_eq!($left, $right, $message);
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
            debug_assert_eq!($left, $right, $($arg)*);
    );
}

/// Equivalent to the standard debug_assert_ne! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to check the condition and
/// emit a diagnostic unless it can prove it to be true.
#[macro_export]
macro_rules! debug_checked_verify_ne {
    ($left:expr, $right:expr) => (
            debug_assert_ne!($left, $right);
    );
    ($left:expr, $right:expr, $message:literal) => (
            debug_assert_ne!($left, $right, $message);
    );
    ($left:expr, $right:expr, $($arg:tt)*) => (
            debug_assert_ne!($left, $right, $($arg)*);
    );
}
/*
/// Retrieves the value of the specified model field, or the given default value if the model field
/// is not set.
/// This function has no meaning outside of a verification
/// condition and should not be used with checked or debug_checked conditions.
/// For example: precondition!(get_model_field!(x, f) > 1).
#[macro_export]
macro_rules! get_model_field {
    ($target:expr, $field_name:ident, $default_value:expr) => {
    };
}

/// Provides a way to refer to the result value of an abstract or contract function without
/// specifying an actual value anywhere.
/// This macro expands to unimplemented!() unless the program is compiled with MIRAI.
/// It result should therefore not be assigned to a variable unless the assignment is contained
/// inside a specification macro argument list.
/// It may, however, be the return value of the function, which should never be called and
/// therefore unimplemented!() is the right behavior for it at runtime.
#[macro_export]
macro_rules! result {
    () => {
        if cfg!(mirai) {
        } else {
            unimplemented!()
        }
    };
}

/// Sets the value of the specified model field.
/// A model field does not exist at runtime and is invisible to the Rust compiler.
/// This macro expands to nothing unless the program is compiled with MIRAI.
#[macro_export]
macro_rules! set_model_field {
    ($target:expr, $field_name:ident, $value:expr) => {
        if cfg!(mirai) {
        }
    };
}
 */
/// Equivalent to unreachable! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to assume that the annotation statement cannot be reached.
#[macro_export]
macro_rules! assume_unreachable {
    () => {
        if cfg!(mirai) {
            unreachable!()
        } else {
            unreachable!()
        }
    };
    ($message:literal) => {
        if cfg!(mirai) {
            unreachable!()
        } else {
            unreachable!($message)
        }
    };
    ($msg:expr,) => ({
        if cfg!(mirai) {
            unreachable!()
        } else {
            unreachable!($msg)
        }
    });
    ($fmt:expr, $($arg:tt)*) => {
        if cfg!(mirai) {
            unreachable!()
        } else {
            unreachable!($fmt, $($arg)*)
        }
    };
}

/// Equivalent to unreachable! when used with an unmodified Rust compiler.
/// When compiled with MIRAI, this causes MIRAI to verify that the annotation statement cannot be reached.
#[macro_export]
macro_rules! verify_unreachable {
    () => {
        if cfg!(mirai) {
            panic!("statement is reachable");
        } else {
            unreachable!()
        }
    };
    ($message:literal) => {
        if cfg!(mirai) {
            panic!($message);
        } else {
            unreachable!($message)
        }
    };
    ($msg:expr,) => ({
        if cfg!(mirai) {
            panic!($message)
        } else {
            unreachable!($msg)
        }
    });
    ($fmt:expr, $($arg:tt)*) => {
        if cfg!(mirai) {
            panic!($fmt, $($arg)*);
        } else {
            unreachable!($fmt, $($arg)*)
        }
    };
}
