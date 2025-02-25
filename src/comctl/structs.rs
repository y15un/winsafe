#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::comctl::decl::{BmpIdbRes, HIMAGELIST, HTREEITEM, IconId,
	IconIdTdicon, IndexStr, PFNLVGROUPCOMPARE, PFTASKDIALOGCALLBACK,
	TreeitemTvi};
use crate::comctl::privs::{HINST_COMMCTRL, L_MAX_URL_LENGTH, MAX_LINKID_TEXT};
use crate::kernel::decl::{HINSTANCE, IdStr, SYSTEMTIME, WString};
use crate::kernel::privs::{IS_INTRESOURCE, MAKEINTRESOURCE};
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HBITMAP, HDC, HICON, HWND, POINT, RECT, SIZE,
	WINDOWPOS};

/// [`BUTTON_IMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_imagelist)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct BUTTON_IMAGELIST {
	pub himl: HIMAGELIST,
	pub margin: RECT,
	pub uAlign: co::BIA,
}

/// [`BUTTON_SPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_splitinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct BUTTON_SPLITINFO {
	pub mask: co::BCSIF,
	pub himlGlyph: HIMAGELIST,
	pub uSplitStyle: co::BCSS,
	pub size: SIZE,
}

/// [`DATETIMEPICKERINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-datetimepickerinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct DATETIMEPICKERINFO {
	cbSize: u32,
	pub rcCheck: RECT,
	pub stateCheck: co::STATE_SYSTEM,
	pub rcButton: RECT,
	pub stateButton: co::STATE_SYSTEM,
	pub hwndEdit: HWND,
	pub hwndUD: HWND,
	pub hwndDropDown: HWND,
}

impl_default_with_size!(DATETIMEPICKERINFO, cbSize);

/// [`EDITBALLOONTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-editballoontip)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct EDITBALLOONTIP<'a, 'b> {
	cbStruct: u32,
	pszTitle: *mut u16,
	pszText: *mut u16,
	pub ttiIcon: co::TTI,

	pszTitle_: PhantomData<&'a mut u16>,
	pszText_: PhantomData<&'b mut u16>,
}

impl_default_with_size!(EDITBALLOONTIP, cbStruct, 'a, 'b);

impl<'a, 'b> EDITBALLOONTIP<'a, 'b> {
	pub_fn_string_ptr_get_set!('a, pszTitle, set_pszTitle);
	pub_fn_string_ptr_get_set!('b, pszText, set_pszText);
}

/// [`HDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hditemw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct HDITEM<'a> {
	pub mask: co::HDI,
	pub cxy: i32,
	pszText: *mut u16,
	pub hbm: HBITMAP,
	cchTextMax: i32,
	pub fmt: co::HDF,
	pub lParam: isize,
	pub iImage: i32,
	pub iOrder: i32,
	pub typeFilter: co::HDFT,
	pub pvFilter: *mut std::ffi::c_void,
	pub state: co::HDIS,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(HDITEM, 'a);

impl<'a> HDITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`HDHITTESTINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hdhittestinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
#[derive(Default)]
pub struct HDHITTESTINFO {
	pub pt: POINT,
	pub flags: co::HHT,
	pub iItem: i32,
}

/// [`HDLAYOUT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hdlayout)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct HDLAYOUT<'a, 'b> {
	prc: *mut RECT,
	pwpos: *mut WINDOWPOS,
	fuuu: i32,

	prc_: PhantomData<&'a mut RECT>,
	pwpos_: PhantomData<&'b mut WINDOWPOS>,
}

impl_default!(HDLAYOUT, 'a, 'b);

impl<'a, 'b> HDLAYOUT<'a, 'b> {
	/// Sets the field.
	pub fn set_prc(&mut self, rc: Option<&'a mut RECT>) {
		self.prc = rc.map(|rc| rc as _).unwrap_or(std::ptr::null_mut());
	}

	/// Sets the field.
	pub fn set_pwpos(&mut self, pos: Option<&'b mut WINDOWPOS>) {
		self.pwpos = pos.map(|pos| pos as _).unwrap_or(std::ptr::null_mut());
	}
}

/// [`INITCOMMONCONTROLSEX`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-initcommoncontrolsex)
/// struct
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct INITCOMMONCONTROLSEX {
	dwSize: u32,
	pub icc: co::ICC,
}

impl_default_with_size!(INITCOMMONCONTROLSEX, dwSize);

/// [`LITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-litem)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LITEM {
	pub mask: co::LIF,
	pub iLink: i32,
	pub state: co::LIS,
	pub stateMask: co::LIS,
	szID: [u16; MAX_LINKID_TEXT],
	szUrl: [u16; L_MAX_URL_LENGTH],
}

impl_default!(LITEM);

impl LITEM {
	pub_fn_string_arr_get_set!(szID, set_szID);
	pub_fn_string_arr_get_set!(szUrl, set_szUrl);
}

/// [`LVBKIMAGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvbkimagew)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVBKIMAGE<'a> {
	pub uFlags: co::LVBKIF,
	pub hbm: HBITMAP,
	pszImage: *mut u16,
	cchImageMax: u32,
	pub xOffsetPercent: i32,
	pub yOffsetPercent: i32,

