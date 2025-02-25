pub(in crate::kernel) mod ffi;
pub(crate) mod privs;

mod aliases;
mod co_traits;
mod enums;
mod funcs;
mod handles;
mod structs;
mod utilities;

pub mod co;

pub mod decl {
	pub use super::aliases::*;
	pub use super::enums::*;
	pub use super::funcs::*;
	pub use super::handles::decl::*;
	pub use super::structs::*;
	pub use super::utilities::*;
}

pub mod traits {
	pub use super::co_traits::*;
	pub use super::handles::traits::*;
}
