use std::marker::PhantomData;
use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::gui::events::ComboBoxEvents;
use crate::gui::native_controls::combo_box_items::ComboBoxItems;
use crate::gui::native_controls::base_native_control::{BaseNativeControl, OptsId};
use crate::gui::privs::{auto_ctrl_id, multiply_dpi, ui_font};
use crate::gui::resizer::{Horz, Vert};
use crate::gui::traits::{baseref_from_parent, Parent};
use crate::handles::HWND;
use crate::msg::wm;
use crate::structs::{POINT, SIZE};

/// Native
/// [combo box](https://docs.microsoft.com/en-us/windows/win32/controls/about-combo-boxes)
/// control.
///
/// Implements [`Child`](crate::gui::Child) trait.
#[derive(Clone)]
pub struct ComboBox(Arc<Obj>);

struct Obj { // actual fields of ComboBox
	base: BaseNativeControl,
	opts_id: OptsId<ComboBoxOpts>,
	events: ComboBoxEvents,
}

unsafe impl Send for ComboBox {}
unsafe impl Sync for ComboBox {}

impl_debug!(ComboBox);
impl_child!(ComboBox);

impl ComboBox {
	/// Instantiates a new `ComboBox` object, to be created on the parent window
	/// with [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(parent: &impl Parent, opts: ComboBoxOpts) -> ComboBox {
		let parent_base_ref = baseref_from_parent(parent);
		let opts = ComboBoxOpts::define_ctrl_id(opts);
		let (ctrl_id, horz, vert) = (opts.ctrl_id, opts.horz_resize, opts.vert_resize);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Wnd(opts),
					events: ComboBoxEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm(parent_base_ref.create_or_initdlg(), {
			let me = new_self.clone();
			move |_| { me.create(horz, vert)?; Ok(0) }
		});

		new_self
	}

	/// Instantiates a new `ComboBox` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		parent: &impl Parent, ctrl_id: u16,
		horz_resize: Horz, vert_resize: Vert) -> ComboBox
	{
		let parent_base_ref = baseref_from_parent(parent);

		let new_self = Self(
			Arc::new(
				Obj {
					base: BaseNativeControl::new(parent_base_ref),
					opts_id: OptsId::Dlg(ctrl_id),
					events: ComboBoxEvents::new(parent_base_ref, ctrl_id),
				},
			),
		);

		parent_base_ref.privileged_events_ref().wm_init_dialog({
			let me = new_self.clone();
			move |_| { me.create(horz_resize, vert_resize)?; Ok(true) }
		});

		new_self
	}

	fn create(&self, horz: Horz, vert: Vert) -> WinResult<()> {
		if vert == Vert::Resize {
			panic!("ComboBox cannot be resized with Vert::Resize.");
		}

		match &self.0.opts_id {
			OptsId::Wnd(opts) => {
				let mut pos = opts.position;
				let mut sz = SIZE::new(opts.width as _, 0);
				multiply_dpi(Some(&mut pos), Some(&mut sz))?;

				let our_hwnd = self.0.base.create_window( // may panic
					"COMBOBOX", None, pos, sz,
					opts.ctrl_id,
					opts.window_ex_style,
					opts.window_style | opts.combo_box_style.into(),
				)?;

				our_hwnd.SendMessage(wm::SetFont { hfont: ui_font(), redraw: true });

				self.items().add(&opts.items)?;
				self.items().select(opts.selected_item);
			},
			OptsId::Dlg(ctrl_id) => self.0.base.create_dlg(*ctrl_id).map(|_| ())?, // may panic
		}

		self.0.base.parent_base_ref().resizer_add(
			self.0.base.parent_base_ref(), self.0.base.hwnd_ref(), horz, vert)
	}

	pub_fn_hwnd!();
	pub_fn_ctrlid!();
	pub_fn_focus!();
	pub_fn_onsubclass!();
	pub_fn_on!(ComboBoxEvents);

	/// Item methods.
	pub fn items<'a>(&'a self) -> ComboBoxItems<'a> {
		ComboBoxItems {
			hwnd: self.hwnd(),
			owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`ComboBox`](crate::gui::ComboBox) programmatically with
/// [`ComboBox::new`](crate::gui::ComboBox::new).
pub struct ComboBoxOpts {
	/// Control position within parent client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 0 x 0.
	pub position: POINT,
	/// Control width, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 120.
	pub width: u32,
	/// Combo box styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `CBS::DROPDOWNLIST`.
	///
	/// Suggestions:
	/// * replace with `CBS::DROPDOWN` to allow the user to type a text;
	/// * add `CBS::SORT` to automatically sort the items.
	pub combo_box_style: co::CBS,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CHILD | WS::VISIBLE | WS::TABSTOP | WS::GROUP`.
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
	///
	/// **Note:** A `ComboBox` cannot be resized vertically, so it will panic if
	/// you use `Vert::Resize`.
	pub vert_resize: Vert,

	/// Items to be added right away to the control.
	///
	/// Defaults to none.
	pub items: Vec<String>,
	/// Index of the item initially selected. The item must exist.
	///
	/// Defaults to `None`.
	pub selected_item: Option<u32>,
}

impl Default for ComboBoxOpts {
	fn default() -> Self {
		Self {
			position: POINT::new(0, 0),
			width: 120,
			combo_box_style: co::CBS::DROPDOWNLIST,
			window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::TABSTOP | co::WS::GROUP,
			window_ex_style: co::WS_EX::LEFT,
			ctrl_id: 0,
			horz_resize: Horz::None,
			vert_resize: Vert::None,
			items: Vec::default(),
			selected_item: None,
		}
	}
}

impl ComboBoxOpts {
	fn define_ctrl_id(mut self) -> Self {
		if self.ctrl_id == 0 {
			self.ctrl_id = auto_ctrl_id();
		}
		self
	}
}