	pszImage_: PhantomData<&'a mut u16>,
}

impl_default!(LVBKIMAGE, 'a);

impl<'a> LVBKIMAGE<'a> {
	pub_fn_string_buf_get_set!('a, pszImage, set_pszImage, cchImageMax);
}

/// [`LVCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvcolumnw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVCOLUMN<'a> {
	pub mask: co::LVCF,
	pub fmt: co::LVCFMT_C,
	pub cx: i32,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iSubItem: i32,
	pub iImage: i32,
	pub iOrder: i32,
	pub cxMin: i32,
	pub cxDefault: i32,
	pub cxIdeal: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(LVCOLUMN, 'a);

impl<'a> LVCOLUMN<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`LVFINDINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVFINDINFO<'a> {
	pub flags: co::LVFI,
	psz: *mut u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK_DIR,

	psz_: PhantomData<&'a mut u16>,
}

impl_default!(LVFINDINFO, 'a);

impl<'a> LVFINDINFO<'a> {
	pub_fn_string_ptr_get_set!('a, psz, set_psz);
}

/// [`LVFOOTERINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfooterinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVFOOTERINFO<'a> {
	pub mask: co::LVFF,
	pszText: *mut u16,
	cchTextMax: i32,
	pub cItems: u32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(LVFOOTERINFO, 'a);

impl<'a> LVFOOTERINFO<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`LVFOOTERITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfooteritem)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVFOOTERITEM<'a> {
	pub mask: co::LVFIF,
	pub iItem: i32,
	pszText: *mut u16,
	cchTextMax: i32,
	pub state: co::LVFIS,
	pub stateMask: co::LVFIS,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(LVFOOTERITEM, 'a);

impl<'a> LVFOOTERITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`LVGROUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvgroup)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	cbSize: u32,
	pub mask: co::LVGF,
	pszHeader: *mut u16,
	cchHeader: i32,
	pszFooter: *mut u16,
	cchFooter: i32,
	pub iGroupId: i32,
	pub stateMask: co::LVGS,
	pub state: co::LVGS,
	pub uAlign: co::LVGA_FH,
	pszSubtitle: *mut u16,
	cchSubtitle: i32,
	pszTask: *mut u16,
	cchTask: i32,
	pszDescriptionTop: *mut u16,
	cchDescriptionTop: i32,
	pszDescriptionBottom: *mut u16,
	cchDescriptionBottom: i32,
	pub iTitleImage: i32,
	pub iExtendedImage: i32,
	pub iFirstItem: i32,
	pub cItems: u32,
	pszSubsetTitle: *mut u16,
	cchSubsetTitle: i32,

	pszHeader_: PhantomData<&'a mut u16>,
	pszFooter_: PhantomData<&'b mut u16>,
	pszSubtitle_: PhantomData<&'c mut u16>,
	pszTask_: PhantomData<&'d mut u16>,
	pszDescriptionTop_: PhantomData<&'e mut u16>,
	pszDescriptionBottom_: PhantomData<&'f mut u16>,
	pszSubsetTitle_: PhantomData<&'g mut u16>,
}

impl_default_with_size!(LVGROUP, cbSize, 'a, 'b, 'c, 'd, 'e, 'f, 'g);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	pub_fn_string_buf_get_set!('a, pszHeader, set_pszHeader, cchHeader);
	pub_fn_string_buf_get_set!('b, pszFooter, set_pszFooter, cchFooter);
	pub_fn_string_buf_get_set!('c, pszSubtitle, set_pszSubtitle, cchSubtitle);
	pub_fn_string_buf_get_set!('d, pszTask, set_pszTask, cchTask);
	pub_fn_string_buf_get_set!('e, pszDescriptionTop, set_pszDescriptionTop, cchDescriptionTop);
	pub_fn_string_buf_get_set!('f, pszDescriptionBottom, set_pszDescriptionBottom, cchDescriptionBottom);
	pub_fn_string_buf_get_set!('g, pszSubsetTitle, set_pszSubsetTitle, cchSubsetTitle);
}

/// [`LVGROUPMETRICS`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvgroupmetrics)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVGROUPMETRICS {
	cbSize: u32,
	pub mask: co::LVGMF,
	pub Left: u32,
	pub Top: u32,
	pub Right: u32,
	pub Bottom: u32,
	pub crLeft: COLORREF,
	pub crTop: COLORREF,
	pub crRight: COLORREF,
	pub crBottom: COLORREF,
	pub crHeader: COLORREF,
	pub crFooter: COLORREF,
}

impl_default_with_size!(LVGROUPMETRICS, cbSize);

/// [`LVHITTESTINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvhittestinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
#[derive(Default)]
pub struct LVHITTESTINFO {
	pub pt: POINT,
	pub flags: co::LVHT,
	pub iItem: i32,
	pub iSubItem: i32,
	pub iGroup: i32,
}

/// [`LVINSERTGROUPSORTED`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvinsertgroupsorted)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVINSERTGROUPSORTED<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	pub pfnGroupCompare: Option<PFNLVGROUPCOMPARE>,
	pub pvData: usize,
	pub lvGroup: LVGROUP<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> Default for LVINSERTGROUPSORTED<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
	fn default() -> Self {
		Self {
			pfnGroupCompare: None,
			pvData: 0,
			lvGroup: LVGROUP::default(), // has cbSize, so we can't use impl_default_size macro
		}
	}
}

/// [`LVINSERTMARK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvinsertmark)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVINSERTMARK {
	cbSize: u32,
	pub dwFlags: co::LVIM,
	pub iItem: i32,
	dwReserved: u32,
}

impl_default!(LVINSERTMARK);

/// [`LVITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVITEM<'a> {
	pub mask: co::LVIF,
	pub iItem: i32,
	pub iSubItem: i32,
	pub state: co::LVIS,
	pub stateMask: co::LVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,
	pub iIndent: i32,
	pub iGroupId: co::LVI_GROUPID,
	pub cColumns: u32,
	pub puColumns: *mut i32,
	pub piColFmt: *mut co::LVCFMT_I,
	pub iGroup: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(LVITEM, 'a);

impl<'a> LVITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`LVITEMINDEX`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemindex)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct LVITEMINDEX {
	pub iItem: i32,
	pub iGroup: i32,
}

/// [`LVSETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvsetinfotip)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVSETINFOTIP<'a> {
	cbSize: u32,
	pub dwFlags: u32, // unspecified
	pszText: *mut u16,
	pub iItem: i32,
	pub iSubItem: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default_with_size!(LVSETINFOTIP, cbSize, 'a);

impl<'a> LVSETINFOTIP<'a> {
	pub_fn_string_ptr_get_set!('a, pszText, set_pszText);
}

/// [`LVTILEINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvtileinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVTILEINFO<'a> {
	cbSize: u32,
	pub iItem: i32,
	cColumns: u32,
	puColumns: *mut u32,
	piColFmt: *mut co::LVCFMT_C,

	puColumns_: PhantomData<&'a mut u32>,
}

impl_default_with_size!(LVTILEINFO, cbSize, 'a);

impl<'a> LVTILEINFO<'a> {
	/// Returns the `puColumns` field.
	pub fn puColumns(&self) -> Option<&'a mut [u32]> {
		unsafe {
			self.puColumns.as_mut()
				.map(|_| std::slice::from_raw_parts_mut(self.puColumns, self.cColumns as _))
		}
	}

	/// Returns the `piColFmt` field.
	pub fn piColFmt(&self) -> Option<&'a mut [co::LVCFMT_C]> {
		unsafe {
			self.puColumns.as_mut()
				.map(|_| std::slice::from_raw_parts_mut(self.piColFmt, self.cColumns as _))
		}
	}

	/// Sets the `puColumns` and `piColFmt` fields. The slices must have the
	/// same length.
	pub fn set_puColumns_piColFmt(&mut self, val: Option<(&'a mut [u32], &'a mut [co::LVCFMT_C])>) {
		if let Some(val) = val {
			if val.0.len() != val.1.len() {
				panic!("Different slice lengths: {} and {}.", val.0.len(), val.1.len());
			}
			self.cColumns = val.0.len() as _;
			self.puColumns = val.0.as_mut_ptr();
			self.piColFmt = val.1.as_mut_ptr();
		} else {
			self.cColumns = 0;
			self.puColumns = std::ptr::null_mut();
			self.piColFmt = std::ptr::null_mut();
		}
	}
}

/// [`LVTILEVIEWINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvtileviewinfo)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct LVTILEVIEWINFO {
	cbSize: u32,
	pub dwMask: co::LVTVIM,
	pub dwFlags: co::LVTVIF,
	pub sizeTile: SIZE,
	pub cLines: i32,
	pub rcLabelMargin: RECT,
}

impl_default_with_size!(LVTILEVIEWINFO, cbSize);

/// [`NMBCDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbcdropdown)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMBCDROPDOWN {
	pub hdr: NMHDR,
	pub rcButton: RECT,
}

/// [`NMBCHOTITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbchotitem)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMBCHOTITEM {
	pub hdr: NMHDR,
	pub dwFlags: co::HICF,
}

/// [`NMCHAR`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmchar)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMCHAR {
	pub hdr: NMHDR,
	pub ch: u32,
	pub dwItemPrev: u32,
	pub dwItemNext: u32,
}

/// [`NMCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmcustomdraw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMCUSTOMDRAW {
	pub hdr: NMHDR,
	pub dwDrawStage: co::CDDS,
	pub hdc: HDC,
	pub rc: RECT,
	pub dwItemSpec: usize,
	pub uItemState: co::CDIS,
	pub lItemlParam: isize,
}

/// [`NMDATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimechange)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMDATETIMECHANGE {
	pub nmhdr: NMHDR,
	pub dwFlags: co::GDT,
	pub st: SYSTEMTIME,
}

/// [`NMDATETIMEFORMAT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMDATETIMEFORMAT<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *mut u16,
	pub st: SYSTEMTIME,
	pszDisplay: *mut u16,
	szDisplay: [u16; 64], // used as a buffer to pszDisplay

	pszFormat_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEFORMAT, 'a);

impl<'a> NMDATETIMEFORMAT<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);

	/// Returns the `pszDisplay` field.
	pub fn pszDisplay(&self) -> String {
		WString::from_wchars_nullt(self.pszDisplay).to_string()
	}

	/// Sets the `pszDisplay` field.
	pub fn set_pszDisplay(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szDisplay);
	}
}

/// [`NMDATETIMEFORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatqueryw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMDATETIMEFORMATQUERY<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *mut u16,
	pub szMax: SIZE,

	pszFormat_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEFORMATQUERY, 'a);

impl<'a> NMDATETIMEFORMATQUERY<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);
}

