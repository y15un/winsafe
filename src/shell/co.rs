#![allow(non_camel_case_types)]

const_ordinary! { DROPEFFECT: u32: "shell";
	/// [`DROPEFFECT`](https://docs.microsoft.com/en-us/windows/win32/com/dropeffect-constants)
	/// constants (`u32`).
	=>
	=>
	NONE 0
	COPY 1
	MOVE 2
	LINK 4
	SCROLL 0x8000_0000
}

const_ordinary! { FO: u32: "shell";
	/// [`SHFILEOPSTRUCT`](crate::SHFILEOPSTRUCT) `wFunc` (`u32`).
	=>
	=>
	MOVE 0x0001
	COPY 0x0002
	DELETE 0x0003
	RENAME 0x0004
}

const_bitflag! { FOF: u16: "shell";
	/// [`SHFILEOPSTRUCT`](crate::SHFILEOPSTRUCT) `fFlags` (`u16`).
	=>
	=>
	MULTIDESTFILES 0x0001
	CONFIRMMOUSE 0x0002
	SILENT 0x0004
	RENAMEONCOLLISION 0x0008
	NOCONFIRMATION 0x0010
	WANTMAPPINGHANDLE 0x0020
	ALLOWUNDO 0x0040
	FILESONLY 0x0080
	SIMPLEPROGRESS 0x0100
	NOCONFIRMMKDIR 0x0200
	NOERRORUI 0x0400
	NOCOPYSECURITYATTRIBS 0x0800
	NORECURSION 0x1000
	NO_CONNECTED_ELEMENTS 0x2000
	WANTNUKEWARNING 0x4000
	NORECURSEREPARSE 0x8000
	NO_UI Self::SILENT.0 | Self::NOCONFIRMATION.0 | Self::NOERRORUI.0 | Self::NOCONFIRMMKDIR.0
}

