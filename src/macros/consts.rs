#![allow(unused_macros)]

/// Writes pub(crate) and pub values of the given constant type.
macro_rules! const_values {
	(
		$name:ident $(: $feature:literal)*;
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		$( #[cfg_attr(docsrs, doc(cfg(feature = $feature)))] )*
		impl $name {
			$(
				$( #[$privvaldoc] )*
				pub(crate) const $privvalname: Self = Self($privval);
			)*
			$(
				$( #[$pubvaldoc] )*
				pub const $pubvalname: Self = Self($pubval);
			)*
		}
	};
}

/// Declares the type of a constant, along with private and public values. Won't
/// include `Debug` and `Display` impls.
macro_rules! const_no_debug_display {
	(
		$name:ident: $ntype:ty : $feature:literal;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		#[repr(transparent)]
		#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
		pub struct $name(pub(crate) $ntype);

		// Conversions from/to underlying number type.
		impl From<$ntype> for $name {
			fn from(n: $ntype) -> Self {
				Self(n)
			}
		}
		impl From<$name> for $ntype {
			fn from(n: $name) -> Self {
				n.0
			}
		}

		// Formatters.
		impl std::fmt::LowerHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::UpperHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::Binary for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Binary::fmt(&self.0, f)
			}
		}
		impl std::fmt::Octal for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Octal::fmt(&self.0, f)
			}
		}
	};
}

/// Declares the type of an ordinary constant, along with private and public
/// values.
macro_rules! const_ordinary {
	(
		$name:ident: $ntype:ty : $feature:literal;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_no_debug_display! {
			$name: $ntype : $feature;
			$( #[$doc] )*
			#[derive(Debug)]
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(&self.0, f) // delegate
			}
		}

		const_values! {
			$name;
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}
	};
}

/// Declares the type of an ordinary bitflag constant, along with private and
/// public values.
macro_rules! const_bitflag {
	(
		$name:ident: $ntype:ty : $feature:literal;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_ordinary! {
			$name: $ntype : $feature;
			$( #[$doc] )*
			///
			/// This is a bitflag constant, which implements the
			/// [`NativeBitflag`](crate::prelude::NativeBitflag) trait.
			=>
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}

		// Bitflag operations.
		impl std::ops::BitAnd for $name {
			type Output = $name;
			fn bitand(self, rhs: Self) -> Self::Output {
				Self(self.0 & rhs.0)
			}
		}
		impl std::ops::BitAndAssign for $name {
			fn bitand_assign(&mut self, rhs: Self) {
				*self = Self(self.0 & rhs.0);
			}
		}
		impl std::ops::BitOr for $name {
			type Output = $name;
			fn bitor(self, rhs: Self) -> Self {
				Self(self.0 | rhs.0)
			}
		}
		impl std::ops::BitOrAssign for $name {
			fn bitor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 | rhs.0);
			}
		}
		impl std::ops::BitXor for $name {
			type Output = $name;
			fn bitxor(self, rhs: Self) -> Self::Output {
				Self(self.0 ^ rhs.0)
			}
		}
		impl std::ops::BitXorAssign for $name {
			fn bitxor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 ^ rhs.0);
			}
		}
		impl std::ops::Not for $name {
			type Output = $name;
			fn not(self) -> Self::Output {
				Self(!self.0)
			}
		}

		// NativeBitflag trait.
		impl crate::prelude::NativeBitflag for $name {
			fn has(&self, other: Self) -> bool {
				(self.0 & other.0) != 0
			}
		}
	};
}

/// Declares the type of a constant for a window message, convertible to
/// [`WM`](crate::co::WM) constant type, along with private and public values.
macro_rules! const_wm {
	(
		$name:ident $(: $feature:literal)*;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_ordinary! {
			$name: u32 $(: $feature )*;
			$( #[$doc] )*
			///
			/// This is a window message, convertible to [`WM`](crate::co::WM).
			=>
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}

		impl From<$name> for crate::co::WM {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for a WM_COMMAND notification code,
/// convertible to [`CMD`](crate::co::CMD) constant type, along with private and
/// public values.
macro_rules! const_cmd {
	(
		$name:ident $(: $feature:literal)*;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_ordinary! {
			$name: u16 $(: $feature )*;
			$( #[$doc] )*
			///
			/// This is a [`wm::Command`](crate::msg::wm::Command) notification
			/// code, convertible to [`CMD`](crate::co::CMD).
			=>
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}

		impl From<$name> for crate::co::CMD {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for a WM_NOTIFY notification code,
/// convertible to [`NM`](crate::co::NM) constant type, along with private and
/// public values.
macro_rules! const_nm {
	(
		$name:ident $(: $feature:literal)*;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_ordinary! {
			$name: i32 $(: $feature )*;
			$( #[$doc] )*
			///
			/// This is a [`wm::Notify`](crate::msg::wm::Notify) notification
			/// code, convertible to [`NM`](crate::co::NM).
			=>
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}

		impl From<$name> for crate::co::NM {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for a window style, convertible to
/// [`WS`](crate::co::WS) constant type, along with private and public values.
macro_rules! const_ws {
	(
		$name:ident: $ntype:ty $(: $feature:literal)*;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_bitflag! {
			$name: $ntype $(: $feature )*;
			$( #[$doc] )*
			///
			/// This is a window style, convertible to [`WS`](crate::co::WS).
			=>
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}

		impl From<$name> for crate::co::WS {
			fn from(v: $name) -> Self {
				Self(v.0 as _)
			}
		}
	};
}

/// Declares the type of a constant for an extended window style, convertible to
/// [`WS_EX`](crate::co::WS_EX) constant type, along with private and public
/// values.
macro_rules! const_wsex {
	(
		$name:ident $(: $feature:literal)*;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$privvaldoc:meta] )*
			$privvalname:ident $privval:expr
		)*
		=>
		$(
			$( #[$pubvaldoc:meta] )*
			$pubvalname:ident $pubval:expr
		)*
	) => {
		const_bitflag! {
			$name: u32 $(: $feature )*;
			$( #[$doc] )*
			///
			/// This is an extended windoow style, convertible to
			/// [`WS_EX`](crate::co::WS_EX).
			=>
			$(
				$( #[$privvaldoc] )*
				$privvalname $privval
			)*
			=>
			$(
				$( #[$pubvaldoc] )*
				$pubvalname $pubval
			)*
		}

		impl From<$name> for crate::co::WS_EX {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}
