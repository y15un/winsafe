use crate::co;
use crate::comctl::decl::DATETIMEPICKERINFO;
use crate::comctl::privs::GDT_ERROR;
use crate::kernel::decl::{SYSTEMTIME, WinResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{COLORREF, HWND, SIZE};
use crate::user::privs::zero_as_err;

pub_struct_msg_empty! { CloseMonthCal: co::DTM::CLOSEMONTHCAL.into(); "comctl";
	/// [`DTM_CLOSEMONTHCAL`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-closemonthcal)
}

/// [`DTM_GETDATETIMEPICKERINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getdatetimepickerinfo)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetDateTimePickerInfo<'a> {
	pub info: &'a mut DATETIMEPICKERINFO,
}

impl<'a> MsgSend for GetDateTimePickerInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETDATETIMEPICKERINFO.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`DTM_GETIDEALSIZE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getidealsize)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetIdealSize<'a> {
	pub size: &'a mut SIZE,
}

impl<'a> MsgSend for GetIdealSize<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETIDEALSIZE.into(),
			wparam: 0,
			lparam: self.size as *mut _ as _,
		}
	}
}

/// [`DTM_GETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMcColor {
	pub color_index: co::MCSC,
}

impl MsgSend for GetMcColor {
	type RetType = WinResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			rgb => Ok(COLORREF(rgb as _)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCCOLOR.into(),
			wparam: self.color_index.0 as _,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMCSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmcstyle)
/// message, which has no parameters.
///
/// Return type: `WinResult<co::MCS>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMcStyle {}

impl MsgSend for GetMcStyle {
	type RetType = WinResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|v| co::MCS(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETMONTHCAL`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmonthcal)
/// message, which has no parameters.
///
/// Return type: `WinResult<HWND>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetMonthCal {}

impl MsgSend for GetMonthCal {
	type RetType = WinResult<HWND>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HWND(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMONTHCAL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`DTM_GETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getrange)
/// message parameters.
///
/// Return type: `co::GDTR`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetRange<'a> {
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for GetRange<'a> {
	type RetType = co::GDTR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::GDTR(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETRANGE.into(),
			wparam: 0,
			lparam: self.system_times as *mut _ as _,
		}
	}
}

/// [`DTM_GETSYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getsystemtime)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetSystemTime<'a> {
	pub system_time: &'a mut SYSTEMTIME,
}

impl<'a> MsgSend for GetSystemTime<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const GDT_NONE: i32 = co::GDT::NONE.0 as _;
		match v as i32 {
			GDT_ERROR => Err(co::ERROR::BAD_ARGUMENTS),
			GDT_NONE => Err(co::ERROR::INVALID_DATA),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETSYSTEMTIME.into(),
			wparam: 0,
			lparam: self.system_time as *mut _ as _,
		}
	}
}

/// [`DTM_SETFORMAT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setformat)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetFormat {
	pub format_string: Option<WString>,
}

impl MsgSend for SetFormat {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETFORMAT.into(),
			wparam: 0,
			lparam: self.format_string.as_ref().map_or(0, |ws| unsafe { ws.as_ptr() } as _),
		}
	}
}

/// [`DTM_SETMCCOLOR`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmccolor)
/// message parameters.
///
/// Return type: `WinResult<COLORREF>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetMcColor {
	pub color_index: co::MCSC,
	pub color: COLORREF,
}

impl MsgSend for SetMcColor {
	type RetType = WinResult<COLORREF>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			rgb => Ok(COLORREF(rgb as _)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCCOLOR.into(),
			wparam: self.color_index.0 as _,
			lparam: self.color.0 as _,
		}
	}
}

/// [`DTM_SETMCSTYLE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmcstyle)
/// message parameters.
///
/// Return type: `WinResult<co::MCS>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetMcStyle {
	pub style: co::MCS,
}

impl MsgSend for SetMcStyle {
	type RetType = WinResult<co::MCS>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|v| co::MCS(v as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCSTYLE.into(),
			wparam: 0,
			lparam: self.style.0 as _,
		}
	}
}

/// [`DTM_SETRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setrange)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetRange<'a> {
	pub valid: co::GDTR,
	pub system_times: &'a mut [SYSTEMTIME; 2],
}

impl<'a> MsgSend for SetRange<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETRANGE.into(),
			wparam: self.valid.0 as _,
			lparam: self.system_times as *mut _ as _,
		}
	}
}

/// [`DTM_SETSYSTEMTIME`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setsystemtime)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetSystemTime<'a> {
	pub system_time: Option<&'a SYSTEMTIME>,
}

impl<'a> MsgSend for SetSystemTime<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETSYSTEMTIME.into(),
			wparam: self.system_time.as_ref().map_or(co::GDT::NONE.0, |_| co::GDT::VALID.0) as _,
			lparam: self.system_time.as_ref().map_or(0, |st| st as *const _ as _),
		}
	}
}
