use std::sync::Arc;

use crate::co;
use crate::gui::base::Base;
use crate::gui::privs::multiply_dpi;
use crate::gui::raw_base::RawBase;
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::kernel::decl::{WinResult, WString};
use crate::prelude::{GdiHbrush, GuiEventsView, Handle, UserHwnd};
use crate::user::decl::{AdjustWindowRectEx, HBRUSH, HCURSOR, HICON, HWND,
	DispatchMessage, GetMessage, IdMenu, MSG, POINT, PostQuitMessage, RECT, SIZE,
	TranslateMessage, WNDCLASSEX};

/// A WindowModal with a raw window.
#[derive(Clone)]
pub(in crate::gui) struct RawModal(pub(in crate::gui) Arc<Obj>);

pub(in crate::gui) struct Obj { // actual fields of RawModal
	pub(in crate::gui) raw_base: RawBase,
	opts: WindowModalOpts,
	hchild_prev_focus_parent: VeryUnsafeCell<HWND>,
}

impl RawModal {
	pub(in crate::gui) fn new(
		parent_base: &Base,
		opts: WindowModalOpts) -> RawModal
	{
		let wnd = Self(Arc::new(
			Obj {
				raw_base: RawBase::new(Some(parent_base)),
				opts,
				hchild_prev_focus_parent: VeryUnsafeCell::new(HWND::NULL),
			},
		));
		wnd.default_message_handlers();
		wnd
	}

	pub(in crate::gui) fn show_modal(&self) -> WinResult<i32> {
		let hparent = self.0.raw_base.base.parent_base().unwrap().hwnd();
		let opts = &self.0.opts;

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::default();
		RawBase::fill_wndclassex(
			self.0.raw_base.base.parent_base().unwrap().hwnd().hinstance(),
			opts.class_style, opts.class_icon, opts.class_icon,
			opts.class_bg_brush, opts.class_cursor, &mut wcx, &mut class_name_buf)?;
		let atom = self.0.raw_base.register_class(&mut wcx)?;

		*self.0.hchild_prev_focus_parent.as_mut() = HWND::GetFocus().unwrap_or(HWND::NULL);
		hparent.EnableWindow(false); // https://devblogs.microsoft.com/oldnewthing/20040227-00/?p=40463

		let mut wnd_sz = opts.size;
		multiply_dpi(None, Some(&mut wnd_sz))?;

		let mut wnd_rc = RECT { // client area, will be adjusted to size with title bar and borders
			left: 0,
			top: 0,
			right: wnd_sz.cx,
			bottom: wnd_sz.cy,
		};
		AdjustWindowRectEx(&mut wnd_rc, opts.style, false, opts.ex_style)?;
		wnd_sz.cx = wnd_rc.right - wnd_rc.left;
		wnd_sz.cy = wnd_rc.bottom - wnd_rc.top;

		let rc_parent = hparent.GetWindowRect()?; // relative to screen
		let wnd_pos = POINT {
			x: rc_parent.left + (rc_parent.right - rc_parent.left) / 2 - wnd_sz.cx / 2, // center on parent
			y: rc_parent.top + (rc_parent.bottom - rc_parent.top) / 2 - wnd_sz.cy / 2
		};

		self.0.raw_base.create_window(
			atom,
			Some(&opts.title),
			IdMenu::None,
			wnd_pos, wnd_sz,
			opts.ex_style, opts.style,
		)?;

		self.run_modal_loop()
	}

	fn run_modal_loop(&self) -> WinResult<i32> {
		loop {
			let mut msg = MSG::default();
			if !GetMessage(&mut msg, None, 0, 0)? {
				// WM_QUIT was sent, exit modal loop now and signal parent.
				// wParam has the program exit code.
				// https://devblogs.microsoft.com/oldnewthing/20050222-00/?p=36393
				PostQuitMessage(msg.wParam as _);
				return Ok(msg.wParam as _);
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg.hwnd.GetAncestor(co::GA::ROOT)
				.unwrap_or(msg.hwnd);

			// Try to process keyboard actions for child controls.
			if hwnd_top_level.IsDialogMessage(&mut msg) {
				// Processed all keyboard actions for child controls.
				if self.0.raw_base.base.hwnd().is_null() {
					return Ok(0); // our modal was destroyed, terminate loop
				} else {
					continue;
				}
			}

			TranslateMessage(&msg);
			DispatchMessage(&msg);

			if self.0.raw_base.base.hwnd().is_null() {
				return Ok(0); // our modal was destroyed, terminate loop
			}
		}
	}

	fn default_message_handlers(&self) {
		self.0.raw_base.base.on().wm_set_focus({
			let self2 = self.clone();
			move |_| self2.0.raw_base.delegate_focus_to_first_child()
		});

		self.0.raw_base.base.on().wm_close({
			let self2 = self.clone();
			move || {
				if let Ok(hparent) = self2.0.raw_base.base.hwnd().GetWindow(co::GW::OWNER) {
					hparent.EnableWindow(true); // re-enable parent
					self2.0.raw_base.base.hwnd().DestroyWindow()?; // then destroy modal
					if !self2.0.hchild_prev_focus_parent.is_null() {
						self2.0.hchild_prev_focus_parent.SetFocus(); // this focus could be set on WM_DESTROY as well
					}
				}
				Ok(())
			}
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`WindowModal`](crate::gui::WindowModal) programmatically
/// with [`WindowModal::new`](crate::gui::WindowModal::new).
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct WindowModalOpts {
	/// Window class name to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to none.
	pub class_icon: HICON,
	/// Window cursor to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::IDC::ARROW`.
	pub class_cursor: HCURSOR,
	/// Window background brush to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::COLOR::BTNFACE`.
	pub class_bg_brush: HBRUSH,

	/// Window title to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Size of window client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 500 x 400.
	pub size: SIZE,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable;
	/// * `WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::DLGMODALFRAME`.
	pub ex_style: co::WS_EX,
}

impl Default for WindowModalOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: HICON::NULL,
			class_cursor: HCURSOR::NULL,
			class_bg_brush: HBRUSH::from_sys_color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			size: SIZE { cx: 500, cy: 400 },
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::DLGMODALFRAME,
		}
	}
}