/// [`NMDATETIMESTRING`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimestringw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMDATETIMESTRING<'a> {
	pub nmhdr: NMHDR,
	pszUserString: *mut u16,
	pub st: SYSTEMTIME,
	pub dwFlags: co::GDT,

	pszUserString_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMESTRING, 'a);

impl<'a> NMDATETIMESTRING<'a> {
	pub_fn_string_ptr_get_set!('a, pszUserString, set_pszUserString);
}

/// [`NMDATETIMEWMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimewmkeydownw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWN<'a> {
	pub nmhdr: NMHDR,
	pub nVirtKey: i32,
	pszFormat: *mut u16,
	pub st: SYSTEMTIME,

	pszFormat_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEWMKEYDOWN, 'a);

impl<'a> NMDATETIMEWMKEYDOWN<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);
}

/// [`NMDAYSTATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdaystate)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMDAYSTATE<'a> {
	pub nmhdr: NMHDR,
	pub stStart: SYSTEMTIME,
	cDayState: i32,
	prgDayState: *mut u32,

	prgDayState_: PhantomData<&'a mut u32>,
}

impl_default!(NMDAYSTATE, 'a);

impl<'a> NMDAYSTATE<'a> {
	pub_fn_array_buf_get_set!('a, prgDayState, set_prgDayState, cDayState, u32);
}

/// [`NMHDR`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nmhdr)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMHDR {
	/// A window handle to the control sending the message.
	pub hwndFrom: HWND,
	idFrom: usize,
	/// Notification code sent in
	/// [`WM_NOTIFY`](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).
	pub code: co::NM,
}

impl_default!(NMHDR);

impl NMHDR {
	/// `Returns the `idFrom` field, the ID of the control sending the message.
	pub fn idFrom(&self) -> u16 {
		self.idFrom as _
	}

	/// Sets the `idFrom` field, the ID of the control sending the message.
	pub fn set_idFrom(&mut self, val: u16) {
		self.idFrom = val as _
	}
}

/// [`NMITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmitemactivate)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMITEMACTIVATE {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
	pub uKeyFlags: co::LVKF,
}

