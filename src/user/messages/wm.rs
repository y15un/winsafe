//! Generic window
//! [messages](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-messages-and-message-queues),
//! whose constants have [`WM`](crate::co::WM) prefix.

use crate::co;
use crate::kernel::decl::{HIWORD, LOWORD, MAKEDWORD};
use crate::msg::WndMsg;
use crate::prelude::{MsgSend, MsgSendRecv};
use crate::user::decl::{AccelMenuCtrl, AccelMenuCtrlData, CREATESTRUCT,
	DELETEITEMSTRUCT, HDC, HELPINFO, HICON, HMENU, HWND, HwndFocus, HwndHmenu,
	HwndPointId, MINMAXINFO, MSG, NccspRect, POINT, RECT, SIZE, STYLESTRUCT,
	TIMERPROC, TITLEBARINFOEX, WINDOWPOS};
use crate::user::privs::{CB_ERR, FAPPCOMMAND_MASK, LB_ERRSPACE, zero_as_none};

/// [`WM_ACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-activate)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Activate {
	pub event: co::WA,
	pub is_minimized: bool,
	pub hwnd: HWND,
}

impl MsgSend for Activate {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATE,
			wparam: MAKEDWORD(self.event.0, self.is_minimized as _) as _,
			lparam: self.hwnd.0 as _,
		}
	}
}

impl MsgSendRecv for Activate {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: co::WA(LOWORD(p.wparam as _)),
			is_minimized: HIWORD(p.wparam as _) != 0,
			hwnd: HWND(p.lparam as _),
		}
	}
}

/// [`WM_ACTIVATEAPP`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ActivateApp {
	pub is_being_activated: bool,
	pub thread_id: u32,
}

impl MsgSend for ActivateApp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ACTIVATEAPP,
			wparam: self.is_being_activated as _,
			lparam: self.thread_id as _,
		}
	}
}

impl MsgSendRecv for ActivateApp {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_being_activated: p.wparam != 0,
			thread_id: p.lparam as _,
		}
	}
}

/// [`WM_APPCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct AppCommand {
	pub hwnd_owner: HWND,
	pub app_command: co::APPCOMMAND,
	pub u_device: co::FAPPCOMMAND,
	pub keys: co::MK,
}

impl MsgSend for AppCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::APPCOMMAND,
			wparam: self.hwnd_owner.0 as _,
			lparam: MAKEDWORD(self.keys.into(), self.app_command.0 | self.u_device.0) as _,
		}
	}
}

impl MsgSendRecv for AppCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_owner: HWND(p.wparam as _),
			app_command: co::APPCOMMAND(HIWORD(p.lparam as _) & !FAPPCOMMAND_MASK),
			u_device: co::FAPPCOMMAND(HIWORD(p.lparam as _) & FAPPCOMMAND_MASK),
			keys: co::MK(LOWORD(p.lparam as _)),
		}
	}
}

pub_struct_msg_empty_handleable! { CancelMode: co::WM::CANCELMODE; "user";
	/// [`WM_CANCELMODE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode)
}

/// [`WM_CAPTURECHANGED`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct CaptureChanged {
	pub hwnd_gaining_mouse: HWND,
}

impl MsgSend for CaptureChanged {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CAPTURECHANGED,
			wparam: self.hwnd_gaining_mouse.0 as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for CaptureChanged {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_gaining_mouse: HWND(p.wparam as _),
		}
	}
}

pub_struct_msg_empty_handleable! { ChildActivate: co::WM::CHILDACTIVATE; "user";
	/// [`WM_CHILDACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate)
}

pub_struct_msg_empty_handleable! { Close: co::WM::CLOSE; "user";
	/// [`WM_CLOSE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close)
}

pub_struct_msg_char! { Char: co::WM::CHAR; "user";
	/// [`WM_CHAR`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-char)
}

/// [`WM_COMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Command {
	pub event: AccelMenuCtrl,
}