const_bitflag! { FOS: u32: "shell";
	/// [`_FILEOPENDIALOGOPTIONS`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
	/// enumeration (`u32`).
	=>
	=>
	/// When saving a file prompt before overwriting an existing file of the
	/// same name. This is a default value for the Save dialog.
	OVERWRITEPROMPT 0x2
	/// In the Save dialog only allow the user to choose a file that has one of
	/// the file name extensions specified through
	/// [`IFileDialog::SetFileTypes`](crate::prelude::ShellIFileDialog::SetFileTypes).
	STRICTFILETYPES 0x4
	/// Don't change the current working directory.
	NOCHANGEDIR 0x8
	/// Present an Open dialog that offers a choice of folders rather than
	/// files.
	PICKFOLDERS 0x20
	/// Ensures that returned items are file system items
	/// ([`SFGAO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM)). Note that this
	/// does not apply to items returned by
	/// [`IFileDialog::GetCurrentSelection`](crate::prelude::ShellIFileDialog::GetCurrentSelection).
	FORCEFILESYSTEM 0x40
	/// Enables the user to choose any item in the Shell namespace not just
	/// those with [`SFGAO::STREAM`](crate::co::SFGAO::STREAM) or
	/// [`SFAGO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM) attributes. This
	/// flag cannot be combined with
	/// [`FOS::FORCEFILESYSTEM`](crate::co::FOS::FORCEFILESYSTEM).
	ALLNONSTORAGEITEMS 0x80
	/// Do not check for situations that would prevent an application from
	/// opening the selected file such as sharing violations or access denied
	/// errors.
	NOVALIDATE 0x100
	/// Enables the user to select multiple items in the open dialog. Note that
	/// when this flag is set the [`IFileOpenDialog`](crate::IFileOpenDialog)
	/// interface must be used to retrieve those items.
	ALLOWMULTISELECT 0x200
	/// The item returned must be in an existing folder. This is a default
	/// value.
	PATHMUSTEXIST 0x800
	/// The item returned must exist. This is a default value for the Open
	/// dialog.
	FILEMUSTEXIST 0x1000
	/// Prompt for creation if the item returned in the save dialog does not
	/// exist. Note that this does not actually create the item.
	CREATEPROMPT 0x2000
	/// In the case of a sharing violation when an application is opening a
	/// file call the application back through
	/// [`OnShareViolation`](crate::prelude::ShellIFileDialogEvents::OnShareViolation)
	/// for guidance. This flag is overridden by
	/// [`FOS::NOVALIDATE`](crate::co::FOS::NOVALIDATE).
	SHAREAWARE 0x4000
	/// Do not return read-only items. This is a default value for the Save
	/// dialog.
	NOREADONLYRETURN 0x8000
	/// Do not test whether creation of the item as specified in the Save dialog
	/// will be successful. If this flag is not set the calling application
	/// must handle errors such as denial of access discovered when the item
	/// is created.
	NOTESTFILECREATE 0x1_0000
	/// Hide the list of places from which the user has recently opened or saved
	/// items. This value is not supported as of Windows 7.
	HIDEMRUPLACES 0x2_0000
	/// Hide items shown by default in the view's navigation pane. This flag is
	/// often used in conjunction with the
	/// [`IFileDialog::AddPlace`](crate::prelude::ShellIFileDialog::AddPlace)
	/// method, to hide standard locations and replace them with custom
	/// locations.
	///
	/// Windows 7 and later. Hide all of the standard namespace locations (such
	/// as Favorites Libraries Computer and Network) shown in the navigation
	/// pane.
	///
	/// Windows Vista. Hide the contents of the Favorite Links tree in the
	/// navigation pane. Note that the category itself is still displayed but
	/// shown as empty.
	HIDEPINNEDPLACES 0x4_0000
	/// Shortcuts should not be treated as their target items. This allows an
	/// application to open a .lnk file rather than what that file is a shortcut
	/// to.
	NODEREFERENCELINKS 0x10_0000
	/// (This constant has no official documentation.)
	OKBUTTONNEEDSINTERACTION 0x20_0000
	/// Do not add the item being opened or saved to the recent documents list
	/// ([`SHAddToRecentDocs`](crate::SHAddToRecentDocs)).
	DONTADDTORECENT 0x200_0000
	/// Include hidden and system items.
	FORCESHOWHIDDEN 0x1000_0000
	/// Indicates to the Save As dialog box that it should open in expanded
	/// mode. Expanded mode is the mode that is set and unset by clicking the
	/// button in the lower-left corner of the Save As dialog box that switches
	/// between Browse Folders and Hide Folders when clicked. This value is not
	/// supported as of Windows 7.
	DEFAULTNOMINIMODE 0x2000_0000
	/// Indicates to the Open dialog box that the preview pane should always be
	/// displayed.
	FORCEPREVIEWPANEON 0x4000_0000
	/// Indicates that the caller is opening a file as a stream
	/// ([`BHID_Stream`](crate::prelude::ShellIShellItem::BindToHandler)) so
	/// there is no need to download that file.
	SUPPORTSTREAMABLEITEMS 0x8000_0000
}

const_ordinary! { FDAP: u32: "shell";
	/// [`FDAP`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-fdap)
	/// enumeration (`u32`).
	=>
	=>
	BOTTOM 0
	TOP 1
}

const_bitflag! { NIF: u32: "shell";
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `uFlags` (`u32`).
	=>
	=>
	MESSAGE 0x0000_0001
	ICON 0x0000_0002
	TIP 0x0000_0004
	STATE 0x0000_0008
	INFO 0x0000_0010
	GUID 0x0000_0020
	REALTIME 0x0000_0040
	SHOWTIP 0x0000_0080
}

const_bitflag! { NIIF: u32: "shell";
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `dwInfoFlags` (`u32`).
	=>
	=>
	NONE 0x0000_0000
	INFO 0x0000_0001
	WARNING 0x0000_0002
	ERROR 0x0000_0003
	USER 0x0000_0004
	NOSOUND 0x0000_0010
	LARGE_ICON 0x0000_0020
	RESPECT_QUIET_TIME 0x0000_0080
}

const_ordinary! { NIM: u32: "shell";
	/// [`Shell_NotifyIcon`](crate::Shell_NotifyIcon) `message` (`u32`).
	=>
	=>
	ADD 0x0000_0000
	MODIFY 0x0000_0001
	DELETE 0x0000_0002
	SETFOCUS 0x0000_0003
	SETVERSION 0x0000_0004
}

