use crate::co;
use crate::prelude::{MsgSend, MsgSendRecv};

/// Generic
/// [window message](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues)
/// parameters: `WPARAM` and `LPARAM`.
///
/// All message types can be converted to `WndMsg` via the
/// [`as_generic_wm`](crate::prelude::MsgSend::as_generic_wm) method.
///
/// Return type: `isize`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Copy, Clone)]
pub struct WndMsg {
	/// The [`co::WM`](crate::co::WM) constant that identifies the window
	/// message.
	pub msg_id: co::WM,
	/// First message parameter.
	pub wparam: usize,
	/// Second message parameter.
	pub lparam: isize,
}

impl MsgSend for WndMsg {
	type RetType = isize;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> Self {
		*self
	}
}

impl MsgSendRecv for WndMsg {
	fn from_generic_wm(p: Self) -> Self {
		p
	}
}