impl MsgSend for Command {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::COMMAND,
			wparam: match self.event {
				AccelMenuCtrl::Accel(id) => MAKEDWORD(id, 1) as _,
				AccelMenuCtrl::Menu(id) => MAKEDWORD(id, 0) as _,
				AccelMenuCtrl::Ctrl(data) => MAKEDWORD(data.ctrl_id, data.notif_code.0) as _,
			},
			lparam: match self.event {
				AccelMenuCtrl::Accel(_) => co::CMD::Accelerator.0 as _,
				AccelMenuCtrl::Menu(_) => co::CMD::Menu.0 as _,
				AccelMenuCtrl::Ctrl(data) => data.ctrl_hwnd.0 as _,
			},
		}
	}
}

impl MsgSendRecv for Command {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			event: match HIWORD(p.wparam as _) {
				1 => AccelMenuCtrl::Accel(LOWORD(p.wparam as _)),
				0 => AccelMenuCtrl::Menu(LOWORD(p.wparam as _)),
				code => AccelMenuCtrl::Ctrl(
					AccelMenuCtrlData {
						notif_code: co::CMD(code),
						ctrl_id: LOWORD(p.wparam as _),
						ctrl_hwnd: HWND(p.lparam as _),
					},
				),
			},
		}
	}
}

/// [`WM_CONTEXTMENU`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ContextMenu {
	pub hwnd: HWND,
	pub cursor_pos: POINT,
}

impl MsgSend for ContextMenu {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CONTEXTMENU,
			wparam: self.hwnd.0 as _,
			lparam: self.cursor_pos.into_u32() as _,
		}
	}
}

impl MsgSendRecv for ContextMenu {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: HWND(p.wparam as _),
			cursor_pos: POINT::from_u32(p.lparam as _),
		}
	}
}

/// [`WM_CREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create)
/// message parameters.
///
/// Return type: `i32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Create<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for Create<'a, 'b, 'c> {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::CREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for Create<'a, 'b, 'c> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-deleteitem)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct DeleteItem<'a> {
	pub control_id: u16,
	pub deleteitemstruct: &'a DELETEITEMSTRUCT,
}

impl<'a> MsgSend for DeleteItem<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::DELETEITEM,
			wparam: self.control_id as _,
			lparam: self.deleteitemstruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for DeleteItem<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			control_id: p.wparam as _,
			deleteitemstruct: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

pub_struct_msg_empty_handleable! { Destroy: co::WM::DESTROY; "user";
	/// [`WM_DESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
}

/// [`WM_ENABLE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-enable)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Enable {
	pub has_been_enabled: bool,
}

impl MsgSend for Enable {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENABLE,
			wparam: self.has_been_enabled as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for Enable {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			has_been_enabled: p.wparam != 0,
		}
	}
}

/// [`WM_ENDSESSION`](https://docs.microsoft.com/en-us/windows/win32/shutdown/wm-endsession)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct EndSession {
	pub is_session_being_ended: bool,
	pub event: co::ENDSESSION,
}

impl MsgSend for EndSession {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENDSESSION,
			wparam: self.is_session_being_ended as _,
			lparam: self.event.0 as _,
		}
	}
}

impl MsgSendRecv for EndSession {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_session_being_ended: p.wparam != 0,
			event: co::ENDSESSION(p.lparam as _),
		}
	}
}

/// [`WM_ENTERIDLE`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct EnterIdle {
	pub reason: co::MSGF,
	pub handle: HwndHmenu,
}

impl MsgSend for EnterIdle {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERIDLE,
			wparam: self.reason.0 as _,
			lparam: self.handle.as_isize(),
		}
	}
}

