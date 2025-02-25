#![allow(non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{FILETIME, GetLastError, SECURITY_ATTRIBUTES,
	WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::{Handle, HandleClose};

impl_handle! { HTHREAD: "kernel";
	/// Handle to a
	/// [thread](https://docs.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HTHREAD {}
impl KernelHthread for HTHREAD {}

/// [`HTHREAD`](crate::HTHREAD) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHthread: Handle {
	/// [`CreateThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
	/// static method.
	///
	/// Returns the thread handle and ID.
	///
	/// **Note:** Must be paired with an
	/// [`HTHREAD::CloseHandle`](crate::prelude::HandleClose::CloseHandle) call.
	fn CreateThread(
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		stack_size: u64,
		start_addr: *mut std::ffi::c_void,
		parameter: *mut std::ffi::c_void,
		flags: co::THREAD_CREATE) -> WinResult<(HTHREAD, u32)>
	{
		let mut thread_id = u32::default();
		unsafe {
			kernel::ffi::CreateThread(
				thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				stack_size,
				start_addr,
				parameter,
				flags.0,
				&mut thread_id,
			).as_mut()
		}.map(|ptr| (HTHREAD(ptr), thread_id))
			.ok_or_else(|| GetLastError())
	}

	/// [`ExitThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
	/// static method.
	fn ExitThread(exit_code: u32) {
		unsafe { kernel::ffi::ExitThread(exit_code) }
	}

	/// [`GetCurrentThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)
	/// static method.
	fn GetCurrentThread() -> HTHREAD {
		HTHREAD(unsafe { kernel::ffi::GetCurrentThread() })
	}

	/// [`GetExitCodeThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)
	/// method.
	fn GetExitCodeThread(self) -> WinResult<u32> {
		let mut exit_code = u32::default();
		bool_to_winresult(
			unsafe {
				kernel::ffi::GetExitCodeThread(self.as_ptr(), &mut exit_code)
			},
		).map(|_| exit_code)
	}

	/// [`GetProcessIdOfThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread)
	/// method.
	fn GetProcessIdOfThread(self) -> WinResult<u32> {
		match unsafe { kernel::ffi::GetProcessIdOfThread(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid)
	/// method.
	fn GetThreadId(self) -> WinResult<u32> {
		match unsafe { kernel::ffi::GetThreadId(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes)
	/// method.
	fn GetThreadTimes(self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel::ffi::GetThreadTimes(
					self.as_ptr(),
					creation as *mut _ as _,
					exit as *mut _ as _,
					kernel as *mut _ as _,
					user as *mut _ as _,
				)
			},
		)
	}
}