/// [`PBRANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-pbrange)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct PBRANGE {
	pub iLow: i32,
	pub iHigh: i32,
}

/// [`NMIPADDRESS`](https://docs.microsoft.com/en-us/windows/win32/api/Commctrl/ns-commctrl-nmipaddress)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMIPADDRESS {
	pub hdr: NMHDR,
	pub iField: i32,
	pub iValue: i32,
}

/// [`NMLINK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlink)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLINK {
	pub hdr: NMHDR,
	pub item: LITEM,
}

/// [`NMLISTVIEW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlistview)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLISTVIEW {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
}

/// [`NMLVCACHEHINT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcachehint)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVCACHEHINT {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
}

/// [`NMLVCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcustomdraw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVCUSTOMDRAW {
	pub mcd: NMCUSTOMDRAW,
	pub clrText: COLORREF,
	pub clrTextBk: COLORREF,
	pub iSubItem: i32,
	pub dwItemType: co::LVCDI,
	pub clrFace: COLORREF,
	pub iIconEffect: i32,
	pub iIconPhase: i32,
	pub iPartId: i32,
	pub iStateId: i32,
	pub rcText: RECT,
	pub uAlign: co::LVGA_HEADER,
}

/// [`NMLVDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvdispinfow)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVDISPINFO<'a> {
	pub hdr: NMHDR,
	pub item: LVITEM<'a>,
}

