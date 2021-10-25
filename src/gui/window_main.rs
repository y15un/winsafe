use crate::aliases::ErrResult;
use crate::co;
use crate::funcs::{InitCommonControls, IsWindowsVistaOrGreater, SetProcessDPIAware};
use crate::gui::base::Base;
use crate::gui::dlg_main::DlgMain;
use crate::gui::events::WindowEventsAll;
use crate::gui::privs::{create_ui_font, delete_ui_font};
use crate::gui::raw_main::{WindowMainOpts, RawMain};
use crate::gui::traits::{ParentEvents, UiThread, Window};
use crate::handles::HWND;

#[derive(Clone)]
enum RawDlg { Raw(RawMain), Dlg(DlgMain) }

/// An user main window, which can handle events. Usually, this is the first
/// window of your application, launched directly from the `main` function. Can
/// be programmatically created or load a dialog resource from a `.res` file.
///
/// # Examples
///
/// The two examples below show how to create the main window: programmatically,
/// or by loading a dialog resource from a `.res` file.
///
/// ## Programmaticaly creating a window
///
/// Below is a full application based on a non-dialog `WindowMain`, whose
/// instance is kept within `MyMain` struct. This is not necessary, but is
/// highly recommended, because it makes it easier to manage the window
/// contents.
///
/// The `main` function instantiates `MyMain` by calling `MyMain::new`, which
/// then calls `WindowMain::new`. Note how it receives a `WindowMainOpts`
/// argument, who defines all the options.
///
/// The window is handling the
/// [mouse click event](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown)
/// with a closure, and it displays the clicked coordinates in the
/// [title bar](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw).
///
/// ```rust,ignore
/// #![windows_subsystem = "windows"]
///
/// use winsafe::prelude::*;
/// use winsafe::{gui, msg, ErrResult};
///
/// fn main() {
///     let my_main = MyMain::new();
///     if let Err(e) = my_main.wnd.run_main(None) {
///         eprintln!("{}", e);
///     }
/// }
///
/// pub struct MyMain {
///     wnd: gui::WindowMain,
/// }
///
/// impl MyMain {
///     pub fn new() -> MyMain {
///         let wnd = gui::WindowMain::new(
///             gui::WindowMainOpts {
///                 title: "My window".to_owned(),
///                 ..Default::default()
///             },
///         );
///
///         let new_self = Self { wnd };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///         self.wnd.on().wm_l_button_down({
///             let wnd = self.wnd.clone(); // clone to pass it to the closure
///             move |p: msg::wm::LButtonDown| -> ErrResult<()> {
///                 let txt = &format!("Coords {} x {}", p.coords.x, p.coords.y);
///                 wnd.hwnd().SetWindowText(txt)?;
///                 Ok(())
///             }
///         });
///     }
/// }
/// ```
///
/// ## Loading a window resource from a `.res` file
///
/// A window can also be loaded from a Win32 resource file (usually a `.res`
/// file). Below, a full aplication where `WindowMain` loads a window resource,
/// instead of creating the window programmatically. Note how
/// `WindowMain::new_dlg` instead of `WindowMain::new`.
///
/// ```rust,ignore
/// #![windows_subsystem = "windows"]
///
/// use winsafe::prelude::*;
/// use winsafe::gui;
///
/// const ID_DLG_MAIN: i32 = 101; // in our .res file, this is the dialog ID
///
/// fn main() {
///     let my_main = MyDlg::new();
///     if let Err(e) = my_main.dlg.run_main(None) {
///         eprintln!("{}", e);
///     }
/// }
///
/// pub struct MyDlg {
///     dlg: gui::WindowMain,
/// }
///
/// impl MyDlg {
///     pub fn new() -> MyDlg {
///         let dlg = gui::WindowMain::new_dlg(ID_DLG_MAIN, None, None);
///
///         let new_self = Self { dlg };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///
///     }
/// }
/// ```
#[derive(Clone)]
pub struct WindowMain {
	raw_dlg: RawDlg,
}

impl_send_sync!(WindowMain);
impl_debug!(WindowMain);
impl_parent!(WindowMain);

impl_window!(WindowMain);
impl_aswindow!(WindowMain);
impl_uithread!(WindowMain);
impl_parentevents!(WindowMain);

impl WindowMain {
	/// Instantiates a new `WindowMain` object, to be created with
	/// [`HWND::CreateWindowEx`](crate::HWND::CreateWindowEx).
	pub fn new(opts: WindowMainOpts) -> WindowMain {
		Self {
			raw_dlg: RawDlg::Raw(
				RawMain::new(opts),
			),
		}
	}

	/// Instantiates a new `WindowMain` object, to be loaded from a dialog
	/// resource with [`HWND::GetDlgItem`](crate::HWND::GetDlgItem).
	pub fn new_dlg(
		dialog_id: u16,
		icon_id: Option<u16>,
		accel_table_id: Option<u16>) -> WindowMain
	{
		Self {
			raw_dlg: RawDlg::Dlg(
				DlgMain::new(dialog_id, icon_id, accel_table_id),
			),
		}
	}

	fn_base_ref!();

	/// Physically creates the window, then runs the main application loop. This
	/// method will block until the window is closed.
	///
	/// The `cmd_show` parameter defaults to
	/// [`co::SW::SHOW`](crate::co::SW::SHOW).
	///
	/// # Panics
	///
	/// Panics if the window is already created.
	pub fn run_main(&self, cmd_show: Option<co::SW>) -> ErrResult<i32> {
		if IsWindowsVistaOrGreater()? {
			SetProcessDPIAware()?;
		}

		InitCommonControls();
		create_ui_font()?;

		let res = match &self.raw_dlg {
			RawDlg::Raw(r) => r.run_main(cmd_show),
			RawDlg::Dlg(d) => d.run_main(cmd_show),
		};

		delete_ui_font()?; // cleanup
		res
	}
}
