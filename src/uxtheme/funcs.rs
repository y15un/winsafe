#![allow(non_snake_case)]

use crate::uxtheme;

/// [`IsThemeActive`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemeactive)
/// static method.
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub fn IsThemeActive() -> bool {
	unsafe { uxtheme::ffi::IsThemeActive() != 0 }
}

/// [`IsAppThemed`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isappthemed)
/// static method.
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub fn IsAppThemed() -> bool {
	unsafe { uxtheme::ffi::IsAppThemed() != 0 }
}

/// [`IsCompositionActive`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-iscompositionactive)
/// static method.
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub fn IsCompositionActive() -> bool {
	unsafe { uxtheme::ffi::IsCompositionActive() != 0 }
}

/// [`IsThemeDialogTextureEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
/// static method.
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub fn IsThemeDialogTextureEnabled() -> bool {
	unsafe { uxtheme::ffi::IsThemeDialogTextureEnabled() != 0 }
}