impl MsgSendRecv for EnterIdle {
	fn from_generic_wm(p: WndMsg) -> Self {
		let reason = co::MSGF(p.wparam as _);
		Self {
			reason,
			handle: match reason {
				co::MSGF::DIALOGBOX => HwndHmenu::Hwnd(HWND(p.lparam as _)),
				_ => HwndHmenu::Hmenu(HMENU(p.lparam as _)),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { EnterSizeMove: co::WM::ENTERSIZEMOVE; "user";
	/// [`WM_ENTERSIZEMOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove)
}

/// [`WM_ENTERMENULOOP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct EnterMenuLoop {
	pub with_trackpopupmenu: bool,
}

impl MsgSend for EnterMenuLoop {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ENTERMENULOOP,
			wparam: self.with_trackpopupmenu as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for EnterMenuLoop {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			with_trackpopupmenu: p.wparam != 0,
		}
	}
}

/// [`WM_ERASEBKGND`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd)
/// message parameters.
///
/// Return type: `i32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct EraseBkgnd {
	pub hdc: HDC,
}

impl MsgSend for EraseBkgnd {
	type RetType = i32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::ERASEBKGND,
			wparam: self.hdc.0 as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for EraseBkgnd {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hdc: HDC(p.wparam as _),
		}
	}
}

/// [`WM_EXITMENULOOP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ExitMenuLoop {
	pub is_shortcut: bool,
}

impl MsgSend for ExitMenuLoop {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::EXITMENULOOP,
			wparam: self.is_shortcut as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for ExitMenuLoop {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			is_shortcut: p.wparam != 0,
		}
	}
}

pub_struct_msg_empty_handleable! { ExitSizeMove: co::WM::EXITSIZEMOVE; "user";
	/// [`WM_EXITSIZEMOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)
}

/// [`WM_GETDLGCODE`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode)
/// message parameters.
///
/// Return type: `co::DLGC`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetDlgCode<'a> {
	pub vkey_code: co::VK,
	pub msg: Option<&'a mut MSG>,
}

impl<'a> MsgSend for GetDlgCode<'a> {
	type RetType = co::DLGC;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::DLGC(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETDLGCODE,
			wparam: self.vkey_code.0 as _,
			lparam: self.msg.as_mut().map_or(0, |m| m as *mut _ as _),
		}
	}
}

impl<'a> MsgSendRecv for GetDlgCode<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			vkey_code: co::VK(p.wparam as _),
			msg: match p.lparam {
				0 => None,
				ptr => Some(unsafe { &mut *(ptr as *mut _) })
			},
		}
	}
}

/// [`WM_GETHMENU`](https://docs.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu)
/// message, which has no parameters. Originally has `MN` prefix.
///
/// Return type: `Option<HMENU>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetHMenu {}