const_bitflag! { NIS: u32: "shell";
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `dwState` and `dwStateFlags`
	/// (`u32`).
	=>
	=>
	HIDDEN 0x0000_0001
	SHAREDICON 0x0000_0002
}

const_ordinary! { SE_ERR: u32: "shell";
	/// [`HWND::ShellExecute`](crate::prelude::ShellHwnd::ShellExecute) return
	/// value (`u32`).
	=>
	=>
	FILE_NOT_FOUND 2
	PATH_NOT_FOUND 3
	BAD_FORMAT 11

	ACCESSDENIED 5
	OOM 8
	DLLNOTFOUND 32

	SHARE 26
	ASSOCINCOMPLETE 27
	DDETIMEOUT 28
	DDEFAIL 29
	DDEBUSY 30
	NOASSOC 31
}

const_bitflag! { SFGAO: u32: "shell";
	/// [`SFGAO`](https://docs.microsoft.com/en-us/windows/win32/shell/sfgao)
	/// constants (`u32`).
	=>
	=>
	CANCOPY DROPEFFECT::COPY.0
	CANMOVE DROPEFFECT::MOVE.0
	CANLINK DROPEFFECT::LINK.0
	STORAGE 0x0000_0008
	CANRENAME 0x0000_0010
	CANDELETE 0x0000_0020
	HASPROPSHEET 0x0000_0040
	DROPTARGET 0x0000_0100
	CAPABILITYMASK 0x0000_0177
	SYSTEM 0x0000_1000
	ENCRYPTED 0x0000_2000
	ISSLOW 0x0000_4000
	GHOSTED 0x0000_8000
	LINK 0x0001_0000
	SHARE 0x0002_0000
	READONLY 0x0004_0000
	HIDDEN 0x0008_0000
	FILESYSANCESTOR 0x1000_0000
	FOLDER 0x2000_0000
	FILESYSTEM 0x4000_0000
	HASSUBFOLDER 0x8000_0000
	CONTENTSMASK 0x8000_0000
	VALIDATE 0x0100_0000
	REMOVABLE 0x0200_0000
	COMPRESSED 0x0400_0000
	BROWSABLE 0x0800_0000
	NONENUMERATED 0x0010_0000
	NEWCONTENT 0x0020_0000
	CANMONIKER 0x0040_0000
	HASSTORAGE 0x0040_0000
	STREAM 0x0040_0000
	STORAGEANCESTOR 0x0080_0000
	STORAGECAPMASK 0x70c5_0008
	PKEYSFGAOMASK 0x8104_4000
}

const_ordinary! { SHARD: u32: "shell";
	/// [`SHARD`](https://docs.microsoft.com/en-us/windows/win32/api/shlobj_core/ne-shlobj_core-shard)
	/// enumeration (`u32`).
	=>
	=>
	PIDL 0x0000_0001
	PATHA 0x0000_0002
	PATHW 0x0000_0003
	APPIDINFO 0x0000_0004
	APPIDINFOIDLIST 0x0000_0005
	LINK 0x0000_0006
	APPIDINFOLINK 0x0000_0007
	SHELLITEM 0x0000_0008
}

const_bitflag! { SHGFI: u32: "shell";
	/// [`SHGetFileInfo`](crate::SHGetFileInfo) `flags` (`u32`).
	=>
	=>
	ICON 0x0000_0100
	DISPLAYNAME 0x0000_0200
	TYPENAME 0x0000_0400
	ATTRIBUTES 0x0000_0800
	ICONLOCATION 0x0000_1000
	EXETYPE 0x0000_2000
	SYSICONINDEX 0x0000_4000
	LINKOVERLAY 0x0000_8000
	SELECTED 0x0001_0000
	ATTR_SPECIFIED 0x0002_0000
	LARGEICON 0x0000_0000
	SMALLICON 0x0000_0001
	OPENICON 0x0000_0002
	SHELLICONSIZE 0x0000_0004
	PIDL 0x0000_0008
	USEFILEATTRIBUTES 0x0000_0010
	ADDOVERLAYS 0x0000_0020
	OVERLAYINDEX 0x0000_0040
}