/// [`NMLVEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl_default!(NMLVEMPTYMARKUP);

impl NMLVEMPTYMARKUP {
	pub_fn_string_arr_get_set!(szMarkup, set_szMarkup);
}

/// [`NMLVFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvfinditemw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVFINDITEM<'a> {
	pub hdr: NMHDR,
	pub iStart: i32,
	pub lvfi: LVFINDINFO<'a>,
}

/// [`NMLVGETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvgetinfotipw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVGETINFOTIP<'a> {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(NMLVGETINFOTIP, 'a);

impl<'a> NMLVGETINFOTIP<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`NMLVKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvkeydown)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVKEYDOWN {
	pub hdr: NMHDR,
	pub wVKey: co::VK,
	flags: u32,
}

impl_default!(NMLVKEYDOWN);

/// [`NMLVLINK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvlink)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVLINK {
	pub hdr: NMHDR,
	pub link: LITEM,
	pub iItem: i32,
	pub iSubItem: i32,
}

/// [`NMLVODSTATECHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvodstatechange)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVODSTATECHANGE {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
}

/// [`NMLVSCROLL`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvscroll)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMLVSCROLL {
	pub hdr: NMHDR,
	pub dx: i32,
	pub dy: i32,
}

/// [`NMMOUSE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmmouse)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMMOUSE {
	pub hdr: NMHDR,
	pub dwItemSpec: usize,
	pub dwItemData: usize,
	pub pt: POINT,
	pub dwHitInfo: isize,
}

