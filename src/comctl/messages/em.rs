use crate::co;
use crate::comctl::decl::EDITBALLOONTIP;
use crate::kernel::decl::{WinResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::privs::zero_as_err;

/// [`EM_GETCUEBANNER`](https://docs.microsoft.com/en-us/windows/win32/controls/em-getcuebanner)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct GetCueBanner<'a> {
	pub buffer: &'a mut WString,
}

impl<'a> MsgSend for GetCueBanner<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 | 1 => Ok(()),
			_ => Err(co::ERROR::BAD_ARGUMENTS),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::GETCUEBANNER.into(),
			wparam: unsafe { self.buffer.as_mut_ptr() } as _,
			lparam: self.buffer.buffer_size() as _,
		}
	}
}

/// [`EM_HIDEBALLOONTIP`](https://docs.microsoft.com/en-us/windows/win32/controls/em-hideballoontip)
/// message, which has no parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HideBalloonTip {}

impl MsgSend for HideBalloonTip {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::HIDEBALLOONTIP.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`EM_SETCUEBANNER`](https://docs.microsoft.com/en-us/windows/win32/controls/em-setcuebanner)
/// message parameters..
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct SetCueBanner {
	pub show_even_with_focus: bool,
	pub text: WString,
}

impl MsgSend for SetCueBanner {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SETCUEBANNER.into(),
			wparam: self.show_even_with_focus as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`EM_SHOWBALLOONTIP`](https://docs.microsoft.com/en-us/windows/win32/controls/em-showballoontip)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ShowBalloonTip<'a, 'b, 'c> {
	pub info: &'c EDITBALLOONTIP<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for ShowBalloonTip<'a, 'b, 'c> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::EM::SHOWBALLOONTIP.into(),
			wparam: 0,
			lparam: self.info as *const _ as _,
		}
	}
}
