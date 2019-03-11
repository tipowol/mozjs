// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A typesafe bitmask flag generator useful for sets of C-style bitmask flags.
//! It can be used for creating typesafe wrappers around C APIs.
//!
//! The `bitflags!` macro generates a `struct` that manages a set of flags. The
//! flags should only be defined for integer types, otherwise unexpected type
//! errors may occur at compile time.
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!         const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
//!     }
//! }
//!
//! fn main() {
//!     let e1 = Flags::A | Flags::C;
//!     let e2 = Flags::B | Flags::C;
//!     assert_eq!((e1 | e2), Flags::ABC);   // union
//!     assert_eq!((e1 & e2), Flags::C);     // intersection
//!     assert_eq!((e1 - e2), Flags::A);     // set difference
//!     assert_eq!(!e2, Flags::A);           // set complement
//! }
//! ```
//!
//! See [`example_generated::Flags`](./example_generated/struct.Flags.html) for documentation of code
//! generated by the above `bitflags!` expansion.
//!
//! The generated `struct`s can also be extended with type and trait
//! implementations:
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//!
//! use std::fmt;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!     }
//! }
//!
//! impl Flags {
//!     pub fn clear(&mut self) {
//!         self.bits = 0;  // The `bits` field can be accessed from within the
//!                         // same module where the `bitflags!` macro was invoked.
//!     }
//! }
//!
//! impl fmt::Display for Flags {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "hi!")
//!     }
//! }
//!
//! fn main() {
//!     let mut flags = Flags::A | Flags::B;
//!     flags.clear();
//!     assert!(flags.is_empty());
//!     assert_eq!(format!("{}", flags), "hi!");
//!     assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
//!     assert_eq!(format!("{:?}", Flags::B), "B");
//! }
//! ```
//!
//! # Visibility
//!
//! The generated struct and its associated flag constants are not exported
//! out of the current module by default. A definition can be exported out of
//! the current module by adding `pub` before `flags`:
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//!
//! mod example {
//!     bitflags! {
//!         pub struct Flags1: u32 {
//!             const A = 0b00000001;
//!         }
//!     }
//!     bitflags! {
//! #       pub
//!         struct Flags2: u32 {
//!             const B = 0b00000010;
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let flag1 = example::Flags1::A;
//!     let flag2 = example::Flags2::B; // error: const `B` is private
//! }
//! ```
//!
//! # Attributes
//!
//! Attributes can be attached to the generated `struct` by placing them
//! before the `flags` keyword.
//!
//! # Trait implementations
//!
//! The `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash`
//! traits automatically derived for the `struct` using the `derive` attribute.
//! Additional traits can be derived by providing an explicit `derive`
//! attribute on `flags`.
//!
//! The `Extend` and `FromIterator` traits are implemented for the `struct`,
//! too: `Extend` adds the union of the instances of the `struct` iterated over,
//! while `FromIterator` calculates the union.
//!
//! The `Binary`, `Debug`, `LowerExp`, `Octal` and `UpperExp` trait is also
//! implemented by displaying the bits value of the internal struct.
//!
//! ## Operators
//!
//! The following operator traits are implemented for the generated `struct`:
//!
//! - `BitOr` and `BitOrAssign`: union
//! - `BitAnd` and `BitAndAssign`: intersection
//! - `BitXor` and `BitXorAssign`: toggle
//! - `Sub` and `SubAssign`: set difference
//! - `Not`: set complement
//!
//! # Methods
//!
//! The following methods are defined for the generated `struct`:
//!
//! - `empty`: an empty set of flags
//! - `all`: the set of all flags
//! - `bits`: the raw value of the flags currently stored
//! - `from_bits`: convert from underlying bit representation, unless that
//!                representation contains bits that do not correspond to a flag
//! - `from_bits_truncate`: convert from underlying bit representation, dropping
//!                         any bits that do not correspond to flags
//! - `is_empty`: `true` if no flags are currently stored
//! - `is_all`: `true` if all flags are currently set
//! - `intersects`: `true` if there are flags common to both `self` and `other`
//! - `contains`: `true` all of the flags in `other` are contained within `self`
//! - `insert`: inserts the specified flags in-place
//! - `remove`: removes the specified flags in-place
//! - `toggle`: the specified flags will be inserted if not present, and removed
//!             if they are.
//! - `set`: inserts or removes the specified flags depending on the passed value
//!
//! ## Default
//!
//! The `Default` trait is not automatically implemented for the generated struct.
//!
//! If your default value is equal to `0` (which is the same value as calling `empty()`
//! on the generated struct), you can simply derive `Default`:
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//!
//! bitflags! {
//!     // Results in default value with bits: 0
//!     #[derive(Default)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! fn main() {
//!     let derived_default: Flags = Default::default();
//!     assert_eq!(derived_default.bits(), 0);
//! }
//! ```
//!
//! If your default value is not equal to `0` you need to implement `Default` yourself:
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! // explicit `Default` implementation
//! impl Default for Flags {
//!     fn default() -> Flags {
//!         Flags::A | Flags::C
//!     }
//! }
//!
//! fn main() {
//!     let implemented_default: Flags = Default::default();
//!     assert_eq!(implemented_default, (Flags::A | Flags::C));
//! }
//! ```
//!
//! # Zero Flags
//!
//! Flags with a value equal to zero will have some strange behavior that one should be aware of.
//!
//! ```
//! #[macro_use]
//! extern crate bitflags;
//!
//! bitflags! {
//!     struct Flags: u32 {
//!         const NONE = 0b00000000;
//!         const SOME = 0b00000001;
//!     }
//! }
//!
//! fn main() {
//!     let empty = Flags::empty();
//!     let none = Flags::NONE;
//!     let some = Flags::SOME;
//!
//!     // Zero flags are treated as always present
//!     assert!(empty.contains(Flags::NONE));
//!     assert!(none.contains(Flags::NONE));
//!     assert!(some.contains(Flags::NONE));
//!
//!     // Zero flags will be ignored when testing for emptiness
//!     assert!(none.is_empty());
//! }
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/bitflags/1.0.4")]