impl MsgSend for GetHMenu {
	type RetType = Option<HMENU>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HMENU(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MN_GETHMENU,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetHMenu {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETMINMAXINFO`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetMinMaxInfo<'a> {
	pub info: &'a mut MINMAXINFO,
}

impl<'a> MsgSend for GetMinMaxInfo<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETMINMAXINFO,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for GetMinMaxInfo<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_GETTEXT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-gettext)
/// message parameters.
///
/// Return type: `u32`.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{msg::wm, HWND, WString};
///
/// let hwnd: HWND; // initialized somewhere
/// # let hwnd = HWND::NULL;
///
/// let needed_len = hwnd.SendMessage(wm::GetTextLength {});
/// let mut buf = WString::new_alloc_buffer(needed_len as _);
///
/// hwnd.SendMessage(wm::GetText {
///     buffer: buf.as_mut_slice(),
/// });
///
/// println!("Text: {}", buf.to_string());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetText<'a> {
	pub buffer: &'a mut [u16], // can't be WString because this message can be received
}

impl<'a> MsgSend for GetText<'a> {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTEXT,
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

impl<'a> MsgSendRecv for GetText<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			buffer: unsafe { std::slice::from_raw_parts_mut(p.lparam as _, p.wparam) },
		}
	}
}

/// [`WM_GETTEXTLENGTH`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetTextLength {}

impl MsgSend for GetTextLength {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTEXTLENGTH,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for GetTextLength {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_GETTITLEBARINFOEX`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetTitleBarInfoEx<'a> {
	pub info: &'a mut TITLEBARINFOEX,
}

impl<'a> MsgSend for GetTitleBarInfoEx<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::GETTITLEBARINFOEX,
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for GetTitleBarInfoEx<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			info: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HELP`](https://docs.microsoft.com/en-us/windows/win32/shell/wm-help)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Help<'a> {
	pub helpinfo: &'a HELPINFO,
}

impl<'a> MsgSend for Help<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HELP,
			wparam: 0,
			lparam: self.helpinfo as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Help<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			helpinfo: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_HSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-hscroll)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct HScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

impl MsgSend for HScroll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::HSCROLL,
			wparam: MAKEDWORD(self.request.0, self.scroll_box_pos) as _,
			lparam: self.hcontrol.map_or(0, |h| h.0 as _),
		}
	}
}

impl MsgSendRecv for HScroll {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: co::SB_REQ(LOWORD(p.wparam as _)),
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(HWND(ptr as _)),
			},
		}
	}
}

/// [`WM_INITDIALOG`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct InitDialog {
	pub hwnd_focus: HWND,
	pub additional_data: isize,
}

impl MsgSend for InitDialog {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITDIALOG,
			wparam: self.hwnd_focus.0 as _,
			lparam: self.additional_data,
		}
	}
}

impl MsgSendRecv for InitDialog {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: HWND(p.wparam as _),
			additional_data: p.lparam,
		}
	}
}

/// [`WM_INITMENUPOPUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct InitMenuPopup {
	pub hmenu: HMENU,
	pub item_pos: u16,
	pub is_window_menu: bool,
}

impl MsgSend for InitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::INITMENUPOPUP,
			wparam: self.hmenu.0 as _,
			lparam: MAKEDWORD(self.item_pos, self.is_window_menu as _) as _,
		}
	}
}

impl MsgSendRecv for InitMenuPopup {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: HMENU(p.wparam as _),
			item_pos: LOWORD(p.lparam as _),
			is_window_menu: HIWORD(p.lparam as _) != 0,
		}
	}
}

pub_struct_msg_char! { KeyDown: co::WM::KEYDOWN; "user";
	/// [`WM_KEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keydown)
}

pub_struct_msg_char! { KeyUp: co::WM::KEYUP; "user";
	/// [`WM_KEYUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keyup)
}

/// [`WM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct KillFocus {
	pub hwnd: Option<HWND>,
}

impl MsgSend for KillFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::KILLFOCUS,
			wparam: self.hwnd.map_or(0, |h| h.0 as _),
			lparam: 0,
		}
	}
}

impl MsgSendRecv for KillFocus {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: match p.wparam {
				0 => None,
				ptr => Some(HWND(ptr as _)),
			},
		}
	}
}

pub_struct_msg_button! { LButtonDblClk: co::WM::LBUTTONDBLCLK; "user";
	/// [`WM_LBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk)
}

pub_struct_msg_button! { LButtonDown: co::WM::LBUTTONDOWN; "user";
	/// [`WM_LBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
}

pub_struct_msg_button! { LButtonUp: co::WM::LBUTTONUP; "user";
	/// [`WM_LBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup)
}

pub_struct_msg_button! { MButtonDblClk: co::WM::MBUTTONDBLCLK; "user";
	/// [`WM_MBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk)
}

pub_struct_msg_button! { MButtonDown: co::WM::MBUTTONDOWN; "user";
	/// [`WM_MBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown)
}

pub_struct_msg_button! { MButtonUp: co::WM::MBUTTONUP; "user";
	/// [`WM_MBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup)
}

/// [`WM_MENUCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menucommand)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct MenuCommand {
	pub item_index: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUCOMMAND,
			wparam: self.item_index as _,
			lparam: self.hmenu.0 as _,
		}
	}
}

impl MsgSendRecv for MenuCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			item_index: p.wparam as _,
			hmenu: HMENU(p.lparam as _),
		}
	}
}

