//! Variant types needed for some Win32 functions.

#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{ATOM, HBITMAP, HMENU};
use crate::co;
use crate::Utf16;

/// Wraps a variant parameter.
///
/// Used in:
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `lpClassName`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
pub enum AtomStr<'a> {
	Atom(ATOM),
	Str(&'a str),
}

impl<'a> AtomStr<'a> {
	/// [`MAKEINTRESOURCE`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
	/// macro. Uses an external [`Utf16`](crate::Utf16) buffer to keep the
	/// string, if needed.
	pub fn MAKEINTRESOURCE(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			AtomStr::Str(name) => {
				*buf16 = Utf16::from_str(name); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			}
			AtomStr::Atom(atom) => atom.as_ptr(),
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in:
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `lpNewItem`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `lpNewItem`.
pub enum BitmapPtrStr<'a> {
	Bitmap(HBITMAP),
	Str(&'a str),
	Param(*const c_void),
}

impl<'a> BitmapPtrStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			BitmapPtrStr::Bitmap(hbmp) => unsafe { hbmp.as_ptr() as *const u16 },
			BitmapPtrStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
			BitmapPtrStr::Param(lp) => *lp as *const u16,
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in:
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `uIDNewItem`.
pub enum IdMenu {
	Id(i32),
	Menu(HMENU),
	None,
}

impl IdMenu {
	/// Converts the internal value to a pointer.
	pub fn as_ptr(&self) -> *const c_void {
		match self {
			IdMenu::Id(id) => *id as *const c_void,
			IdMenu::Menu(hMenu) => unsafe { hMenu.as_ptr() },
			IdMenu::None => std::ptr::null(),
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in:
/// * [`CheckMenuItem`](crate::HMENU::CheckMenuItem) `uIDCheckItem`;
/// * [`DeleteMenu`](crate::HMENU::DeleteMenu) `uPosition`;
/// * [`EnableMenuItem`](crate::HMENU::EnableMenuItem) `uIDEnableItem`;
/// * [`HiliteMenuItem`](crate::HMENU::HiliteMenuItem) `uIDHiliteItem`;
/// * [`InsertMenuItem`](crate::HMENU::InsertMenuItem) `item`;
/// * [`RemoveMenu`](crate::HMENU::RemoveMenu) `uPosition`;
/// * [`SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo) `item`.
pub enum IdPos {
	Id(i32),
	Pos(u32),
}

impl From<IdPos> for u32 {
	fn from(v: IdPos) -> u32 {
		match v {
			IdPos::Id(id) => id as u32,
			IdPos::Pos(pos) => pos,
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in:
/// * [`LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName`.
pub enum IdIdcStr<'a> {
	Id(i32),
	Idc(co::IDC),
	Str(&'a str),
}

impl<'a> IdIdcStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			IdIdcStr::Id(id) => *id as *const u16,
			IdIdcStr::Idc(idc) => usize::from(*idc) as *const u16,
			IdIdcStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
		}
	}
}

//------------------------------------------------------------------------------

/// Wraps a variant parameter.
///
/// Used in:
/// * [`LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`.
pub enum IdIdiStr<'a> {
	Id(i32),
	Idi(co::IDI),
	Str(&'a str),
}

impl<'a> IdIdiStr<'a> {
	/// Converts the internal value to a pointer. Uses an external
	/// [`Utf16`](crate::Utf16) buffer to keep the string, if needed.
	pub fn as_ptr(&self, buf16: &mut Utf16) -> *const u16 {
		match self {
			IdIdiStr::Id(id) => *id as *const u16,
			IdIdiStr::Idi(idi) => usize::from(*idi) as *const u16,
			IdIdiStr::Str(str) => {
				*buf16 = Utf16::from_str(str); // convert string into u16 array, keep in buffer
				unsafe { buf16.as_ptr() } // return pointer from buffer
			},
		}
	}
}