/// [`NMTRBTHUMBPOSCHANGING`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtrbthumbposchanging)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMTRBTHUMBPOSCHANGING {
	pub hdr: NMHDR,
	pub dwPos: u32,
	pub nReason: co::TB,
}

/// [`NMSELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmselchange)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMSELCHANGE {
	pub nmhdr: NMHDR,
	pub stSelStart: SYSTEMTIME,
	pub stSelEnd: SYSTEMTIME,
}

/// [`NMTREEVIEW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtreevieww)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMTREEVIEW<'a, 'b> {
	pub hdr: NMHDR,
	pub action: u32, // actual type varies
	pub itemOld: TVITEM<'a>,
	pub itemNew: TVITEM<'b>,
	pub ptDrag: POINT,
}

/// [`NMTVCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvcustomdraw)
/// stuct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMTVCUSTOMDRAW {
	pub nmcd: NMCUSTOMDRAW,
	pub clrText: COLORREF,
	pub clrTextBk: COLORREF,
	pub iLevel: i32,
}

/// [`NMTVITEMCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvitemchange)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMTVITEMCHANGE {
	pub hdr: NMHDR,
	pub uChanged: co::TVIF,
	pub hItem: HTREEITEM,
	pub uStateNew: co::TVIS,
	pub uStateOld: co::TVIS,
	pub lParam: isize,
}

/// [`NMVIEWCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmviewchange)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct NMVIEWCHANGE {
	pub nmhdr: NMHDR,
	pub dwOldView: co::MCMV,
	pub dwNewView: co::MCMV,
}

/// [`TASKDIALOG_BUTTON`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-taskdialog_button)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C, packed)]
pub struct TASKDIALOG_BUTTON<'a> {
	nButtonID: i32,
	pszButtonText: *mut u16,

	pszButtonText_: PhantomData<&'a mut u16>,
}