/// [`WM_MENUDRAG`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menudrag)
/// message parameters.
///
/// Return type: `co::MND`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct MenuDrag {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuDrag {
	type RetType = co::MND;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::MND(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENUDRAG,
			wparam: self.position as _,
			lparam: self.hmenu.0 as _,
		}
	}
}

impl MsgSendRecv for MenuDrag {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: HMENU(p.lparam as _),
		}
	}
}

/// [`WM_MENURBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct MenuRButtonUp {
	pub position: u32,
	pub hmenu: HMENU,
}

impl MsgSend for MenuRButtonUp {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MENURBUTTONUP,
			wparam: self.position as _,
			lparam: self.hmenu.0 as _,
		}
	}
}

impl MsgSendRecv for MenuRButtonUp {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			position: p.wparam as _,
			hmenu: HMENU(p.lparam as _),
		}
	}
}

pub_struct_msg_button! { MouseHover: co::WM::MOUSEHOVER; "user";
	/// [`WM_MOUSEHOVER`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover)
}

pub_struct_msg_empty_handleable! { MouseLeave: co::WM::MOUSELEAVE; "user";
	/// [`WM_MOUSELEAVE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave)
}

pub_struct_msg_button! { MouseMove: co::WM::MOUSEMOVE; "user";
	/// [`WM_MOUSEMOVE`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove)
}

/// [`WM_MOVE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-move)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Move {
	pub coords: POINT,
}

impl MsgSend for Move {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVE,
			wparam: 0,
			lparam: self.coords.into_u32() as _,
		}
	}
}

impl MsgSendRecv for Move {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			coords: POINT::from_u32(p.lparam as _),
		}
	}
}

/// [`WM_MOVING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-moving)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Moving<'a> {
	pub window_pos: &'a mut RECT,
}

impl<'a> MsgSend for Moving<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::MOVING,
			wparam: 0,
			lparam: self.window_pos as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Moving<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_pos: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_NCCALCSIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize)
/// message parameters.
///
/// Return type: `co::WVR`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct NcCalcSize<'a, 'b> {
	pub data: NccspRect<'a, 'b>,
}

impl<'a, 'b> MsgSend for NcCalcSize<'a, 'b> {
	type RetType = co::WVR;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::WVR(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCALCSIZE,
			wparam: match &self.data {
				NccspRect::Nccsp(_) => true as _,
				NccspRect::Rect(_) => false as _,
			},
			lparam: match &self.data {
				NccspRect::Nccsp(nccalc) => *nccalc as *const _ as _,
				NccspRect::Rect(rc) => *rc as *const _ as _,
			},
		}
	}
}

impl<'a, 'b> MsgSendRecv for NcCalcSize<'a, 'b> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			data: match p.wparam {
				0 => NccspRect::Rect(unsafe { &mut *(p.lparam as *mut _) }),
				_ => NccspRect::Nccsp(unsafe { &mut *(p.lparam as *mut _) }),
			},
		}
	}
}

/// [`WM_NCCREATE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct NcCreate<'a, 'b, 'c> {
	pub createstruct: &'c CREATESTRUCT<'a, 'b>,
}

impl<'a, 'b, 'c> MsgSend for NcCreate<'a, 'b, 'c> {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCCREATE,
			wparam: 0,
			lparam: self.createstruct as *const _ as _,
		}
	}
}

impl<'a, 'b, 'c> MsgSendRecv for NcCreate<'a, 'b, 'c> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			createstruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_empty_handleable! { NcDestroy: co::WM::NCDESTROY; "user";
	/// [`WM_NCDESTROY`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy)
}

/// [`WM_NCHITTEST`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
/// message parameters.
///
/// Return type: `co::HT`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct NcHitTest{
	pub cursor_pos: POINT,
}

impl MsgSend for NcHitTest {
	type RetType = co::HT;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::HT(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NCHITTEST,
			wparam: 0,
			lparam: self.cursor_pos.into_u32() as _,
		}
	}
}

impl MsgSendRecv for NcHitTest {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			cursor_pos: POINT::from_u32(p.lparam as _),
		}
	}
}

