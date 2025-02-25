# WinSafe

[![Crates.io](https://img.shields.io/crates/v/winsafe.svg)](https://crates.io/crates/winsafe)
[![Docs.rs](https://docs.rs/winsafe/badge.svg)](https://docs.rs/winsafe)
[![Lines of code](https://tokei.rs/b1/github/rodrigocfd/winsafe)](https://github.com/rodrigocfd/winsafe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Windows API and GUI in safe, idiomatic Rust.

WinSafe has:

* high-level structs to build native Win32 GUI applications;
* low-level Win32 API constants, functions and structs related to GUI.

If you're looking for a comprehensive Win32 coverage, take a look at [winapi](https://crates.io/crates/winapi) or [windows](https://crates.io/crates/windows) crates, which are *unsafe*, but have everything.

WinSafe documentation:
* stable release: [docs.rs/winsafe](https://docs.rs/winsafe)
* master branch: [rodrigocfd.github.io/winsafe/winsafe](https://rodrigocfd.github.io/winsafe/winsafe/)

## Current status

This crate is still in alpha stage. Below is an estimated progress of feature groups:

| Feature group | Estimated progress |
| - | - |
| User windows (main, modal and control) | ![Progress](https://progress-bar.dev/100/) |
| Native controls | ![Progress](https://progress-bar.dev/85/) |
| Window messages | ![Progress](https://progress-bar.dev/75/) |
| Overall Win32 APIs | ![Progress](https://progress-bar.dev/35/) | |

## Usage

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
winsafe = { version = "0.0.9", features = [] }
```

Then you must enable the [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) you want to be included – these modules are named after native Windows DLL and library names, mostly.

The following Cargo features are available so far:

| Feature | Description |
| - | - |
| `advapi` | Advapi32.dll, for Windows Registry |
| `comctl` | ComCtl32.dll, for [Common Controls](https://docs.microsoft.com/en-us/windows/win32/api/_controls/) |
| `comdlg` | ComDlg32.dll, for the old [Common Dialogs](https://docs.microsoft.com/en-us/windows/win32/uxguide/win-common-dlg) |
| `dshow` | [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow) |
| `gdi` | Gdi32.dll, the [Windows GDI](https://docs.microsoft.com/en-us/windows/win32/gdi/windows-gdi) |
| **`gui`** | **The WinSafe high-level GUI structs** |
| `kernel` | Kernel32.dll, all others will include it |
| `msimg` | Msimg32.dll |
| `ole` | OLE and basic COM support |
| `oleaut` | [OLE Automation](https://docs.microsoft.com/en-us/windows/win32/api/_automat/) |
| `shell` | Shell32.dll, the COM-based [Windows Shell](https://docs.microsoft.com/en-us/windows/win32/shell/shell-entry) |
| `shlwapi` | Shlwapi.dll, for some [Shell](https://docs.microsoft.com/en-us/windows/win32/api/shlwapi/) functions |
| `user` | User32.dll, the basic Windows GUI support |
| `uxtheme` | UxTheme.dll, extended window theming |
| `version` | Version.dll, to manipulate *.exe version info |

Note that a Cargo feature may depend on other features, which will be enabled automatically.


## Example

**Note:** You can find several examples in the dedicated repo: [github.com/rodrigocfd/winsafe-examples](https://github.com/rodrigocfd/winsafe-examples)

WinSafe allows you to create windows in two ways:

* programmatically defining parameters; or
* [loading dialogs](https://github.com/rodrigocfd/winsafe-examples/tree/master/03_dialog_resources) from a `.res` file created with a WYSIWYG resource editor.

The [example below](https://github.com/rodrigocfd/winsafe-examples/tree/master/01_button_click/) creates a window  with a button programmatically. Note how the click event is handled with a closure:

![Example 01](https://raw.githubusercontent.com/rodrigocfd/winsafe-examples/master/01_button_click/screen.gif)

```toml
[dependencies]
winsafe = { version = "0.0.9", features = ["gui"] }
```

```rust
#![windows_subsystem = "windows"]

use winsafe::prelude::*;
use winsafe::{gui, POINT, SIZE, WinResult};

fn main() {
    let my = MyWindow::new();  // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}


#[derive(Clone)]
pub struct MyWindow {
    wnd:       gui::WindowMain, // responsible for managing the window
    btn_hello: gui::Button,     // a button
}

impl MyWindow {
    pub fn new() -> MyWindow {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
                title: "My window title".to_owned(),
                size: SIZE::new(300, 150),
                ..Default::default() // leave all other options as default
            },
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: POINT::new(20, 20),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_hello };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        self.btn_hello.on().bn_clicked({
            let wnd = self.wnd.clone(); // clone so it can be passed into the closure
            move || {
                wnd.hwnd().SetWindowText("Hello, world!")?;
                Ok(())
            }
        });
    }
}
```

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