impl_default!(TASKDIALOG_BUTTON, 'a);

impl<'a> TASKDIALOG_BUTTON<'a> {
	pub_fn_resource_id_get_set!(nButtonID, set_nButtonID);
	pub_fn_string_ptr_get_set!('a, pszButtonText, set_pszButtonText);
}

/// [`TASKDIALOGCONFIG`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-taskdialogconfig)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C, packed)]
pub struct TASKDIALOGCONFIG<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
	cbSize: u32,
	pub hwndParent: HWND,
	pub hInstance: HINSTANCE,
	pub dwFlags: co::TDF,
	pub dwCommonButtons: co::TDCBF,
	pszWindowTitle: *mut u16,
	pszMainIcon: *const u16, // union with HICON
	pszMainInstruction: *mut u16,
	pszContent: *mut u16,
	cButtons: u32,
	pButtons: *mut TASKDIALOG_BUTTON<'d>,
	pub nDefaultButton: i32, // actually co::DLGID, which is u16
	cRadioButtons: u32,
	pRadioButtons: *mut TASKDIALOG_BUTTON<'e>,
	pub nDefaultRadioButton: i32,
	pszVerificationText: *mut u16,
	pszExpandedInformation: *mut u16,
	pszExpandedControlText: *mut u16,
	pszCollapsedControlText: *mut u16,
	pszFooterIcon: *const u16, // union with HICON
	pszFooter: *mut u16,
	pub pfCallback: Option<PFTASKDIALOGCALLBACK>,
	pub lpCallbackData: isize,
	pub cxWidth: u32,

	pszWindowTitle_: PhantomData<&'a mut u16>,
	pszMainInstruction_: PhantomData<&'b mut u16>,
	pszContent_: PhantomData<&'c mut u16>,
	pButtons_: PhantomData<&'d mut TASKDIALOG_BUTTON<'d>>,
	pRadioButtons_: PhantomData<&'e mut TASKDIALOG_BUTTON<'e>>,
	pszVerificationText_: PhantomData<&'f mut u16>,
	pszExpandedInformation_: PhantomData<&'g mut u16>,
	pszExpandedControlText_: PhantomData<&'h mut u16>,
	pszCollapsedControlText_: PhantomData<&'i mut u16>,
	pszFooter_: PhantomData<&'j mut u16>,
}

impl_default_with_size!(TASKDIALOGCONFIG, cbSize, 'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j);

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
	TASKDIALOGCONFIG<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
{
	pub_fn_string_ptr_get_set!('a, pszWindowTitle, set_pszWindowTitle);

	/// Returns the `pszMainIcon` field.
	pub fn pszMainIcon(&self) -> IconIdTdicon {
		if IS_INTRESOURCE(self.pszMainIcon) {
			if self.pszMainIcon as u16 >= 0xfffc {
				IconIdTdicon::Tdicon(co::TD_ICON(self.pszMainIcon as _))
			} else {
				IconIdTdicon::Id(self.pszMainIcon as _)
			}
		} else {
			IconIdTdicon::Icon(HICON(self.pszMainIcon as _))
		}
	}

	/// Sets the `pszMainIcon` field.
	pub fn set_pszMainIcon(&mut self, val: IconIdTdicon) {
		match val {
			IconIdTdicon::None => self.pszMainIcon = std::ptr::null_mut(),
			IconIdTdicon::Icon(hicon) => self.pszMainIcon = hicon.0 as _,
			IconIdTdicon::Id(id) => self.pszMainIcon = MAKEINTRESOURCE(id as _),
			IconIdTdicon::Tdicon(tdi) => self.pszMainIcon = MAKEINTRESOURCE(tdi.0 as _),
		}
	}

	pub_fn_string_ptr_get_set!('b, pszMainInstruction, set_pszMainInstruction);
	pub_fn_string_ptr_get_set!('c, pszContent, set_pszContent);
	pub_fn_array_buf_get_set!('d, pButtons, set_pButtons, cButtons, TASKDIALOG_BUTTON);
	pub_fn_array_buf_get_set!('e, pRadioButtons, set_pRadioButtons, cRadioButtons, TASKDIALOG_BUTTON);
	pub_fn_string_ptr_get_set!('f, pszVerificationText, set_pszVerificationText);
	pub_fn_string_ptr_get_set!('g, pszExpandedInformation, set_pszExpandedInformation);
	pub_fn_string_ptr_get_set!('h, pszExpandedControlText, set_pszExpandedControlText);
	pub_fn_string_ptr_get_set!('i, pszCollapsedControlText, set_pszCollapsedControlText);

	/// Returns the `pszFooterIcon` field.
	pub fn pszFooterIcon(&self) -> IconId {
		if IS_INTRESOURCE(self.pszFooterIcon) {
			IconId::Id(self.pszFooterIcon as _)
		} else {
			IconId::Icon(HICON(self.pszFooterIcon as _))
		}
	}

	/// Sets the `pszFooterIcon` field.
	pub fn set_pszFooterIcon(&mut self, val: IconId) {
		match val {
			IconId::None => self.pszFooterIcon = std::ptr::null_mut(),
			IconId::Icon(hicon) => self.pszFooterIcon = hicon.0 as _,
			IconId::Id(id) => self.pszFooterIcon = MAKEINTRESOURCE(id as _),
		}
	}

	pub_fn_string_ptr_get_set!('j, pszFooter, set_pszFooter);
}

/// [`TBADDBITMAP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbaddbitmap)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
#[derive(Clone)]
pub struct TBADDBITMAP {
	hInst: HINSTANCE,
	nID: usize,
}

impl_default!(TBADDBITMAP);

impl TBADDBITMAP {
	/// Returns the `hInst` and `nID` fields.
	pub fn nID(&self) -> BmpIdbRes {
		match self.hInst {
			HINST_COMMCTRL => BmpIdbRes::Idb(co::IDB(self.nID)),
			HINSTANCE::NULL => BmpIdbRes::Bmp(HBITMAP(self.nID as _ )),
			hInst => BmpIdbRes::Res(IdStr::from_ptr(self.nID as _), hInst),
		}
	}

	/// Sets the `hInst` and `nID` fields.
	pub fn set_nID(&mut self, val: &BmpIdbRes) {
		*self = match val {
			BmpIdbRes::Idb(idb) => Self { hInst: HINST_COMMCTRL, nID: idb.0 },
			BmpIdbRes::Bmp(bmp) => Self { hInst: HINSTANCE::NULL, nID: bmp.0 as _ },
			BmpIdbRes::Res(res, hInst) => Self { hInst: *hInst, nID: res.as_ptr() as _ },
		}
	}
}

/// [`TBBUTTON`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbbutton)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct TBBUTTON<'a> {
	pub iBitmap: i32,
	pub idCommand: i32,
	pub fsState: co::TBSTATE,
	pub fsStyle: co::BTNS,
	bReserved: [u8; 6], // assumes 64-bit architecture
	pub dwData: usize,
	iString: isize,

	iString_: PhantomData<&'a mut u16>,
}

impl_default!(TBBUTTON, 'a);

impl<'a> TBBUTTON<'a> {
	/// Returns the `iString` field.
	pub fn iString(&self) -> IndexStr {
		if IS_INTRESOURCE(self.iString as _) {
			IndexStr::Index(self.iString as _)
		} else {
			IndexStr::Str(WString::from_wchars_nullt(self.iString as _))
		}
	}

	/// Sets the `iString` field.
	pub fn set_iString(&mut self, val: &'a mut IndexStr) {
		self.iString = match val {
			IndexStr::Index(i) => *i as _,
			IndexStr::Str(s) => unsafe { s.as_mut_ptr() as _ },
		};
	}
}

/// [`TVINSERTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvinsertstructw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct TVINSERTSTRUCT<'a> {
	pub hParent: HTREEITEM,
	hInsertAfter: isize,
	pub itemex: TVITEMEX<'a>,
}

impl_default!(TVINSERTSTRUCT, 'a);

impl<'a> TVINSERTSTRUCT<'a> {
	/// Returns the `hInsertAfter` field.
	pub fn hInsertAfter(&self) -> TreeitemTvi {
		TreeitemTvi::from_isize(self.hInsertAfter)
	}

	/// Sets the `hInsertAfter` field.
	pub fn set_hInsertAfter(&mut self, val: TreeitemTvi) {
		self.hInsertAfter = val.as_isize();
	}
}

/// [`TVITEMEX`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvitemexw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct TVITEMEX<'a> {
	pub mask: co::TVIF,
	pub hItem: HTREEITEM,
	pub state: co::TVIS,
	pub stateMask: co::TVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub iSelectedImage: i32,
	pub cChildren: i32,
	pub lParam: isize,
	pub iIntegral: i32,
	pub uStateEx: co::TVIS_EX,
	hwnd: HWND,
	pub iExpandedImage: i32,
	iReserved: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(TVITEMEX, 'a);

impl<'a> TVITEMEX<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`TVITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvitemw)
/// struct.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
#[repr(C)]
pub struct TVITEM<'a> {
	pub mask: co::TVIF,
	pub hItem: HTREEITEM,
	pub state: co::TVIS,
	pub stateMask: co::TVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub iSelectedImage: i32,
	pub cChildren: i32,
	pub lParam: isize,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(TVITEM, 'a);

impl<'a> TVITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}
