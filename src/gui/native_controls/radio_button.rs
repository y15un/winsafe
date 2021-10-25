use crate::aliases::WinResult;
use crate::co;
use crate::enums::{AccelMenuCtrl, AccelMenuCtrlData};
use crate::gui::events::ButtonEvents;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, calc_text_bound_box_check, multiply_dpi, ui_font};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{baseref_from_parent, Child, Parent, Window};
use crate::handles::HWND;
use crate::msg::{bm, wm};
use crate::structs::{POINT, SIZE};

struct Obj { // actual fields of RadioButton
	base: BaseNativeControl,
	opts_id: OptsId<RadioButtonOpts>,
	events: ButtonEvents,
}

impl_obj_window!(Obj);
impl_obj_child!(Obj);
impl_obj_nativecontrol!(Obj);

//------------------------------------------------------------------------------

/// Native
/// [radio button](https://docs.microsoft.com/en-us/windows/win32/controls/button-types-and-styles#radio-buttons)
/// control, actually a variation of the ordinary
/// [`Button`](crate::gui::Button): just a button with a specific style.
///
/// You cannot directly instantiate this object, you must use
/// [`RadioGroup`](crate::gui::RadioGroup).
pub struct RadioButton(Obj);

impl_send_sync!(RadioButton);
impl_ctl_debug!(RadioButton);

impl_ctl_window!(RadioButton);
impl_ctl_child!(RadioButton);
impl_ctl_nativecontrol!(RadioButton);
impl_ctl_nativecontrolevents!(RadioButton, ButtonEvents);
impl_ctl_focus!(RadioButton);

impl RadioButton {
	pub(in crate::gui) fn new(parent: &impl Parent, opts: RadioButtonOpts) -> RadioButton {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = RadioButtonOpts::define_ctrl_id(opts);
		let ctrl_id = opts.ctrl_id;

		Self(
			Obj {
				base: BaseNativeControl::new(parent_base_ref),
				opts_id: OptsId::Wnd(opts),
				events: ButtonEvents::new(parent_base_ref, ctrl_id),
			},
		)
	}

	pub(in crate::gui) fn new_dlg(parent: &impl Parent, ctrl_id: u16) -> RadioButton {
		let parent_base_ref = baseref_from_parent(parent);

		Self(
			Obj {
				base: BaseNativeControl::new(parent_base_ref),
				opts_id: OptsId::Dlg(ctrl_id),
				events: ButtonEvents::new(parent_base_ref, ctrl_id),
			},
		)
	}

	pub(in crate::gui) fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				multiply_dpi(Some(&mut pos), None)?;

				let mut sz = opts.size;
					if sz.cx == -1 && sz.cy == -1 {
						sz = calc_text_bound_box_check(&opts.text)?; // resize to fit text
					} else {
						multiply_dpi(None, Some(&mut sz))?; // user-defined size
					}

				let our_hwnd = self.0.base.create_window( // may panic
					"BUTTON", Some(&opts.text), pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.button_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });
				if opts.selected { self.select(true); }
			},
			OptsId::Dlg(ctrl_id) => {
				self.0.base.create_dlg(*ctrl_id)?; // may panic
			},
		}

		self.0.base.parent_base_ref().resizer_add(
			self.0.base.parent_base_ref(), self.0.base.hwnd_ref(), horz, vert)?;

		self.hwnd().SendMessage(bm::SetDontClick { dont_click: true });
		Ok(())
	}

	pub(in crate::gui) fn parent_hwnd_ref(&self) -> &HWND {
		self.0.base.parent_base_ref().hwnd_ref() // used by RadioGroup
	}

	/// Emulates the click event for the radio button by sending a
	/// [`bm::Click`](crate::msg::bm::Click) message.
	pub fn emulate_click(&self) {
		self.hwnd().SendMessage(bm::Click {});
	}

	/// Tells if this radio button is the currently selected one by sending a
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) message.
	pub fn is_selected(&self) -> bool {
		self.hwnd().SendMessage(bm::GetCheck {}) == co::BST::CHECKED
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message.
	pub fn select(&self, selected: bool) {
		self.hwnd().SendMessage(bm::SetCheck {
			state: if selected { co::BST::CHECKED } else { co::BST::UNCHECKED },
		});
	}

	/// Sets the this radio button as the currently selected one by sending a
	/// [`bm::SetCheck`](crate::msg::bm::SetCheck) message, then sends a
	/// [`wm::Command`](crate::msg::wm::Command) message to the parent, so it
	/// can handle the event.
	pub fn select_and_trigger(&self, selected: bool) -> WinResult<()> {
		self.select(selected);
		self.hwnd().GetParent()?.SendMessage(wm::Command {
			event: AccelMenuCtrl::Ctrl(
				AccelMenuCtrlData {
					notif_code: co::BN::CLICKED.into(),
					ctrl_id: self.ctrl_id(),
					ctrl_hwnd: self.hwnd(),
				},
			),
		});
		Ok(())
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`RadioButton`](crate::gui::RadioButton)
/// programmatically with [`RadioGroup::new`](crate::gui::RadioGroup::new).
pub struct RadioButtonOpts {
	/// Text of the control to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub text: String,
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control size, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to the size needed to fit the text.
	pub size: SIZE,
	/// Radio button styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `BS::AUTORADIOBUTTON`.
	pub button_style: co::BS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE`.
	///
	/// The first RadioButton of a group should also have `WS::TABSTOP | WS::GROUP`.
	/// If this object being passed to a [`RadioGroup`](crate::gui::RadioGroup),
	/// this will be automatically set.
	pub window_style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT`.
	pub window_ex_style: co::WS_EX,

	/// The control ID.
	///
	/// Defaults to an auto-generated ID.
	pub ctrl_id: u16,
	/// Horizontal behavior when the parent is resized.
	///
	/// Defaults to `Horz::None`.
	pub horz_resize: Horz,
	/// Vertical behavior when the parent is resized.
	///
	/// Defaults to `Vert::None`.
	pub vert_resize: Vert,

	/// Initial selection state.
	///
	/// Defaults to `false`.
	pub selected: bool,
}

impl Default for RadioButtonOpts {
	fn default() -> Self {
		Self {
			text: "".to_owned(),
			position: POINT::new(0, 0),
			size: SIZE::new(-1, -1), // will resize to fit the text
			button_style: co::BS::AUTORADIOBUTTON,
			window_style: co::WS::CHILD | co::WS::VISIBLE,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			selected: false,
		}
	}
}

impl RadioButtonOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}

	pub(in crate::gui) fn manual_clone(&self) -> RadioButtonOpts { // avoids a public clone method
		Self {
			text: self.text.clone(),
			position: self.position,
			size: self.size,
			button_style: self.button_style,
			window_style: self.window_style,
			window_ex_style: self.window_ex_style,
			ctrl_id: self.ctrl_id,
			horz_resize: self.horz_resize,
			vert_resize: self.vert_resize,
			selected: self.selected,
		}
	}
}
