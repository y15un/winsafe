//! List box control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-messages),
//! whose constants have [`LB`](crate::co::LB) prefix.

use crate::co;
use crate::kernel::decl::{HIWORD, LCID, LOWORD, MAKEDWORD, WinResult, WString};
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::{POINT, RECT};
use crate::user::privs::{LB_ERR, LB_ERRSPACE, zero_as_err};

/// [`LB_ADDFILE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-addfile)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct AddFile {
	pub text: WString,
}

impl MsgSend for AddFile {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ADDFILE.into(),
			wparam: 0,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`LB_ADDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-addstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct AddString {
	pub text: WString,
}

impl MsgSend for AddString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ADDSTRING.into(),
			wparam: 0,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`LB_DELETESTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-deletestring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct DeleteString {
	pub index: u32,
}

impl MsgSend for DeleteString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::DELETESTRING.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_DIR`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-dir)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct Dir {
	pub attributes: co::DDL,
	pub path: WString,
}

impl MsgSend for Dir {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::DIR.into(),
			wparam: self.attributes.0 as _,
			lparam: unsafe { self.path.as_ptr() } as _,
		}
	}
}

/// [`LB_FINDSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-findstring)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct FindString {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for FindString {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::FINDSTRING.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`LB_FINDSTRINGEXACT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-findstringexact)
/// message parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct FindStringExact {
	pub preceding_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for FindStringExact {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::FINDSTRINGEXACT.into(),
			wparam: self.preceding_index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`LB_GETANCHORINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getanchorindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetAnchorIndex {}

impl MsgSend for GetAnchorIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETANCHORINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCARETINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcaretindex)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetCaretIndex {}

impl MsgSend for GetCaretIndex {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCARETINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetCount {}

impl MsgSend for GetCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetCurSel {}

impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => None,
			idx => Some(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETHORIZONTALEXTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gethorizontalextent)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetHorizontalExtent {}

impl MsgSend for GetHorizontalExtent {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETHORIZONTALEXTENT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMDATA`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemdata)
/// message parameters.
///
/// Return type: `WinResult<isize>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetItemData {
	pub index: u32,
}

impl MsgSend for GetItemData {
	type RetType = WinResult<isize>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		const LB_ERR_ISIZE: isize = LB_ERR as _;
		match v {
			LB_ERR_ISIZE => Err(co::ERROR::BAD_ARGUMENTS),
			data => Ok(data),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMDATA.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemheight)
/// message parameters.
///
/// Return type: `WinResult<u8>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetItemHeight {
	pub index: Option<u32>,
}

impl MsgSend for GetItemHeight {
	type RetType = WinResult<u8>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			height => Ok(height as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMHEIGHT.into(),
			wparam: self.index.unwrap_or(0) as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETITEMRECT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getitemrect)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for GetItemRect<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`LB_GETLISTBOXINFO`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getlistboxinfo)
/// message, which has no parameters.
///
/// Return type: `u32`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetListBoxInfo {}

impl MsgSend for GetListBoxInfo {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETLISTBOXINFO.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETLOCALE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getlocale)
/// message, which has no parameters.
///
/// Return type: `LCID`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetLocale {}

impl MsgSend for GetLocale {
	type RetType = LCID;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		LCID(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETLOCALE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getsel)
/// message parameters.
///
/// Return type: `WinResult<bool>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetSel {
	pub index: u32,
}

impl MsgSend for GetSel {
	type RetType = WinResult<bool>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			status => Ok(status != 0),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSEL.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETSELCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getselcount)
/// message, which has no parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetSelCount {}

impl MsgSend for GetSelCount {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSELCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_GETSELITEMS`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-getselitems)
/// message parameters.
///
/// Return type `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetSelItems<'a> {
	pub buffer: &'a mut [u32],
}

impl<'a> MsgSend for GetSelItems<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			count => Ok(count as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETSELITEMS.into(),
			wparam: self.buffer.len(),
			lparam: self.buffer.as_mut_ptr() as _,
		}
	}
}

/// [`LB_GETTEXT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gettext)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetText<'a> {
	pub index: u32,
	pub text: &'a mut WString,
}

impl<'a> MsgSend for GetText<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTEXT.into(),
			wparam: self.index as _,
			lparam: unsafe { self.text.as_mut_ptr() } as _,
		}
	}
}

/// [`LB_GETTEXTLEN`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gettextlen)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetTextLen {
	pub index: u32,
}

impl MsgSend for GetTextLen {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			nchars => Ok(nchars as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTEXTLEN.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_GETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-gettopindex)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct GetTopIndex {}

impl MsgSend for GetTopIndex {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::GETTOPINDEX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`LB_INITSTORAGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-initstorage)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct InitStorage {
	pub num_items: u32,
	pub memory_bytes: u32,
}

impl MsgSend for InitStorage {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERRSPACE => Err(co::ERROR::BAD_ARGUMENTS),
			n_items => Ok(n_items as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::INITSTORAGE.into(),
			wparam: self.num_items as _,
			lparam: self.memory_bytes as _,
		}
	}
}

/// [`LB_INSERTSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-insertstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct InsertString {
	pub insertion_index: Option<u32>,
	pub text: WString,
}

impl MsgSend for InsertString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::INSERTSTRING.into(),
			wparam: self.insertion_index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.text.as_ptr() } as _,
		}
	}
}

