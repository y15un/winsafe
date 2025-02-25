use crate::co;
use crate::gdi::decl::HFONT;
use crate::kernel::decl::WinResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::privs::zero_as_err;

/// [`DTM_SETMCFONT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-setmcfont)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "gdi"))))]
pub struct SetMcFont {
	pub hfont: HFONT,
	pub redraw: bool,
}

impl MsgSend for SetMcFont {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::SETMCFONT.into(),
			wparam: self.hfont.0 as _,
			lparam: self.redraw as _,
		}
	}
}

/// [`DTM_GETMCFONT`](https://docs.microsoft.com/en-us/windows/win32/controls/dtm-getmcfont)
/// message, which has no parameters.
///
/// Return type: `WinResult<HFONT>`.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "gdi"))))]
pub struct GetMcFont {}

impl MsgSend for GetMcFont {
	type RetType = WinResult<HFONT>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HFONT(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::DTM::GETMCFONT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}