#[cfg(test)]
#[macro_use]
extern crate std;

// Re-export libcore using an alias so that the macros can work without
// requiring `extern crate core` downstream.
#[doc(hidden)]
pub extern crate core as _core;

/// The macro used to generate the flag structure.
///
/// See the [crate level docs](../bitflags/index.html) for complete documentation.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate bitflags;
///
/// bitflags! {
///     struct Flags: u32 {
///         const A = 0b00000001;
///         const B = 0b00000010;
///         const C = 0b00000100;
///         const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
///     }
/// }
///
/// fn main() {
///     let e1 = Flags::A | Flags::C;
///     let e2 = Flags::B | Flags::C;
///     assert_eq!((e1 | e2), Flags::ABC);   // union
///     assert_eq!((e1 & e2), Flags::C);     // intersection
///     assert_eq!((e1 - e2), Flags::A);     // set difference
///     assert_eq!(!e2, Flags::A);           // set complement
/// }
/// ```
///
/// The generated `struct`s can also be extended with type and trait
/// implementations:
///
/// ```
/// #[macro_use]
/// extern crate bitflags;
///
/// use std::fmt;
///
/// bitflags! {
///     struct Flags: u32 {
///         const A = 0b00000001;
///         const B = 0b00000010;
///     }
/// }
///
/// impl Flags {
///     pub fn clear(&mut self) {
///         self.bits = 0;  // The `bits` field can be accessed from within the
///                         // same module where the `bitflags!` macro was invoked.
///     }
/// }
///
/// impl fmt::Display for Flags {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         write!(f, "hi!")
///     }
/// }
///
/// fn main() {
///     let mut flags = Flags::A | Flags::B;
///     flags.clear();
///     assert!(flags.is_empty());
///     assert_eq!(format!("{}", flags), "hi!");
///     assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
///     assert_eq!(format!("{:?}", Flags::B), "B");
/// }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        pub struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        __bitflags! {
            $(#[$outer])*
            (pub) $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        __bitflags! {
            $(#[$outer])*
            () $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub ($($vis:tt)+) struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        __bitflags! {
            $(#[$outer])*
            (pub ($($vis)+)) $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __bitflags {
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )+
        }
    ) => {
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $(#[$outer])*
        $($vis)* struct $BitFlags {
            bits: $T,
        }

        __impl_bitflags! {
            $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )+
            }
        }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_bitflags {
    (
        $BitFlags:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Flag:ident = $value:expr;
            )+
        }
    ) => {
        impl $crate::_core::fmt::Debug for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                // This convoluted approach is to handle #[cfg]-based flag
                // omission correctly. For example it needs to support:
                //
                //    #[cfg(unix)] const A: Flag = /* ... */;
                //    #[cfg(windows)] const B: Flag = /* ... */;

                // Unconditionally define a check for every flag, even disabled
                // ones.
                #[allow(non_snake_case)]
                trait __BitFlags {
                    $(
                        #[inline]
                        fn $Flag(&self) -> bool { false }
                    )+
                }

                // Conditionally override the check for just those flags that
                // are not #[cfg]ed away.
                impl __BitFlags for $BitFlags {
                    $(
                        __impl_bitflags! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Flag(&self) -> bool {
                                if Self::$Flag.bits == 0 && self.bits != 0 {
                                    false
                                } else {
                                    self.bits & Self::$Flag.bits == Self::$Flag.bits
                                }
                            }
                        }
                    )+
                }

                let mut first = true;
                $(
                    if <$BitFlags as __BitFlags>::$Flag(self) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(__bitflags_stringify!($Flag))?;
                    }
                )+
                if first {
                    f.write_str("(empty)")?;
                }
                Ok(())
            }
        }
        impl $crate::_core::fmt::Binary for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::Binary::fmt(&self.bits, f)
            }
        }
        impl $crate::_core::fmt::Octal for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::Octal::fmt(&self.bits, f)
            }
        }
        impl $crate::_core::fmt::LowerHex for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::LowerHex::fmt(&self.bits, f)
            }
        }
        impl $crate::_core::fmt::UpperHex for $BitFlags {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                $crate::_core::fmt::UpperHex::fmt(&self.bits, f)
            }
        }

        #[allow(dead_code)]
        impl $BitFlags {
            $(
                $(#[$attr $($args)*])*
                pub const $Flag: $BitFlags = $BitFlags { bits: $value };
            )+

            /// Returns an empty set of flags.
            #[inline]
            pub fn empty() -> $BitFlags {
                $BitFlags { bits: 0 }
            }

            /// Returns the set containing all flags.
            #[inline]
            pub fn all() -> $BitFlags {
                // See `Debug::fmt` for why this approach is taken.
                #[allow(non_snake_case)]
                trait __BitFlags {
                    $(
                        #[inline]
                        fn $Flag() -> $T { 0 }
                    )+
                }
                impl __BitFlags for $BitFlags {
                    $(
                        __impl_bitflags! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Flag() -> $T { Self::$Flag.bits }
                        }
                    )+
                }
                $BitFlags { bits: $(<$BitFlags as __BitFlags>::$Flag())|+ }
            }

            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub fn bits(&self) -> $T {
                self.bits
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub fn from_bits(bits: $T) -> $crate::_core::option::Option<$BitFlags> {
                if (bits & !$BitFlags::all().bits()) == 0 {
                    $crate::_core::option::Option::Some($BitFlags { bits })
                } else {
                    $crate::_core::option::Option::None
                }
            }

            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub fn from_bits_truncate(bits: $T) -> $BitFlags {
                $BitFlags { bits } & $BitFlags::all()
            }

            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub fn is_empty(&self) -> bool {
                *self == $BitFlags::empty()
            }

            /// Returns `true` if all flags are currently set.
            #[inline]
            pub fn is_all(&self) -> bool {
                *self == $BitFlags::all()
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub fn intersects(&self, other: $BitFlags) -> bool {
                !(*self & other).is_empty()
            }

            /// Returns `true` all of the flags in `other` are contained within `self`.
            #[inline]
            pub fn contains(&self, other: $BitFlags) -> bool {
                (*self & other) == other
            }

            /// Inserts the specified flags in-place.
            #[inline]
            pub fn insert(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }

            /// Removes the specified flags in-place.
            #[inline]
            pub fn remove(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }

            /// Toggles the specified flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: $BitFlags) {
                self.bits ^= other.bits;
            }

            /// Inserts or removes the specified flags depending on the passed value.
            #[inline]
            pub fn set(&mut self, other: $BitFlags, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
        }

        impl $crate::_core::ops::BitOr for $BitFlags {
            type Output = $BitFlags;

            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits | other.bits }
            }
        }

        impl $crate::_core::ops::BitOrAssign for $BitFlags {

            /// Adds the set of flags.
            #[inline]
            fn bitor_assign(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }
        }

        impl $crate::_core::ops::BitXor for $BitFlags {
            type Output = $BitFlags;

            /// Returns the left flags, but with all the right flags toggled.
            #[inline]
            fn bitxor(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits ^ other.bits }
            }
        }

        impl $crate::_core::ops::BitXorAssign for $BitFlags {

            /// Toggles the set of flags.
            #[inline]
            fn bitxor_assign(&mut self, other: $BitFlags) {
                self.bits ^= other.bits;
            }
        }

        impl $crate::_core::ops::BitAnd for $BitFlags {
            type Output = $BitFlags;

            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & other.bits }
            }
        }

        impl $crate::_core::ops::BitAndAssign for $BitFlags {

            /// Disables all flags disabled in the set.
            #[inline]
            fn bitand_assign(&mut self, other: $BitFlags) {
                self.bits &= other.bits;
            }
        }

        impl $crate::_core::ops::Sub for $BitFlags {
            type Output = $BitFlags;

            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(self, other: $BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & !other.bits }
            }
        }

        impl $crate::_core::ops::SubAssign for $BitFlags {

            /// Disables all flags enabled in the set.
            #[inline]
            fn sub_assign(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }
        }

        impl $crate::_core::ops::Not for $BitFlags {
            type Output = $BitFlags;

            /// Returns the complement of this set of flags.
            #[inline]
            fn not(self) -> $BitFlags {
                $BitFlags { bits: !self.bits } & $BitFlags::all()
            }
        }

        impl $crate::_core::iter::Extend<$BitFlags> for $BitFlags {
            fn extend<T: $crate::_core::iter::IntoIterator<Item=$BitFlags>>(&mut self, iterator: T) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }

        impl $crate::_core::iter::FromIterator<$BitFlags> for $BitFlags {
            fn from_iter<T: $crate::_core::iter::IntoIterator<Item=$BitFlags>>(iterator: T) -> $BitFlags {
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
    };

    // Every attribute that the user writes on a const is applied to the
    // corresponding const that we generate, but within the implementation of
    // Debug and all() we want to ignore everything but #[cfg] attributes. In
    // particular, including a #[deprecated] attribute on those items would fail
    // to compile.
    // https://github.com/bitflags/bitflags/issues/109
    //
    // Input:
    //
    //     ? #[cfg(feature = "advanced")]
    //     ? #[deprecated(note = "Use somthing else.")]
    //     ? #[doc = r"High quality documentation."]
    //     fn f() -> i32 { /* ... */ }
    //
    // Output:
    //
    //     #[cfg(feature = "advanced")]
    //     fn f() -> i32 { /* ... */ }
    (
        $(#[$filtered:meta])*
        ? #[cfg $($cfgargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        __impl_bitflags! {
            $(#[$filtered])*
            #[cfg $($cfgargs)*]
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        // $next != `cfg`
        ? #[$next:ident $($nextargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        __impl_bitflags! {
            $(#[$filtered])*
            // $next filtered out
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        fn $($item:tt)*
    ) => {
        $(#[$filtered])*
        fn $($item)*
    };
}

// Same as std::stringify but callable from __impl_bitflags, which needs to use
// local_inner_macros so can only directly call macros from this crate.
#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags_stringify {
    ($s:ident) => {
        stringify!($s)
    };
}

#[cfg(feature = "example_generated")]
pub mod example_generated;

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    bitflags! {
        #[doc = "> The first principle is that you must not fool yourself — and"]
        #[doc = "> you are the easiest person to fool."]
        #[doc = "> "]
        #[doc = "> - Richard Feynman"]
        struct Flags: u32 {
            const A = 0b00000001;
            #[doc = "<pcwalton> macros are way better at generating code than trans is"]
            const B = 0b00000010;
            const C = 0b00000100;
            #[doc = "* cmr bed"]
            #[doc = "* strcat table"]
            #[doc = "<strcat> wait what?"]
            const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
        }
    }

    bitflags! {
        struct _CfgFlags: u32 {
            #[cfg(windows)]
            const _CFG_A = 0b01;
            #[cfg(unix)]
            const _CFG_B = 0b01;
            #[cfg(windows)]
            const _CFG_C = _CFG_A.bits | 0b10;
        }
    }

    bitflags! {
        struct AnotherSetOfFlags: i8 {
            const ANOTHER_FLAG = -1_i8;
        }
    }

    bitflags! {
        struct LongFlags: u32 {
            const LONG_A = 0b1111111111111111;
        }
    }

    #[test]
    fn test_bits() {
        assert_eq!(Flags::empty().bits(), 0b00000000);
        assert_eq!(Flags::A.bits(), 0b00000001);
        assert_eq!(Flags::ABC.bits(), 0b00000111);

        assert_eq!(AnotherSetOfFlags::empty().bits(), 0b00);
        assert_eq!(AnotherSetOfFlags::ANOTHER_FLAG.bits(), !0_i8);
    }

    #[test]
    fn test_from_bits() {
        assert_eq!(Flags::from_bits(0), Some(Flags::empty()));
        assert_eq!(Flags::from_bits(0b1), Some(Flags::A));
        assert_eq!(Flags::from_bits(0b10), Some(Flags::B));
        assert_eq!(Flags::from_bits(0b11), Some(Flags::A | Flags::B));
        assert_eq!(Flags::from_bits(0b1000), None);

        assert_eq!(
            AnotherSetOfFlags::from_bits(!0_i8),
            Some(AnotherSetOfFlags::ANOTHER_FLAG)
        );
    }

    #[test]
    fn test_from_bits_truncate() {
        assert_eq!(Flags::from_bits_truncate(0), Flags::empty());
        assert_eq!(Flags::from_bits_truncate(0b1), Flags::A);
        assert_eq!(Flags::from_bits_truncate(0b10), Flags::B);
        assert_eq!(Flags::from_bits_truncate(0b11), (Flags::A | Flags::B));
        assert_eq!(Flags::from_bits_truncate(0b1000), Flags::empty());
        assert_eq!(Flags::from_bits_truncate(0b1001), Flags::A);

        assert_eq!(
            AnotherSetOfFlags::from_bits_truncate(0_i8),
            AnotherSetOfFlags::empty()
        );
    }

    #[test]
    fn test_is_empty() {
        assert!(Flags::empty().is_empty());
        assert!(!Flags::A.is_empty());
        assert!(!Flags::ABC.is_empty());

        assert!(!AnotherSetOfFlags::ANOTHER_FLAG.is_empty());
    }

    #[test]
    fn test_is_all() {
        assert!(Flags::all().is_all());
        assert!(!Flags::A.is_all());
        assert!(Flags::ABC.is_all());

        assert!(AnotherSetOfFlags::ANOTHER_FLAG.is_all());
    }

    #[test]
    fn test_two_empties_do_not_intersect() {
        let e1 = Flags::empty();
        let e2 = Flags::empty();
        assert!(!e1.intersects(e2));

        assert!(AnotherSetOfFlags::ANOTHER_FLAG.intersects(AnotherSetOfFlags::ANOTHER_FLAG));
    }

    #[test]
    fn test_empty_does_not_intersect_with_full() {
        let e1 = Flags::empty();
        let e2 = Flags::ABC;
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_disjoint_intersects() {
        let e1 = Flags::A;
        let e2 = Flags::B;
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_overlapping_intersects() {
        let e1 = Flags::A;
        let e2 = Flags::A | Flags::B;
        assert!(e1.intersects(e2));
    }

    #[test]
    fn test_contains() {
        let e1 = Flags::A;
        let e2 = Flags::A | Flags::B;
        assert!(!e1.contains(e2));
        assert!(e2.contains(e1));
        assert!(Flags::ABC.contains(e2));

        assert!(AnotherSetOfFlags::ANOTHER_FLAG.contains(AnotherSetOfFlags::ANOTHER_FLAG));
    }

    #[test]
    fn test_insert() {
        let mut e1 = Flags::A;
        let e2 = Flags::A | Flags::B;
        e1.insert(e2);
        assert_eq!(e1, e2);

        let mut e3 = AnotherSetOfFlags::empty();
        e3.insert(AnotherSetOfFlags::ANOTHER_FLAG);
        assert_eq!(e3, AnotherSetOfFlags::ANOTHER_FLAG);
    }

    #[test]
    fn test_remove() {
        let mut e1 = Flags::A | Flags::B;
        let e2 = Flags::A | Flags::C;
        e1.remove(e2);
        assert_eq!(e1, Flags::B);

        let mut e3 = AnotherSetOfFlags::ANOTHER_FLAG;
        e3.remove(AnotherSetOfFlags::ANOTHER_FLAG);
        assert_eq!(e3, AnotherSetOfFlags::empty());
    }

    #[test]
    fn test_operators() {
        let e1 = Flags::A | Flags::C;
        let e2 = Flags::B | Flags::C;
        assert_eq!((e1 | e2), Flags::ABC); // union
        assert_eq!((e1 & e2), Flags::C); // intersection
        assert_eq!((e1 - e2), Flags::A); // set difference
        assert_eq!(!e2, Flags::A); // set complement
        assert_eq!(e1 ^ e2, Flags::A | Flags::B); // toggle
        let mut e3 = e1;
        e3.toggle(e2);
        assert_eq!(e3, Flags::A | Flags::B);

        let mut m4 = AnotherSetOfFlags::empty();
        m4.toggle(AnotherSetOfFlags::empty());
        assert_eq!(m4, AnotherSetOfFlags::empty());
    }

    #[test]
    fn test_set() {
        let mut e1 = Flags::A | Flags::C;
        e1.set(Flags::B, true);
        e1.set(Flags::C, false);

        assert_eq!(e1, Flags::A | Flags::B);
    }

    #[test]
    fn test_assignment_operators() {
        let mut m1 = Flags::empty();
        let e1 = Flags::A | Flags::C;
        // union
        m1 |= Flags::A;
        assert_eq!(m1, Flags::A);
        // intersection
        m1 &= e1;
        assert_eq!(m1, Flags::A);
        // set difference
        m1 -= m1;
        assert_eq!(m1, Flags::empty());
        // toggle
        m1 ^= e1;
        assert_eq!(m1, e1);
    }

    #[test]
    fn test_extend() {
        let mut flags;

        flags = Flags::empty();
        flags.extend([].iter().cloned());
        assert_eq!(flags, Flags::empty());

        flags = Flags::empty();
        flags.extend([Flags::A, Flags::B].iter().cloned());
        assert_eq!(flags, Flags::A | Flags::B);

        flags = Flags::A;
        flags.extend([Flags::A, Flags::B].iter().cloned());
        assert_eq!(flags, Flags::A | Flags::B);

        flags = Flags::B;
        flags.extend([Flags::A, Flags::ABC].iter().cloned());
        assert_eq!(flags, Flags::ABC);
    }

    #[test]
    fn test_from_iterator() {
        assert_eq!([].iter().cloned().collect::<Flags>(), Flags::empty());
        assert_eq!(
            [Flags::A, Flags::B].iter().cloned().collect::<Flags>(),
            Flags::A | Flags::B
        );
        assert_eq!(
            [Flags::A, Flags::ABC].iter().cloned().collect::<Flags>(),
            Flags::ABC
        );
    }

    #[test]
    fn test_lt() {
        let mut a = Flags::empty();
        let mut b = Flags::empty();

        assert!(!(a < b) && !(b < a));
        b = Flags::B;
        assert!(a < b);
        a = Flags::C;
        assert!(!(a < b) && b < a);
        b = Flags::C | Flags::B;
        assert!(a < b);
    }

    #[test]
    fn test_ord() {
        let mut a = Flags::empty();
        let mut b = Flags::empty();

        assert!(a <= b && a >= b);
        a = Flags::A;
        assert!(a > b && a >= b);
        assert!(b < a && b <= a);
        b = Flags::B;
        assert!(b > a && b >= a);
        assert!(a < b && a <= b);
    }

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_hash() {
        let mut x = Flags::empty();
        let mut y = Flags::empty();
        assert_eq!(hash(&x), hash(&y));
        x = Flags::all();
        y = Flags::ABC;
        assert_eq!(hash(&x), hash(&y));
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
        assert_eq!(format!("{:?}", Flags::empty()), "(empty)");
        assert_eq!(format!("{:?}", Flags::ABC), "A | B | C | ABC");
    }

    #[test]
    fn test_binary() {
        assert_eq!(format!("{:b}", Flags::ABC), "111");
        assert_eq!(format!("{:#b}", Flags::ABC), "0b111");
    }

    #[test]
    fn test_octal() {
        assert_eq!(format!("{:o}", LongFlags::LONG_A), "177777");
        assert_eq!(format!("{:#o}", LongFlags::LONG_A), "0o177777");
    }

    #[test]
    fn test_lowerhex() {
        assert_eq!(format!("{:x}", LongFlags::LONG_A), "ffff");
        assert_eq!(format!("{:#x}", LongFlags::LONG_A), "0xffff");
    }

    #[test]
    fn test_upperhex() {
        assert_eq!(format!("{:X}", LongFlags::LONG_A), "FFFF");
        assert_eq!(format!("{:#X}", LongFlags::LONG_A), "0xFFFF");
    }

    mod submodule {
        bitflags! {
            pub struct PublicFlags: i8 {
                const X = 0;
            }
        }
        bitflags! {
            struct PrivateFlags: i8 {
                const Y = 0;
            }
        }

        #[test]
        fn test_private() {
            let _ = PrivateFlags::Y;
        }
    }

    #[test]
    fn test_public() {
        let _ = submodule::PublicFlags::X;
    }

    mod t1 {
        mod foo {
            pub type Bar = i32;
        }

        bitflags! {
            /// baz
            struct Flags: foo::Bar {
                const A = 0b00000001;
                #[cfg(foo)]
                const B = 0b00000010;
                #[cfg(foo)]
                const C = 0b00000010;
            }
        }
    }

    #[test]
    fn test_in_function() {
        bitflags! {
           struct Flags: u8 {
                const A = 1;
                #[cfg(any())] // false
                const B = 2;
            }
        }
        assert_eq!(Flags::all(), Flags::A);
        assert_eq!(format!("{:?}", Flags::A), "A");
    }

    #[test]
    fn test_deprecated() {
        bitflags! {
            pub struct TestFlags: u32 {
                #[deprecated(note = "Use something else.")]
                const ONE = 1;
            }
        }
    }

    #[test]
    fn test_pub_crate() {
        mod module {
            bitflags! {
                pub (crate) struct Test: u8 {
                    const FOO = 1;
                }
            }
        }

        assert_eq!(module::Test::FOO.bits(), 1);
    }

    #[test]
    fn test_pub_in_module() {
        mod module {
            mod submodule {
                bitflags! {
                    // `pub (in super)` means only the module `module` will
                    // be able to access this.
                    pub (in super) struct Test: u8 {
                        const FOO = 1;
                    }
                }
            }

            mod test {
                // Note: due to `pub (in super)`,
                // this cannot be accessed directly by the testing code.
                pub(super) fn value() -> u8 {
                    super::submodule::Test::FOO.bits()
                }
            }

            pub fn value() -> u8 {
                test::value()
            }
        }

        assert_eq!(module::value(), 1)
    }

    #[test]
    fn test_zero_value_flags() {
        bitflags! {
            struct Flags: u32 {
                const NONE = 0b0;
                const SOME = 0b1;
            }
        }

        assert!(Flags::empty().contains(Flags::NONE));
        assert!(Flags::SOME.contains(Flags::NONE));
        assert!(Flags::NONE.is_empty());

        assert_eq!(format!("{:?}", Flags::empty()), "NONE");
        assert_eq!(format!("{:?}", Flags::SOME), "SOME");
    }
}