/// [`WM_NEXTDLGCTL`](https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct NextDlgCtl {
	pub hwnd_focus: HwndFocus,
}

impl MsgSend for NextDlgCtl {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::NEXTDLGCTL,
			wparam: match self.hwnd_focus {
				HwndFocus::Hwnd(hctl) => hctl.0 as _,
				HwndFocus::FocusNext(next) => if next { 0 } else { 1 },
			},
			lparam: MAKEDWORD(match self.hwnd_focus {
				HwndFocus::Hwnd(_) => 1,
				HwndFocus::FocusNext(_) => 0,
			}, 0) as _,
		}
	}
}

impl MsgSendRecv for NextDlgCtl {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_focus: match p.wparam {
				1 => HwndFocus::Hwnd(HWND(p.wparam as _)),
				_ => HwndFocus::FocusNext(p.wparam == 0),
			},
		}
	}
}

pub_struct_msg_empty_handleable! { Null: co::WM::NULL; "user";
	/// [`WM_NULL`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-null)
}

/// [`WM_PARENTNOTIFY`](https://docs.microsoft.com/en-us/windows/win32/inputmsg/wm-parentnotify)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ParentNotify {
	pub event: co::WMPN,
	pub child_id: u16,
	pub data: HwndPointId,
}

impl MsgSend for ParentNotify {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::PARENTNOTIFY,
			wparam: MAKEDWORD(self.event.0, self.child_id) as _,
			lparam: self.data.as_isize(),
		}
	}
}

impl MsgSendRecv for ParentNotify {
	fn from_generic_wm(p: WndMsg) -> Self {
		let event = co::WMPN(LOWORD(p.wparam as _));
		Self {
			event,
			child_id: HIWORD(p.wparam as _),
			data: match event {
				co::WMPN::CREATE | co::WMPN::DESTROY => HwndPointId::Hwnd(HWND(p.lparam as _)),
				co::WMPN::POINTERDOWN => HwndPointId::Id(p.lparam as _),
				_ => HwndPointId::Point(POINT::from_u32(p.lparam as _)),
			},
		}
	}
}

/// [`WM_QUERYOPEN`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct QueryOpen {}

impl MsgSend for QueryOpen {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::QUERYOPEN,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for QueryOpen {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

pub_struct_msg_button! { RButtonDblClk: co::WM::RBUTTONDBLCLK; "user";
	/// [`WM_RBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk)
}

pub_struct_msg_button! { RButtonDown: co::WM::RBUTTONDOWN; "user";
	/// [`WM_RBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown)
}

pub_struct_msg_button! { RButtonUp: co::WM::RBUTTONUP; "user";
	/// [`WM_RBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup)
}

/// [`WM_SETCURSOR`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-setcursor)
/// message parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetCursor {
	pub hwnd: HWND,
	pub hit_test: co::HT,
	pub mouse_msg: u16,
}

impl MsgSend for SetCursor {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETCURSOR,
			wparam: self.hwnd.0 as _,
			lparam: MAKEDWORD(self.hit_test.0, self.mouse_msg) as _,
		}
	}
}

impl MsgSendRecv for SetCursor {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd: HWND(p.wparam as _),
			hit_test: co::HT(LOWORD(p.lparam as _)),
			mouse_msg: HIWORD(p.lparam as _),
		}
	}
}

/// [`WM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetFocus {
	pub hwnd_losing_focus: HWND,
}

impl MsgSend for SetFocus {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETFOCUS,
			wparam: self.hwnd_losing_focus.0 as _,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for SetFocus {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hwnd_losing_focus: HWND(p.wparam as _),
		}
	}
}

/// [`WM_SETICON`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-seticon)
/// message parameters.
///
/// Return type: `Option<HICON>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetIcon {
	pub size: co::ICON_SZ,
	pub hicon: HICON,
}

impl MsgSend for SetIcon {
	type RetType = Option<HICON>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| HICON(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETICON,
			wparam: self.size.0 as _,
			lparam: self.hicon.0 as _,
		}
	}
}