/// [`LB_ITEMFROMPOINT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-itemfrompoint)
/// message parameters.
///
/// Return type: `(i32, bool)`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct ItemFromPoint {
	pub coords: POINT,
}

impl MsgSend for ItemFromPoint {
	type RetType = (i32, bool);

	fn convert_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _) as _, HIWORD(v as _) == 1)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::ITEMFROMPOINT.into(),
			wparam: 0,
			lparam: self.coords.into_u32() as _,
		}
	}
}

pub_struct_msg_empty! { ResetContent: co::LB::RESETCONTENT.into(); "user";
	/// [`LB_RESETCONTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-resetcontent)
}

/// [`LB_SELECTSTRING`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-selectstring)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SelectString {
	pub index: Option<u32>,
	pub prefix: WString,
}

impl MsgSend for SelectString {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELECTSTRING.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: unsafe { self.prefix.as_ptr() } as _,
		}
	}
}

/// [`LB_SELITEMRANGE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-selitemrange)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SelItemRange {
	pub select: bool,
	pub first_item: u16,
	pub last_item: u16,
}

impl MsgSend for SelItemRange {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELITEMRANGE.into(),
			wparam: self.select as _,
			lparam: MAKEDWORD(self.first_item, self.last_item) as _,
		}
	}
}

/// [`LB_SELITEMRANGEEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-selitemrangeex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SelItemRangeEx {
	pub first_index: u32,
	pub last_index: u32,
}

impl MsgSend for SelItemRangeEx {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SELITEMRANGEEX.into(),
			wparam: self.first_index as _,
			lparam: self.last_index as _,
		}
	}
}

/// [`LB_SETANCHORINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setanchorindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetAnchorIndex {
	pub index: u32,
}

impl MsgSend for SetAnchorIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETANCHORINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETCARETINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setcaretindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetCaretIndex {
	pub index: u32,
	pub at_least_partially_visible: bool,
}

impl MsgSend for SetCaretIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCARETINDEX.into(),
			wparam: self.index as _,
			lparam: self.at_least_partially_visible as _,
		}
	}
}

/// [`LB_SETCOLUMNWIDTH`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setcolumnwidth)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetColumnWidth {
	pub width: u32,
}

impl MsgSend for SetColumnWidth {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCOLUMNWIDTH.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETCOUNT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setcount)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetCount {
	pub new_count: u32,
}

impl MsgSend for SetCount {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			LB_ERRSPACE => Err(co::ERROR::NOT_ENOUGH_MEMORY),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCOUNT.into(),
			wparam: self.new_count as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETCURSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setcursel)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetCurSel {
	pub index: Option<u32>,
}

impl MsgSend for SetCurSel {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		if let None = self.index {
			Ok(())
		} else {
			match v as i32 {
				LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
				_ => Ok(()),
			}
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETCURSEL.into(),
			wparam: self.index.map_or(-1, |idx| idx as i32) as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETHORIZONTALEXTENT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-sethorizontalextent)
/// message parameters.
///
/// Return type: `()`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetHorizontalExtent {
	pub width: u32,
}

impl MsgSend for SetHorizontalExtent {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETHORIZONTALEXTENT.into(),
			wparam: self.width as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETITEMDATA`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setitemdata)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetItemData {
	pub index: u32,
	pub data: isize,
}

impl MsgSend for SetItemData {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETITEMDATA.into(),
			wparam: self.index as _,
			lparam: self.data,
		}
	}
}

/// [`LB_SETITEMHEIGHT`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setitemheight)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetItemHeight {
	pub index: Option<u32>,
	pub height: u8,
}

impl MsgSend for SetItemHeight {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETITEMHEIGHT.into(),
			wparam: self.index.unwrap_or(0) as _,
			lparam: self.height as _,
		}
	}
}

/// [`LB_SETLOCALE`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setlocale)
/// message parameters.
///
/// Return type: `WinResult<LCID>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetLocale {
	pub locale: LCID,
}

impl MsgSend for SetLocale {
	type RetType = WinResult<LCID>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			lcid => Ok(LCID(lcid as _)),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETLOCALE.into(),
			wparam: self.locale.0 as _,
			lparam: 0,
		}
	}
}

/// [`LB_SETSEL`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-setsel)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetSel {
	pub select: bool,
	pub index: Option<u32>,
}

impl MsgSend for SetSel {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETSEL.into(),
			wparam: self.select as _,
			lparam: self.index.map_or(-1, |idx| idx as i32) as _,
		}
	}
}

/// [`LB_SETTABSTOPS`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-settabstops)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetTabStops<'a> {
	pub tab_stops: &'a [u32],
}

impl<'a> MsgSend for SetTabStops<'a> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETTABSTOPS.into(),
			wparam: self.tab_stops.len(),
			lparam: self.tab_stops.as_ptr() as _,
		}
	}
}

/// [`LB_SETTOPINDEX`](https://docs.microsoft.com/en-us/windows/win32/controls/lb-settopindex)
/// message parameters.
///
/// Return type: `WinResult<()>`.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct SetTopIndex {
	pub index: u32,
}

impl MsgSend for SetTopIndex {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v as i32 {
			LB_ERR => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::LB::SETTOPINDEX.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}
