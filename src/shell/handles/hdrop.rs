#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::kernel::decl::{GetLastError, WinResult, WString};
use crate::kernel::privs::MAX_PATH;
use crate::prelude::Handle;
use crate::shell;
use crate::user::decl::POINT;

impl_handle! { HDROP: "shell";
	/// Handle to an
	/// [internal drop structure](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdrop).
}

impl ShellHdrop for HDROP {}

/// [`HDROP`](crate::HDROP) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellHdrop: Handle {
	/// Returns an iterator over the dropped files by calling
	/// [`HDROP::DragQueryFile`](crate::prelude::ShellHdrop::DragQueryFile)
	/// consecutively, then frees the handle by calling
	/// [`HDROP::DragFinish`](crate::prelude::ShellHdrop::DragFinish).
	///
	/// # Examples
	///
	/// Iterating over the strings:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HDROP;
	///
	/// let hdrop: HDROP; // initialized somewhere
	/// # let hdrop = HDROP::NULL;
	///
	/// for file_path in hdrop.iter()? {
	///     let file_path = file_path?;
	///     println!("File: {}", file_path);
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Collecting the strings into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HDROP, WinResult};
	///
	/// let hdrop: HDROP; // initialized somewhere
	/// # let hdrop = HDROP::NULL;
	///
	/// let file_paths = hdrop.iter()?
	///     .collect::<WinResult<Vec<_>>>()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn iter<'a>(&'a self) -> WinResult<Box<dyn Iterator<Item = WinResult<String>> + 'a>> {
		Ok(Box::new(DropsIter::new(HDROP(unsafe { self.as_ptr() }))?))
	}

	/// [`DragFinish`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragfinish)
	/// method.
	///
	/// Prefer using [`HDROP::iter`](crate::prelude::ShellHdrop::iter), which
	/// calls `DragFinish` automatically.
	fn DragFinish(self) {
		unsafe { shell::ffi::DragFinish(self.as_ptr()) }
	}

	/// [`DragQueryFile`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew)
	/// method.
	///
	/// This method is rather tricky, consider using
	/// [`HDROP::iter`](crate::prelude::ShellHdrop::iter).
	fn DragQueryFile(self,
		ifile: Option<u32>, buf: Option<&mut WString>) -> WinResult<u32>
	{
		let cch = buf.as_ref().map_or(0, |buf| buf.buffer_size());

		match unsafe {
			shell::ffi::DragQueryFileW(
				self.as_ptr(),
				ifile.unwrap_or(0xffff_ffff),
				buf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr()),
				cch as _,
			)
		} {
			0 => Err(GetLastError()),
			char_count => Ok(char_count),
		}
	}

	/// [`DragQueryPoint`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragquerypoint)
	/// method.
	///
	/// Returns the coordinates and whether the drop occurred in the client area
	/// of the window.
	fn DragQueryPoint(self) -> (POINT, bool) {
		let mut pt = POINT::default();
		let client_area = unsafe {
			shell::ffi::DragQueryPoint(self.as_ptr(), &mut pt as *mut _ as _)
		};
		(pt, client_area != 0)
	}
}

struct DropsIter<'a> {
	hdrop: HDROP,
	buffer: WString,
	count: u32,
	current: u32,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Drop for DropsIter<'a> {
	fn drop(&mut self) {
		self.hdrop.DragFinish();
	}
}

impl<'a> Iterator for DropsIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.hdrop
			.DragQueryFile(Some(self.current), Some(&mut self.buffer))
		{
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(_) => {
				self.current += 1;
				Some(Ok(self.buffer.to_string()))
			},
		}
	}
}

impl<'a> DropsIter<'a> {
	fn new(hdrop: HDROP) -> WinResult<Self> {
		Ok(Self {
			hdrop,
			buffer: WString::new_alloc_buffer(MAX_PATH + 1), // so we alloc just once
			count: hdrop.DragQueryFile(None, None)?,
			current: 0,
			owner_: PhantomData,
		})
	}
}