impl MsgSendRecv for SetIcon {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			size: co::ICON_SZ(p.wparam as _),
			hicon: HICON(p.lparam as _),
		}
	}
}

/// [`WM_SETTEXT`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-settext)
/// message parameters.
///
/// Return type: `bool`.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{msg::wm, HWND, WString};
///
/// let hwnd: HWND; // initialized somewhere
/// # let hwnd = HWND::NULL;
///
/// let new_text = WString::from_str("some text");
///
/// hwnd.SendMessage(wm::SetText {
///     text: unsafe { new_text.as_ptr() },
/// });
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetText {
	pub text: *const u16, // can't be WString because this message can be received
}

impl MsgSend for SetText {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			0 | LB_ERRSPACE | CB_ERR => false, // CB_ERRSPACE is equal to LB_ERRSPACE
			_ => true,
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SETTEXT,
			wparam: 0,
			lparam: self.text as _,
		}
	}
}

impl MsgSendRecv for SetText {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			text: p.lparam as _,
		}
	}
}

/// [`WM_SHOWWINDOW`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ShowWindow {
	pub being_shown: bool,
	pub status: co::SW_S,
}

impl MsgSend for ShowWindow {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SHOWWINDOW,
			wparam: self.being_shown as _,
			lparam: self.status.0 as _,
		}
	}
}

impl MsgSendRecv for ShowWindow {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			being_shown: p.wparam != 0,
			status: co::SW_S(p.lparam as _),
		}
	}
}

/// [`WM_SIZE`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Size {
	pub request: co::SIZE_R,
	pub client_area: SIZE,
}

impl MsgSend for Size {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZE,
			wparam: self.request.0 as _,
			lparam: self.client_area.into_u32() as _,
		}
	}
}

impl MsgSendRecv for Size {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: co::SIZE_R(p.wparam as _),
			client_area: SIZE::new(
				LOWORD(p.lparam as _) as _,
				HIWORD(p.lparam as _) as _,
			),
		}
	}
}

/// [`WM_SIZING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-sizing)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Sizing<'a> {
	pub window_edge: co::WMSZ,
	pub coords: &'a mut RECT,
}

impl<'a> MsgSend for Sizing<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SIZING,
			wparam: self.window_edge.0 as _,
			lparam: self.coords as *mut _ as _,
		}
	}
}

impl<'a> MsgSendRecv for Sizing<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			window_edge: co::WMSZ(p.wparam as _),
			coords: unsafe { &mut *(p.lparam as *mut _) },
		}
	}
}

/// [`WM_STYLECHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct StyleChanged<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for StyleChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGED,
			wparam: self.change.0 as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for StyleChanged<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		let change = co::GWL_C(p.wparam as _);
		Self {
			change,
			stylestruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_STYLECHANGING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct StyleChanging<'a> {
	pub change: co::GWL_C,
	pub stylestruct: &'a STYLESTRUCT,
}

impl<'a> MsgSend for StyleChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::STYLECHANGING,
			wparam: self.change.0 as _,
			lparam: self.stylestruct as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for StyleChanging<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		let change = co::GWL_C(p.wparam as _);
		Self {
			change,
			stylestruct: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_char! { SysChar: co::WM::SYSCHAR; "user";
	/// [`WM_SYSCHAR`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syschar)
}

/// [`WM_SYSCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syscommand)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SysCommand {
	pub request: co::SC,
	pub position: POINT,
}

impl MsgSend for SysCommand {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::SYSCOMMAND,
			wparam: self.request.0 as _,
			lparam: self.position.into_u32() as _,
		}
	}
}

impl MsgSendRecv for SysCommand {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			request: co::SC(p.wparam as _),
			position: POINT::from_u32(p.lparam as _),
		}
	}
}

pub_struct_msg_char! { SysDeadChar: co::WM::SYSDEADCHAR; "user";
	/// [`WM_SYSDEADCHAR`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar)
}

