#![allow(unused_macros)]

/// Implements `Default` trait by zeroing all members.
macro_rules! impl_default {
	($name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				unsafe { std::mem::zeroed::<Self>() }
			}
		}
	};
}

/// Implements `Default` trait by zeroing all members. Also sets the size field
/// to struct size.
macro_rules! impl_default_with_size {
	($name:ident, $field:ident $(, $life:lifetime)*) => {
		impl<$($life),*> Default for $name<$($life),*> {
			fn default() -> Self {
				let mut obj = unsafe { std::mem::zeroed::<Self>() };
				obj.$field = std::mem::size_of::<Self>() as _;
				obj
			}
		}
	};
}

/// Implements getter and setter methods for the given `BOOL` member.
macro_rules! pub_fn_bool_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the bool field.
		pub fn $field(&self) -> bool {
			self.$field != 0
		}

		/// Sets the bool field.
		pub fn $setter(&mut self, val: bool) {
			self.$field = val as _
		}
	};
}

/// Implements getter and setter methods for the given resource ID field, stored
/// as `*mut u16`.
macro_rules! pub_fn_resource_id_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the resource ID field.
		pub fn $field(&self) -> u16 {
			self.$field as _
		}

		/// Sets the resource ID field.
		pub fn $setter(&mut self, val: u16) {
			self.$field = val as _;
		}
	};
}

/// Implements getter and setter methods for the given `*mut u16` member.
macro_rules! pub_fn_string_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident) => {
		/// Returns the string field, if any.
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }
				.map(|psz| crate::kernel::decl::WString::from_wchars_nullt(psz).to_string())
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut crate::kernel::decl::WString>) {
			self.$field = buf.map_or(std::ptr::null_mut(), |buf| unsafe { buf.as_mut_ptr() });
		}
	};
}

/// Implements getter and setter methods for the given `[u16; N]` member.
macro_rules! pub_fn_string_arr_get_set {
	($field:ident, $setter:ident) => {
		/// Returns the string field.
		pub fn $field(&self) -> String {
			crate::kernel::decl::WString::from_wchars_slice(&self.$field).to_string()
		}

		/// Sets the string field.
		pub fn $setter(&mut self, text: &str) {
			crate::kernel::decl::WString::from_str(text).copy_to_slice(&mut self.$field);
		}
	};
}

/// Implements getter and setter methods for the given `*mut 16` and `i32`
/// members, setting buffer and its size.
macro_rules! pub_fn_string_buf_get_set {
	($life:lifetime, $field:ident, $setter:ident, $cch:ident) => {
		/// Returns the string field.
		pub fn $field(&self) -> Option<String> {
			unsafe { self.$field.as_mut() }
				.map(|psz| crate::kernel::decl::WString::from_wchars_nullt(psz).to_string())
		}

		/// Sets the string field.
		pub fn $setter(&mut self, buf: Option<&$life mut crate::kernel::decl::WString>) {
			self.$cch = buf.as_ref().map_or(0, |buf| buf.buffer_size() as _);
			self.$field = buf.map_or(std::ptr::null_mut(), |buf| unsafe { buf.as_mut_ptr() });
		}
	};
}

/// Implements getter and setter methods for the given pointer member.
macro_rules! pub_fn_ptr_get_set {
	($life:lifetime, $field:ident, $setter:ident, $ty:ty) => {
		/// Returns the pointer field.
		pub fn $field(&self) -> Option<&$life mut $ty> {
			unsafe { self.$field.as_mut() }
		}

		/// Sets the pointer field.
		pub fn $setter(&mut self, obj: Option<&$life mut $ty>) {
			self.$field = obj.map_or(std::ptr::null_mut(), |obj| obj);
		}
	};
}

/// Implements getter and setter methods for the given array + size members,
/// setting buffer and its size.
macro_rules! pub_fn_array_buf_get_set {
	($life:lifetime, $field:ident, $setter:ident, $cch:ident, $ty:ty) => {
		/// Returns the array field.
		pub fn $field(&self) -> Option<&$life mut [$ty]> {
			unsafe {
				self.$field.as_mut()
					.map(|p| std::slice::from_raw_parts_mut(p, self.$cch as _))
			}
		}

		/// Sets the array field.
		pub fn $setter(&mut self, buf: Option<&$life mut [$ty]>) {
			match buf {
				Some(buf) => {
					self.$field = buf as *mut _ as _;
					self.$cch = buf.len() as _;
				},
				None => {
					self.$field = std::ptr::null_mut();
					self.$cch = 0;
				},
			}
		}
	};
}