const_ordinary! { SIGDN: u32: "shell";
	/// [`SIGDN`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-sigdn)
	/// enumeration (`u32`).
	=>
	=>
	/// Returns the display name relative to the parent folder. In UI this name
	/// is generally ideal for display to the user.
	NORMALDISPLAY 0
	/// Returns the parsing name relative to the parent folder. This name is not
	/// suitable for use in UI.
	PARENTRELATIVEPARSING 0x8001_8001
	/// Returns the parsing name relative to the desktop. This name is not
	/// suitable for use in UI.
	DESKTOPABSOLUTEPARSING 0x8002_8000
	/// Returns the editing name relative to the parent folder. In UI this name
	/// is suitable for display to the user.
	PARENTRELATIVEEDITING 0x8003_1001
	/// Returns the editing name relative to the desktop. In UI this name is
	/// suitable for display to the user.
	DESKTOPABSOLUTEEDITING 0x8004_c000
	/// Returns the item's file system path if it has one. Only items that
	/// report [`SFGAO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM) have a file
	/// system path. When an item does not have a file system path a call to
	/// [`IShellItem::GetDisplayName`](crate::prelude::ShellIShellItem::GetDisplayName)
	/// on that item will fail. In UI this name is suitable for display to the
	/// user in some cases but note that it might not be specified for all
	/// items.
	FILESYSPATH 0x8005_8000
	/// Returns the item's URL if it has one. Some items do not have a URL and
	/// in those cases a call to
	/// [`IShellItem::GetDisplayName`](crate::prelude::ShellIShellItem::GetDisplayName)
	/// will fail. This name is suitable for display to the user in some cases,
	/// but note that it might not be specified for all items.
	URL 0x8006_8000
	/// Returns the path relative to the parent folder in a friendly format as
	/// displayed in an address bar. This name is suitable for display to the
	/// user.
	PARENTRELATIVEFORADDRESSBAR 0x8007_c001
	/// Returns the path relative to the parent folder.
	PARENTRELATIVE 0x8008_0001
	/// Introduced in Windows 8.
	PARENTRELATIVEFORUI 0x8009_4001
}

const_ordinary! { STPFLAG: u32: "shell";
	/// [`STPFLAG`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-stpflag)
	/// enumeration (`u32`).
	=>
	=>
	NONE 0
	USEAPPTHUMBNAILALWAYS 0x1
	USEAPPTHUMBNAILWHENACTIVE 0x2
	USEAPPPEEKALWAYS 0x4
	USEAPPPEEKWHENACTIVE 0x8
}

const_ordinary! { TBPF: u32: "shell";
	/// [`ITaskbarList3::SetProgressState`](crate::prelude::ShellITaskbarList3::SetProgressState)
	/// `tbpFlags` (`u32`).
	=>
	=>
	/// Stops displaying progress and returns the button to its normal state.
	/// Call this method with this flag to dismiss the progress bar when the
	/// operation is complete or canceled.
	NOPROGRESS 0
	/// The progress indicator does not grow in size but cycles repeatedly
	/// along the length of the taskbar button. This indicates activity without
	/// specifying what proportion of the progress is complete. Progress is
	/// taking place but there is no prediction as to how long the operation
	/// will take.
	INDETERMINATE 0x1
	/// The progress indicator grows in size from left to right in proportion to
	/// the estimated amount of the operation completed. This is a determinate
	/// progress indicator; a prediction is being made as to the duration of the
	/// operation.
	NORMAL 0x2
	/// The progress indicator turns red to show that an error has occurred in
	/// one of the windows that is broadcasting progress. This is a determinate
	/// state. If the progress indicator is in the indeterminate state it
	/// switches to a red determinate display of a generic percentage not
	/// indicative of actual progress.
	ERROR 0x4
	/// The progress indicator turns yellow to show that progress is currently
	/// stopped in one of the windows but can be resumed by the user. No error
	/// condition exists and nothing is preventing the progress from continuing.
	/// This is a determinate state. If the progress indicator is in the
	/// indeterminate state it switches to a yellow determinate display of a
	/// generic percentage not indicative of actual progress.
	PAUSED 0x8
}