pub_struct_msg_char! { SysKeyDown: co::WM::SYSKEYDOWN; "user";
	/// [`WM_SYSKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown)
}

pub_struct_msg_char! { SysKeyUp: co::WM::SYSKEYUP; "user";
	/// [`WM_SYSKEYUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup)
}

pub_struct_msg_empty_handleable! { ThemeChanged: co::WM::THEMECHANGED; "user";
	/// [`WM_THEMECHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged)
}

/// [`WM_TIMER`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-timer)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Timer {
	pub timer_id: u32,
	pub timer_proc: Option<TIMERPROC>,
}

impl MsgSend for Timer {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::TIMER,
			wparam: self.timer_id as _,
			lparam: self.timer_proc.map_or(0, |proc| proc as _),
		}
	}
}

impl MsgSendRecv for Timer {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			timer_id: p.wparam as _,
			timer_proc: match p.lparam {
				0 => None,
				addr => unsafe { std::mem::transmute(addr) },
			},
		}
	}
}

/// [`WM_UNINITMENUPOPUP`](https://docs.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct UninitMenuPopup {
	pub hmenu: HMENU,
	pub which: co::MF,
}

impl MsgSend for UninitMenuPopup {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNINITMENUPOPUP,
			wparam: self.hmenu.0 as _,
			lparam: MAKEDWORD(0, self.which.0 as _) as _,
		}
	}
}

impl MsgSendRecv for UninitMenuPopup {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			hmenu: HMENU(p.wparam as _),
			which: co::MF(LOWORD(p.lparam as _) as _),
		}
	}
}

/// [`WM_UNDO`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-undo)
/// message, which has no parameters.
///
/// Return type: `bool`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Undo {}

impl MsgSend for Undo {
	type RetType = bool;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::UNDO,
			wparam: 0,
			lparam: 0,
		}
	}
}

impl MsgSendRecv for Undo {
	fn from_generic_wm(_: WndMsg) -> Self {
		Self {}
	}
}

/// [`WM_VSCROLL`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-vscroll)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct VScroll {
	pub scroll_box_pos: u16,
	pub request: co::SB_REQ,
	pub hcontrol: Option<HWND>,
}

impl MsgSend for VScroll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::VSCROLL,
			wparam: MAKEDWORD(self.request.0, self.scroll_box_pos) as _,
			lparam: self.hcontrol.map_or(0, |h| h.0 as _),
		}
	}
}

impl MsgSendRecv for VScroll {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			scroll_box_pos: HIWORD(p.wparam as _),
			request: co::SB_REQ(LOWORD(p.wparam as _)),
			hcontrol: match p.lparam {
				0 => None,
				ptr => Some(HWND(ptr as _)),
			},
		}
	}
}

/// [`WM_WINDOWPOSCHANGED`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct WindowPosChanged<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WindowPosChanged<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGED,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WindowPosChanged<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

/// [`WM_WINDOWPOSCHANGING`](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct WindowPosChanging<'a> {
	pub windowpos: &'a WINDOWPOS,
}

impl<'a> MsgSend for WindowPosChanging<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::WM::WINDOWPOSCHANGING,
			wparam: 0,
			lparam: self.windowpos as *const _ as _,
		}
	}
}

impl<'a> MsgSendRecv for WindowPosChanging<'a> {
	fn from_generic_wm(p: WndMsg) -> Self {
		Self {
			windowpos: unsafe { &*(p.lparam as *const _) },
		}
	}
}

pub_struct_msg_button! { XButtonDblClk: co::WM::XBUTTONDBLCLK; "user";
	/// [`WM_XBUTTONDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk)
}

pub_struct_msg_button! { XButtonDown: co::WM::XBUTTONDOWN; "user";
	/// [`WM_XBUTTONDOWN`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown)
}

pub_struct_msg_button! { XButtonUp: co::WM::XBUTTONUP; "user";
	/// [`WM_XBUTTONUP`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup)
}
