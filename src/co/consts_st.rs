use crate::co::{ACCESS_RIGHTS, CCM, WM};

pub_struct_const_wm! { SB,
	/// Status bar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-messages)
	/// (`u32`). Convertible to [`WM`](crate::co::WM).
	=>
	=>
	SETTEXT, WM::USER.0 + 11
	GETTEXT, WM::USER.0 + 13
	GETTEXTLENGTH, WM::USER.0 + 12
	SETPARTS, WM::USER.0 + 4
	GETPARTS, WM::USER.0 + 6
	GETBORDERS, WM::USER.0 + 7
	SETMINHEIGHT, WM::USER.0 + 8
	SIMPLE, WM::USER.0 + 9
	GETRECT, WM::USER.0 + 10
	ISSIMPLE, WM::USER.0 + 14
	SETICON, WM::USER.0 + 15
	SETTIPTEXT, WM::USER.0 + 17
	GETTIPTEXT, WM::USER.0 + 19
	GETICON, WM::USER.0 + 20
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	SETBKCOLOR, CCM::SETBKCOLOR.0
}

pub_struct_const_ws! { SBARS,
	/// Status bar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/status-bar-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	NONE, 0
	SIZEGRIP, 0x0100
	TOOLTIPS, 0x0800
}

pub_struct_const! { SBB, i32,
	/// [`GetScrollInfo`](crate::HWND::GetScrollInfo),
	/// [`SetScrollInfo`](crate::HWND::SetScrollInfo) and
	/// [`SetScrollRange`](crate::HWND::SetScrollRange) `nBar` (`i32`). Originally
	/// has `SB` prefix.
	=>
	HORZ, 0
	VERT, 1
	CTL, 2
}

pub_struct_const_nm! { SBN,
	/// Status bar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -880
	=>
	SIMPLEMODECHANGE, Self::FIRST.0 - 0
}

pub_struct_const! { SBT, u16,
	/// [`SB_GETTEXT`](crate::msg::sb::GetText),
	/// [`SB_GETTEXTLENGTH`](crate::msg::sb::GetTextLength) and
	/// [`SB_SETTEXT`](crate::msg::sb::SetText) drawing operation (`u16`).
	=>
	NONE, 0
	OWNERDRAW, 0x1000
	NOBORDERS, 0x0100
	POPOUT, 0x0200
	RTLREADING, 0x0400
	NOTABPARSING, 0x0800
}

pub_struct_const! { SB_REQ, u16,
	/// [`WM_HSCROLL`](crate::msg::wm::HScroll) and
	/// [`WM_VSCROLL`](crate::msg::wm::VScroll) request (`u16`). Originally has
	/// `SB` prefix.
	=>
	LINEUP, 0
	LINELEFT, 0
	LINEDOWN, 1
	LINERIGHT, 1
	PAGEUP, 2
	PAGELEFT, 2
	PAGEDOWN, 3
	PAGERIGHT, 3
	THUMBPOSITION, 4
	THUMBTRACK, 5
	TOP, 6
	LEFT, 6
	BOTTOM, 7
	RIGHT, 7
	ENDSCROLL, 8
}

pub_struct_const! { SC, u32,
	/// [`WM_SYSCOMMAND`](crate::msg::wm::SysCommand) type of system command
	/// requested (`u32`).
	=>
	CLOSE, 0xf060
	CONTEXTHELP, 0xf180
	DEFAULT, 0xf160
	HOTKEY, 0xf150
	HSCROLL, 0xf080
	ISSECURE, 0x00000001
	KEYMENU, 0xf100
	MAXIMIZE, 0xf030
	MINIMIZE, 0xf020
	MONITORPOWER, 0xf170
	MOUSEMENU, 0xf090
	MOVE, 0xf010
	NEXTWINDOW, 0xf040
	PREVWINDOW, 0xf050
	RESTORE, 0xf120
	SCREENSAVE, 0xf140
	SIZE, 0xf000
	TASKLIST, 0xf130
	VSCROLL, 0xf070
}

pub_struct_const! { SECTION, u32,
	/// Composes [`FILE_MAP`](crate::co::FILE_MAP) (`u32`).
	=>
	QUERY, 0x0001
	MAP_WRITE, 0x0002
	MAP_READ, 0x0004
	MAP_EXECUTE, 0x0008
	EXTEND_SIZE, 0x0010
	MAP_EXECUTE_EXPLICIT, 0x0020
	ALL_ACCESS, STANDARD_RIGHTS::REQUIRED.0 | Self::QUERY.0 | Self::MAP_WRITE.0 | Self::MAP_EXECUTE.0 | Self::EXTEND_SIZE.0
}

pub_struct_const! { SIF, u32,
	/// [`SCROLLINFO`](crate::SCROLLINFO) `fMask` (`u32`).
	=>
	RANGE, 0x0001
	PAGE, 0x0002
	POS, 0x0004
	DISABLENOSCROLL, 0x0008
	TRACKPOS, 0x0010
	ALL, Self::RANGE.0 | Self::PAGE.0 | Self::POS.0 | Self::TRACKPOS.0
}

pub_struct_const! { SIZE_R, u8,
	/// [`WM_SIZE`](crate::msg::wm::Size) request (`u8`).
	=>
	RESTORED, 0
	MINIMIZED, 1
	MAXIMIZED, 2
	MAXSHOW, 3
	MAXHIDE, 4
}

pub_struct_const! { SM, i32,
	/// [`GetSystemMetrics`](crate::GetSystemMetrics) `nIndex` (`i32`).
	=>
	CXSCREEN, 0
	CYSCREEN, 1
	CXVSCROLL, 2
	CYHSCROLL, 3
	CYCAPTION, 4
	CXBORDER, 5
	CYBORDER, 6
	CXDLGFRAME, 7
	CYDLGFRAME, 8
	CYVTHUMB, 9
	CXHTHUMB, 10
	CXICON, 11
	CYICON, 12
	CXCURSOR, 13
	CYCURSOR, 14
	CYMENU, 15
	CXFULLSCREEN, 16
	CYFULLSCREEN, 17
	CYKANJIWINDOW, 18
	MOUSEPRESENT, 19
	CYVSCROLL, 20
	CXHSCROLL, 21
	DEBUG, 22
	SWAPBUTTON, 23
	RESERVED1, 24
	RESERVED2, 25
	RESERVED3, 26
	RESERVED4, 27
	CXMIN, 28
	CYMIN, 29
	CXSIZE, 30
	CYSIZE, 31
	CXFRAME, 32
	CYFRAME, 33
	CXMINTRACK, 34
	CYMINTRACK, 35
	CXDOUBLECLK, 36
	CYDOUBLECLK, 37
	CXICONSPACING, 38
	CYICONSPACING, 39
	MENUDROPALIGNMENT, 40
	PENWINDOWS, 41
	DBCSENABLED, 42
	CMOUSEBUTTONS, 43
	CXFIXEDFRAME, Self::CXDLGFRAME.0
	CYFIXEDFRAME, Self::CYDLGFRAME.0
	CXSIZEFRAME, Self::CXFRAME.0
	CYSIZEFRAME, Self::CYFRAME.0
	SECURE, 44
	CXEDGE, 45
	CYEDGE, 46
	CXMINSPACING, 47
	CYMINSPACING, 48
	CXSMICON, 49
	CYSMICON, 50
	CYSMCAPTION, 51
	CXSMSIZE, 52
	CYSMSIZE, 53
	CXMENUSIZE, 54
	CYMENUSIZE, 55
	ARRANGE, 56
	CXMINIMIZED, 57
	CYMINIMIZED, 58
	CXMAXTRACK, 59
	CYMAXTRACK, 60
	CXMAXIMIZED, 61
	CYMAXIMIZED, 62
	NETWORK, 63
	CLEANBOOT, 67
	CXDRAG, 68
	CYDRAG, 69
	SHOWSOUNDS, 70
	CXMENUCHECK, 71
	CYMENUCHECK, 72
	SLOWMACHINE, 73
	MIDEASTENABLED, 74
	MOUSEWHEELPRESENT, 75
	XVIRTUALSCREEN, 76
	YVIRTUALSCREEN, 77
	CXVIRTUALSCREEN, 78
	CYVIRTUALSCREEN, 79
	CMONITORS, 80
	SAMEDISPLAYFORMAT, 81
	IMMENABLED, 82
	CXFOCUSBORDER, 83
	CYFOCUSBORDER, 84
	TABLETPC, 86
	MEDIACENTER, 87
	STARTER, 88
	SERVERR2, 89
	MOUSEHORIZONTALWHEELPRESENT, 91
	CXPADDEDBORDER, 92
	DIGITIZER, 94
	MAXIMUMTOUCHES, 95
	CMETRICS, 97
	REMOTESESSION, 0x1000
	SHUTTINGDOWN, 0x2000
	REMOTECONTROL, 0x2001
	CARETBLINKINGENABLED, 0x2002
	CONVERTIBLESLATEMODE, 0x2003
	SYSTEMDOCKED, 0x2004
}

pub_struct_const! { SORT, u16,
	/// Sort order
	/// [identifiers](https://docs.microsoft.com/en-us/windows/win32/intl/sort-order-identifiers)
	/// (`u16`).
	=>
	DEFAULT, 0x0
	INVARIANT_MATH, 0x1
	JAPANESE_XJIS, 0x0
	JAPANESE_UNICODE, 0x1
	JAPANESE_RADICALSTROKE, 0x4
	CHINESE_BIG5, 0x0
	CHINESE_PRCP, 0x0
	CHINESE_UNICODE, 0x1
	CHINESE_PRC, 0x2
	CHINESE_BOPOMOFO, 0x3
	CHINESE_RADICALSTROKE, 0x4
	KOREAN_KSC, 0x0
	KOREAN_UNICODE, 0x1
	GERMAN_PHONE_BOOK, 0x1
	HUNGARIAN_DEFAULT, 0x0
	HUNGARIAN_TECHNICAL, 0x1
	GEORGIAN_TRADITIONAL, 0x0
	GEORGIAN_MODERN, 0x1
}

pub_struct_const! { SPI, u32,
	/// [`SystemParametersInfo`](crate::SystemParametersInfo) `uiAction` (`u32`).
	=>
	GETBEEP, 0x0001
	SETBEEP, 0x0002
	GETMOUSE, 0x0003
	SETMOUSE, 0x0004
	GETBORDER, 0x0005
	SETBORDER, 0x0006
	GETKEYBOARDSPEED, 0x000a
	SETKEYBOARDSPEED, 0x000b
	LANGDRIVER, 0x000c
	ICONHORIZONTALSPACING, 0x000d
	GETSCREENSAVETIMEOUT, 0x000e
	SETSCREENSAVETIMEOUT, 0x000f
	GETSCREENSAVEACTIVE, 0x0010
	SETSCREENSAVEACTIVE, 0x0011
	GETGRIDGRANULARITY, 0x0012
	SETGRIDGRANULARITY, 0x0013
	SETDESKWALLPAPER, 0x0014
	SETDESKPATTERN, 0x0015
	GETKEYBOARDDELAY, 0x0016
	SETKEYBOARDDELAY, 0x0017
	ICONVERTICALSPACING, 0x0018
	GETICONTITLEWRAP, 0x0019
	SETICONTITLEWRAP, 0x001a
	GETMENUDROPALIGNMENT, 0x001b
	SETMENUDROPALIGNMENT, 0x001c
	SETDOUBLECLKWIDTH, 0x001d
	SETDOUBLECLKHEIGHT, 0x001e
	GETICONTITLELOGFONT, 0x001f
	SETDOUBLECLICKTIME, 0x0020
	SETMOUSEBUTTONSWAP, 0x0021
	SETICONTITLELOGFONT, 0x0022
	GETFASTTASKSWITCH, 0x0023
	SETFASTTASKSWITCH, 0x0024
	SETDRAGFULLWINDOWS, 0x0025
	GETDRAGFULLWINDOWS, 0x0026
	GETNONCLIENTMETRICS, 0x0029
	SETNONCLIENTMETRICS, 0x002a
	GETMINIMIZEDMETRICS, 0x002b
	SETMINIMIZEDMETRICS, 0x002c
	GETICONMETRICS, 0x002d
	SETICONMETRICS, 0x002e
	SETWORKAREA, 0x002f
	GETWORKAREA, 0x0030
	SETPENWINDOWS, 0x0031
	GETHIGHCONTRAST, 0x0042
	SETHIGHCONTRAST, 0x0043
	GETKEYBOARDPREF, 0x0044
	SETKEYBOARDPREF, 0x0045
	GETSCREENREADER, 0x0046
	SETSCREENREADER, 0x0047
	GETANIMATION, 0x0048
	SETANIMATION, 0x0049
	GETFONTSMOOTHING, 0x004a
	SETFONTSMOOTHING, 0x004b
	SETDRAGWIDTH, 0x004c
	SETDRAGHEIGHT, 0x004d
	SETHANDHELD, 0x004e
	GETLOWPOWERTIMEOUT, 0x004f
	GETPOWEROFFTIMEOUT, 0x0050
	SETLOWPOWERTIMEOUT, 0x0051
	SETPOWEROFFTIMEOUT, 0x0052
	GETLOWPOWERACTIVE, 0x0053
	GETPOWEROFFACTIVE, 0x0054
	SETLOWPOWERACTIVE, 0x0055
	SETPOWEROFFACTIVE, 0x0056
	SETCURSORS, 0x0057
	SETICONS, 0x0058
	GETDEFAULTINPUTLANG, 0x0059
	SETDEFAULTINPUTLANG, 0x005a
	SETLANGTOGGLE, 0x005b
	GETWINDOWSEXTENSION, 0x005c
	SETMOUSETRAILS, 0x005d
	GETMOUSETRAILS, 0x005e
	SETSCREENSAVERRUNNING, 0x0061
	SCREENSAVERRUNNING, Self::SETSCREENSAVERRUNNING.0
	GETFILTERKEYS, 0x0032
	SETFILTERKEYS, 0x0033
	GETTOGGLEKEYS, 0x0034
	SETTOGGLEKEYS, 0x0035
	GETMOUSEKEYS, 0x0036
	SETMOUSEKEYS, 0x0037
	GETSHOWSOUNDS, 0x0038
	SETSHOWSOUNDS, 0x0039
	GETSTICKYKEYS, 0x003a
	SETSTICKYKEYS, 0x003b
	GETACCESSTIMEOUT, 0x003c
	SETACCESSTIMEOUT, 0x003d
	GETSERIALKEYS, 0x003e
	SETSERIALKEYS, 0x003f
	GETSOUNDSENTRY, 0x0040
	SETSOUNDSENTRY, 0x0041
	GETSNAPTODEFBUTTON, 0x005f
	SETSNAPTODEFBUTTON, 0x0060
	GETMOUSEHOVERWIDTH, 0x0062
	SETMOUSEHOVERWIDTH, 0x0063
	GETMOUSEHOVERHEIGHT, 0x0064
	SETMOUSEHOVERHEIGHT, 0x0065
	GETMOUSEHOVERTIME, 0x0066
	SETMOUSEHOVERTIME, 0x0067
	GETWHEELSCROLLLINES, 0x0068
	SETWHEELSCROLLLINES, 0x0069
	GETMENUSHOWDELAY, 0x006a
	SETMENUSHOWDELAY, 0x006b
	GETWHEELSCROLLCHARS, 0x006c
	SETWHEELSCROLLCHARS, 0x006d
	GETSHOWIMEUI, 0x006e
	SETSHOWIMEUI, 0x006f
	GETMOUSESPEED, 0x0070
	SETMOUSESPEED, 0x0071
	GETSCREENSAVERRUNNING, 0x0072
	GETDESKWALLPAPER, 0x0073
	GETAUDIODESCRIPTION, 0x0074
	SETAUDIODESCRIPTION, 0x0075
	GETSCREENSAVESECURE, 0x0076
	SETSCREENSAVESECURE, 0x0077
	GETHUNGAPPTIMEOUT, 0x0078
	SETHUNGAPPTIMEOUT, 0x0079
	GETWAITTOKILLTIMEOUT, 0x007a
	SETWAITTOKILLTIMEOUT, 0x007b
	GETWAITTOKILLSERVICETIMEOUT, 0x007c
	SETWAITTOKILLSERVICETIMEOUT, 0x007d
	GETMOUSEDOCKTHRESHOLD, 0x007e
	SETMOUSEDOCKTHRESHOLD, 0x007f
	GETPENDOCKTHRESHOLD, 0x0080
	SETPENDOCKTHRESHOLD, 0x0081
	GETWINARRANGING, 0x0082
	SETWINARRANGING, 0x0083
	GETMOUSEDRAGOUTTHRESHOLD, 0x0084
	SETMOUSEDRAGOUTTHRESHOLD, 0x0085
	GETPENDRAGOUTTHRESHOLD, 0x0086
	SETPENDRAGOUTTHRESHOLD, 0x0087
	GETMOUSESIDEMOVETHRESHOLD, 0x0088
	SETMOUSESIDEMOVETHRESHOLD, 0x0089
	GETPENSIDEMOVETHRESHOLD, 0x008a
	SETPENSIDEMOVETHRESHOLD, 0x008b
	GETDRAGFROMMAXIMIZE, 0x008c
	SETDRAGFROMMAXIMIZE, 0x008d
	GETSNAPSIZING, 0x008e
	SETSNAPSIZING, 0x008f
	GETDOCKMOVING, 0x0090
	SETDOCKMOVING, 0x0091
}

pub_struct_const! { SPIF, u32,
	/// [`SystemParametersInfo`](crate::SystemParametersInfo) `fWinIni` (`u32`).
	=>
	ZERO, 0
	UPDATEINIFILE, 0x0001
	SENDWININICHANGE, 0x0002
	SENDCHANGE, Self::SENDWININICHANGE.0
}

pub_struct_const_ws! { SS,
	/// Label control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/static-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	LEFT, 0x0000_0000
	CENTER, 0x0000_0001
	RIGHT, 0x0000_0002
	ICON, 0x0000_0003
	BLACKRECT, 0x0000_0004
	GRAYRECT, 0x0000_0005
	WHITERECT, 0x0000_0006
	BLACKFRAME, 0x0000_0007
	GRAYFRAME, 0x0000_0008
	WHITEFRAME, 0x0000_0009
	USERITEM, 0x0000_000a
	SIMPLE, 0x0000_000b
	LEFTNOWORDWRAP, 0x0000_000c
	OWNERDRAW, 0x0000_000d
	BITMAP, 0x0000_000e
	ENHMETAFILE, 0x0000_000f
	ETCHEDHORZ, 0x0000_0010
	ETCHEDVERT, 0x0000_0011
	ETCHEDFRAME, 0x0000_0012
	TYPEMASK, 0x0000_001f
	REALSIZECONTROL, 0x0000_0040
	NOPREFIX, 0x0000_0080
	NOTIFY, 0x0000_0100
	CENTERIMAGE, 0x0000_0200
	RIGHTJUST, 0x0000_0400
	REALSIZEIMAGE, 0x0000_0800
	SUNKEN, 0x0000_1000
	EDITCONTROL, 0x0000_2000
	ENDELLIPSIS, 0x0000_4000
	PATHELLIPSIS, 0x0000_8000
	WORDELLIPSIS, 0x0000_c000
}

pub_struct_const! { STANDARD_RIGHTS, u32,
	/// Standard access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)
	/// (`u32`).
	=>
	REQUIRED, 0x000_f0000
	READ, ACCESS_RIGHTS::READ_CONTROL.0
	WRITE, ACCESS_RIGHTS::READ_CONTROL.0
	EXECUTE, ACCESS_RIGHTS::READ_CONTROL.0
	ALL, 0x001_f0000
}

pub_struct_const! { STAP, u32,
	/// [`GetThemeAppProperties`](crate::HTHEME::GetThemeAppProperties) return
	/// value (`u32`).
	=>
	ALLOW_NONCLIENT, 1 << 0
	ALLOW_CONTROLS, 1 << 1
	ALLOW_WEBCONTENT, 1 << 2
	VALIDBITS, Self::ALLOW_NONCLIENT.0 | Self::ALLOW_CONTROLS.0 | Self::ALLOW_WEBCONTENT.0
}

pub_struct_const! { STARTF, u32,
	/// [`CreateProcess`](crate::CreateProcess) `dwFlags` (`u32`).
	=>
	FORCEONFEEDBACK , 0x0000_0040
	FORCEOFFFEEDBACK, 0x0000_0080
	PREVENTPINNING, 0x0000_2000
	RUNFULLSCREEN, 0x0000_0020
	TITLEISAPPID, 0x0000_1000
	TITLEISLINKNAME, 0x0000_0800
	UNTRUSTEDSOURCE, 0x0000_8000
	USECOUNTCHARS, 0x0000_0008
	USEFILLATTRIBUTE, 0x0000_0010
	USEHOTKEY, 0x0000_0200
	USEPOSITION, 0x0000_0004
	USESHOWWINDOW, 0x0000_0001
	USESIZE, 0x0000_0002
	USESTDHANDLES, 0x0000_0100
}

pub_struct_const! { STATE_SYSTEM, u32,
	/// [`DATETIMEPICKERINFO`](crate::DATETIMEPICKERINFO) `stateCheck` and
	/// `stateButton`, [`TITLEBARINFOEX`](crate::TITLEBARINFOEX) `rgstate`
	/// (`u32`).
	=>
	UNAVAILABLE, 0x0000_0001
	SELECTED, 0x0000_0002
	FOCUSED, 0x0000_0004
	PRESSED, 0x0000_0008
	CHECKED, 0x0000_0010
	MIXED, 0x0000_0020
	INDETERMINATE, Self::MIXED.0
	READONLY, 0x0000_0040
	HOTTRACKED, 0x0000_0080
	DEFAULT, 0x0000_0100
	EXPANDED, 0x0000_0200
	COLLAPSED, 0x0000_0400
	BUSY, 0x0000_0800
	FLOATING, 0x0000_1000
	MARQUEED, 0x0000_2000
	ANIMATED, 0x0000_4000
	INVISIBLE, 0x0000_8000
	OFFSCREEN, 0x0001_0000
	SIZEABLE, 0x0002_0000
	MOVEABLE, 0x0004_0000
	SELFVOICING, 0x0008_0000
	FOCUSABLE, 0x0010_0000
	SELECTABLE, 0x0020_0000
	LINKED, 0x0040_0000
	TRAVERSED, 0x0080_0000
	MULTISELECTABLE, 0x0100_0000
	EXTSELECTABLE, 0x0200_0000
	ALERT_LOW, 0x0400_0000
	ALERT_MEDIUM, 0x0800_0000
	ALERT_HIGH, 0x1000_0000
	PROTECTED, 0x2000_0000
	VALID, 0x3fff_ffff
}

pub_struct_const_wm! { STM,
	/// Static control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	SETICON, 0x0170
	GETICON, 0x0171
	SETIMAGE, 0x0172
	GETIMAGE, 0x0173
}

pub_struct_const_cmd! { STN,
	/// Static control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).
	=>
	CLICKED, 0
	DBLCLK, 1
	ENABLE, 2
	DISABLE, 3
}

pub_struct_const! { SUBLANG, u16,
	/// Sublanguage
	/// [identifier](https://docs.microsoft.com/en-us/windows/win32/intl/language-identifier-constants-and-strings)
	/// (`u16`).
	=>
	NEUTRAL, 0x00
	DEFAULT, 0x01
	SYS_DEFAULT, 0x02
	CUSTOM_DEFAULT, 0x03
	CUSTOM_UNSPECIFIED, 0x04
	UI_CUSTOM_DEFAULT, 0x05
	AFRIKAANS_SOUTH_AFRICA, 0x01
	ALBANIAN_ALBANIA, 0x01
	ALSATIAN_FRANCE, 0x01
	AMHARIC_ETHIOPIA, 0x01
	ARABIC_SAUDI_ARABIA, 0x01
	ARABIC_IRAQ, 0x02
	ARABIC_EGYPT, 0x03
	ARABIC_LIBYA, 0x04
	ARABIC_ALGERIA, 0x05
	ARABIC_MOROCCO, 0x06
	ARABIC_TUNISIA, 0x07
	ARABIC_OMAN, 0x08
	ARABIC_YEMEN, 0x09
	ARABIC_SYRIA, 0x0a
	ARABIC_JORDAN, 0x0b
	ARABIC_LEBANON, 0x0c
	ARABIC_KUWAIT, 0x0d
	ARABIC_UAE, 0x0e
	ARABIC_BAHRAIN, 0x0f
	ARABIC_QATAR, 0x10
	ARMENIAN_ARMENIA, 0x01
	ASSAMESE_INDIA, 0x01
	AZERI_LATIN, 0x01
	AZERI_CYRILLIC, 0x02
	AZERBAIJANI_AZERBAIJAN_LATIN, 0x01
	AZERBAIJANI_AZERBAIJAN_CYRILLIC, 0x02
	BANGLA_INDIA, 0x01
	BANGLA_BANGLADESH, 0x02
	BASHKIR_RUSSIA, 0x01
	BASQUE_BASQUE, 0x01
	BELARUSIAN_BELARUS, 0x01
	BENGALI_INDIA, 0x01
	BENGALI_BANGLADESH, 0x02
	BOSNIAN_BOSNIA_HERZEGOVINA_LATIN, 0x05
	BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC, 0x08
	BRETON_FRANCE, 0x01
	BULGARIAN_BULGARIA, 0x01
	CATALAN_CATALAN, 0x01
	CENTRAL_KURDISH_IRAQ, 0x01
	CHEROKEE_CHEROKEE, 0x01
	CHINESE_TRADITIONAL, 0x01
	CHINESE_SIMPLIFIED, 0x02
	CHINESE_HONGKONG, 0x03
	CHINESE_SINGAPORE, 0x04
	CHINESE_MACAU, 0x05
	CORSICAN_FRANCE, 0x01
	CZECH_CZECH_REPUBLIC, 0x01
	CROATIAN_CROATIA, 0x01
	CROATIAN_BOSNIA_HERZEGOVINA_LATIN, 0x04
	DANISH_DENMARK, 0x01
	DARI_AFGHANISTAN, 0x01
	DIVEHI_MALDIVES, 0x01
	DUTCH, 0x01
	DUTCH_BELGIAN, 0x02
	ENGLISH_US, 0x01
	ENGLISH_UK, 0x02
	ENGLISH_AUS, 0x03
	ENGLISH_CAN, 0x04
	ENGLISH_NZ, 0x05
	ENGLISH_EIRE, 0x06
	ENGLISH_SOUTH_AFRICA, 0x07
	ENGLISH_JAMAICA, 0x08
	ENGLISH_CARIBBEAN, 0x09
	ENGLISH_BELIZE, 0x0a
	ENGLISH_TRINIDAD, 0x0b
	ENGLISH_ZIMBABWE, 0x0c
	ENGLISH_PHILIPPINES, 0x0d
	ENGLISH_INDIA, 0x10
	ENGLISH_MALAYSIA, 0x11
	ENGLISH_SINGAPORE, 0x12
	ESTONIAN_ESTONIA, 0x01
	FAEROESE_FAROE_ISLANDS, 0x01
	FILIPINO_PHILIPPINES, 0x01
	FINNISH_FINLAND, 0x01
	FRENCH, 0x01
	FRENCH_BELGIAN, 0x02
	FRENCH_CANADIAN, 0x03
	FRENCH_SWISS, 0x04
	FRENCH_LUXEMBOURG, 0x05
	FRENCH_MONACO, 0x06
	FRISIAN_NETHERLANDS, 0x01
	FULAH_SENEGAL, 0x02
	GALICIAN_GALICIAN, 0x01
	GEORGIAN_GEORGIA, 0x01
	GERMAN, 0x01
	GERMAN_SWISS, 0x02
	GERMAN_AUSTRIAN, 0x03
	GERMAN_LUXEMBOURG, 0x04
	GERMAN_LIECHTENSTEIN, 0x05
	GREEK_GREECE, 0x01
	GREENLANDIC_GREENLAND, 0x01
	GUJARATI_INDIA, 0x01
	HAUSA_NIGERIA_LATIN, 0x01
	HAWAIIAN_US, 0x01
	HEBREW_ISRAEL, 0x01
	HINDI_INDIA, 0x01
	HUNGARIAN_HUNGARY, 0x01
	ICELANDIC_ICELAND, 0x01
	IGBO_NIGERIA, 0x01
	INDONESIAN_INDONESIA, 0x01
	INUKTITUT_CANADA, 0x01
	INUKTITUT_CANADA_LATIN, 0x02
	IRISH_IRELAND, 0x02
	ITALIAN, 0x01
	ITALIAN_SWISS, 0x02
	JAPANESE_JAPAN, 0x01
	KANNADA_INDIA, 0x01
	KASHMIRI_SASIA, 0x02
	KASHMIRI_INDIA, 0x02
	KAZAK_KAZAKHSTAN, 0x01
	KHMER_CAMBODIA, 0x01
	KICHE_GUATEMALA, 0x01
	KINYARWANDA_RWANDA, 0x01
	KONKANI_INDIA, 0x01
	KOREAN, 0x01
	KYRGYZ_KYRGYZSTAN, 0x01
	LAO_LAO, 0x01
	LATVIAN_LATVIA, 0x01
	LITHUANIAN, 0x01
	LOWER_SORBIAN_GERMANY, 0x02
	LUXEMBOURGISH_LUXEMBOURG, 0x01
	MACEDONIAN_MACEDONIA, 0x01
	MALAY_MALAYSIA, 0x01
	MALAY_BRUNEI_DARUSSALAM, 0x02
	MALAYALAM_INDIA, 0x01
	MALTESE_MALTA, 0x01
	MAORI_NEW_ZEALAND, 0x01
	MAPUDUNGUN_CHILE, 0x01
	MARATHI_INDIA, 0x01
	MOHAWK_MOHAWK, 0x01
	MONGOLIAN_CYRILLIC_MONGOLIA, 0x01
	MONGOLIAN_PRC, 0x02
	NEPALI_INDIA, 0x02
	NEPALI_NEPAL, 0x01
	NORWEGIAN_BOKMAL, 0x01
	NORWEGIAN_NYNORSK, 0x02
	OCCITAN_FRANCE, 0x01
	ODIA_INDIA, 0x01
	ORIYA_INDIA, 0x01
	PASHTO_AFGHANISTAN, 0x01
	PERSIAN_IRAN, 0x01
	POLISH_POLAND, 0x01
	PORTUGUESE, 0x02
	PORTUGUESE_BRAZILIAN, 0x01
	PULAR_SENEGAL, 0x02
	PUNJABI_INDIA, 0x01
	PUNJABI_PAKISTAN, 0x02
	QUECHUA_BOLIVIA, 0x01
	QUECHUA_ECUADOR, 0x02
	QUECHUA_PERU, 0x03
	ROMANIAN_ROMANIA, 0x01
	ROMANSH_SWITZERLAND, 0x01
	RUSSIAN_RUSSIA, 0x01
	SAKHA_RUSSIA, 0x01
	SAMI_NORTHERN_NORWAY, 0x01
	SAMI_NORTHERN_SWEDEN, 0x02
	SAMI_NORTHERN_FINLAND, 0x03
	SAMI_LULE_NORWAY, 0x04
	SAMI_LULE_SWEDEN, 0x05
	SAMI_SOUTHERN_NORWAY, 0x06
	SAMI_SOUTHERN_SWEDEN, 0x07
	SAMI_SKOLT_FINLAND, 0x08
	SAMI_INARI_FINLAND, 0x09
	SANSKRIT_INDIA, 0x01
	SCOTTISH_GAELIC, 0x01
	SERBIAN_BOSNIA_HERZEGOVINA_LATIN, 0x06
	SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC, 0x07
	SERBIAN_MONTENEGRO_LATIN, 0x0b
	SERBIAN_MONTENEGRO_CYRILLIC, 0x0c
	SERBIAN_SERBIA_LATIN, 0x09
	SERBIAN_SERBIA_CYRILLIC, 0x0a
	SERBIAN_CROATIA, 0x01
	SERBIAN_LATIN, 0x02
	SERBIAN_CYRILLIC, 0x03
	SINDHI_INDIA, 0x01
	SINDHI_PAKISTAN, 0x02
	SINDHI_AFGHANISTAN, 0x02
	SINHALESE_SRI_LANKA, 0x01
	SOTHO_NORTHERN_SOUTH_AFRICA, 0x01
	SLOVAK_SLOVAKIA, 0x01
	SLOVENIAN_SLOVENIA, 0x01
	SPANISH, 0x01
	SPANISH_MEXICAN, 0x02
	SPANISH_MODERN, 0x03
	SPANISH_GUATEMALA, 0x04
	SPANISH_COSTA_RICA, 0x05
	SPANISH_PANAMA, 0x06
	SPANISH_DOMINICAN_REPUBLIC, 0x07
	SPANISH_VENEZUELA, 0x08
	SPANISH_COLOMBIA, 0x09
	SPANISH_PERU, 0x0a
	SPANISH_ARGENTINA, 0x0b
	SPANISH_ECUADOR, 0x0c
	SPANISH_CHILE, 0x0d
	SPANISH_URUGUAY, 0x0e
	SPANISH_PARAGUAY, 0x0f
	SPANISH_BOLIVIA, 0x10
	SPANISH_EL_SALVADOR, 0x11
	SPANISH_HONDURAS, 0x12
	SPANISH_NICARAGUA, 0x13
	SPANISH_PUERTO_RICO, 0x14
	SPANISH_US, 0x15
	SWAHILI_KENYA, 0x01
	SWEDISH, 0x01
	SWEDISH_FINLAND, 0x02
	SYRIAC_SYRIA, 0x01
	TAJIK_TAJIKISTAN, 0x01
	TAMAZIGHT_ALGERIA_LATIN, 0x02
	TAMAZIGHT_MOROCCO_TIFINAGH, 0x04
	TAMIL_INDIA, 0x01
	TAMIL_SRI_LANKA, 0x02
	TATAR_RUSSIA, 0x01
	TELUGU_INDIA, 0x01
	THAI_THAILAND, 0x01
	TIBETAN_PRC, 0x01
	TIGRIGNA_ERITREA, 0x02
	TIGRINYA_ERITREA, 0x02
	TIGRINYA_ETHIOPIA, 0x01
	TSWANA_BOTSWANA, 0x02
	TSWANA_SOUTH_AFRICA, 0x01
	TURKISH_TURKEY, 0x01
	TURKMEN_TURKMENISTAN, 0x01
	UIGHUR_PRC, 0x01
	UKRAINIAN_UKRAINE, 0x01
	UPPER_SORBIAN_GERMANY, 0x01
	URDU_PAKISTAN, 0x01
	URDU_INDIA, 0x02
	UZBEK_LATIN, 0x01
	UZBEK_CYRILLIC, 0x02
	VALENCIAN_VALENCIA, 0x02
	VIETNAMESE_VIETNAM, 0x01
	WELSH_UNITED_KINGDOM, 0x01
	WOLOF_SENEGAL, 0x01
	XHOSA_SOUTH_AFRICA, 0x01
	YAKUT_RUSSIA, 0x01
	YI_PRC, 0x01
	YORUBA_NIGERIA, 0x01
	ZULU_SOUTH_AFRICA, 0x01
}

pub_struct_const! { SW, i32,
	/// [`ShowWindow`](crate::HWND::ShowWindow) `nCmdShow` (`i32`).
	=>
	HIDE, 0
	SHOWNORMAL, 1
	SHOWMINIMIZED, 2
	SHOWMAXIMIZED, 3
	MAXIMIZE, 3
	SHOWNOACTIVATE, 4
	SHOW, 5
	MINIMIZE, 6
	SHOWMINNOACTIVE, 7
	SHOWNA, 8
	RESTORE, 9
	SHOWDEFAULT, 10
	FORCEMINIMIZE, 11
}

pub_struct_const! { SW_S, u8,
	/// [`WM_SHOWWINDOW`](crate::msg::wm::ShowWindow) status (`u8`). Originally
	/// has `SW` prefix.
	=>
	PARENTCLOSING, 1
	OTHERZOOM, 2
	PARENTOPENING, 3
	OTHERUNZOOM, 4
}

pub_struct_const! { SWP, u32,
	/// [`SetWindowPos`](crate::HWND::SetWindowPos) `uFlags` (`u32`).
	=>
	NOSIZE, 0x0001
	NOMOVE, 0x0002
	NOZORDER, 0x0004
	NOREDRAW, 0x0008
	NOACTIVATE, 0x0010
	FRAMECHANGED, 0x0020
	SHOWWINDOW, 0x0040
	HIDEWINDOW, 0x0080
	NOCOPYBITS, 0x0100
	NOOWNERZORDER, 0x0200
	NOSENDCHANGING, 0x0400
	DRAWFRAME, Self::FRAMECHANGED.0
	NOREPOSITION, Self::NOOWNERZORDER.0
	DEFERERASE, 0x2000
	ASYNCWINDOWPOS, 0x4000
}

pub_struct_const! { TA, u32,
	/// [`SetTextAlign`](crate::HDC::SetTextAlign) `align` (`u32`). Also includes
	/// constants with `VTA` prefix.
	=>
	NOUPDATECP, 0
	UPDATECP, 1
	LEFT, 0
	RIGHT, 2
	CENTER, 6
	TOP, 0
	BOTTOM, 8
	BASELINE, 24
	RTLREADING, 256
}

pub_struct_const! { TB, i32,
	/// [`NMTRBTHUMBPOSCHANGING`](crate::NMTRBTHUMBPOSCHANGING) `nReason`
	/// (`i32`).
	=>
	LINEUP, 0
	LINEDOWN, 1
	PAGEUP, 2
	PAGEDOWN, 3
	THUMBPOSITION, 4
	THUMBTRACK, 5
	TOP, 6
	BOTTOM, 7
	ENDTRACK, 8
}

pub_struct_const_wm! { TBM,
	/// Toolbar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM). Originally has `TB`
	/// prefix.
	=>
	=>
	ADDBITMAP, WM::USER.0 + 19
	ADDBUTTONS, WM::USER.0 + 68
	ADDSTRING, WM::USER.0 + 77
	AUTOSIZE, WM::USER.0 + 33
	BUTTONCOUNT, WM::USER.0 + 24
	BUTTONSTRUCTSIZE, WM::USER.0 + 30
	CHANGEBITMAP, WM::USER.0 + 43
	CHECKBUTTON, WM::USER.0 + 2
	COMMANDTOINDEX, WM::USER.0 + 25
	CUSTOMIZE, WM::USER.0 + 27
	DELETEBUTTON, WM::USER.0 + 22
	ENABLEBUTTON, WM::USER.0 + 1
	GETANCHORHIGHLIGHT, WM::USER.0 + 74
	GETBITMAP, WM::USER.0 + 44
	GETBITMAPFLAGS, WM::USER.0 + 41
	GETBUTTON, WM::USER.0 + 23
	GETBUTTONINFO, WM::USER.0 + 63
	GETBUTTONSIZE, WM::USER.0 + 58
	GETBUTTONTEXT, WM::USER.0 + 75
	GETCOLORSCHEME, CCM::GETCOLORSCHEME.0
	GETDISABLEDIMAGELIST, WM::USER.0 + 55
	GETEXTENDEDSTYLE, WM::USER.0 + 85
	GETHOTIMAGELIST, WM::USER.0 + 53
	GETHOTITEM, WM::USER.0 + 71
	GETIDEALSIZE, WM::USER.0 + 99
	GETIMAGELIST, WM::USER.0 + 49
	GETIMAGELISTCOUNT, WM::USER.0 + 98
	GETINSERTMARK, WM::USER.0 + 79
	GETINSERTMARKCOLOR, WM::USER.0 + 89
	GETITEMDROPDOWNRECT, WM::USER.0 + 103
	GETITEMRECT, WM::USER.0 + 29
	GETMAXSIZE, WM::USER.0 + 83
	GETMETRICS, WM::USER.0 + 101
	GETOBJECT, WM::USER.0 + 62
	GETPADDING, WM::USER.0 + 86
	GETPRESSEDIMAGELIST, WM::USER.0 + 105
	GETRECT, WM::USER.0 + 51
	GETROWS, WM::USER.0 + 40
	GETSTATE, WM::USER.0 + 18
	GETSTRING, WM::USER.0 + 91
	GETSTYLE, WM::USER.0 + 57
	GETTEXTROWS, WM::USER.0 + 61
	GETTOOLTIPS, WM::USER.0 + 35
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	HASACCELERATOR, WM::USER.0 + 95
	HIDEBUTTON, WM::USER.0 + 4
	HITTEST, WM::USER.0 + 69
	INDETERMINATE, WM::USER.0 + 5
	INSERTBUTTON, WM::USER.0 + 67
	INSERTMARKHITTEST, WM::USER.0 + 81
	ISBUTTONCHECKED, WM::USER.0 + 10
	ISBUTTONENABLED, WM::USER.0 + 9
	ISBUTTONHIDDEN, WM::USER.0 + 12
	ISBUTTONHIGHLIGHTED, WM::USER.0 + 14
	ISBUTTONINDETERMINATE, WM::USER.0 + 13
	ISBUTTONPRESSED, WM::USER.0 + 11
	LOADIMAGES, WM::USER.0 + 50
	MAPACCELERATOR, WM::USER.0 + 90
	MARKBUTTON, WM::USER.0 + 6
	MOVEBUTTON, WM::USER.0 + 82
	PRESSBUTTON, WM::USER.0 + 3
	REPLACEBITMAP, WM::USER.0 + 46
	SAVERESTORE, WM::USER.0 + 76
	SETANCHORHIGHLIGHT, WM::USER.0 + 73
	SETBITMAPSIZE, WM::USER.0 + 32
	SETBOUNDINGSIZE, WM::USER.0 + 93
	SETBUTTONINFO, WM::USER.0 + 64
	SETBUTTONSIZE, WM::USER.0 + 31
	SETBUTTONWIDTH, WM::USER.0 + 59
	SETCMDID, WM::USER.0 + 42
	SETCOLORSCHEME, CCM::SETCOLORSCHEME.0
	SETDISABLEDIMAGELIST, WM::USER.0 + 54
	SETDRAWTEXTFLAGS, WM::USER.0 + 70
	SETEXTENDEDSTYLE, WM::USER.0 + 84
	SETHOTIMAGELIST, WM::USER.0 + 52
	SETHOTITEM, WM::USER.0 + 72
	SETHOTITEM2, WM::USER.0 + 94
	SETIMAGELIST, WM::USER.0 + 48
	SETINDENT, WM::USER.0 + 47
	SETINSERTMARK, WM::USER.0 + 80
	SETINSERTMARKCOLOR, WM::USER.0 + 88
	SETLISTGAP, WM::USER.0 + 96
	SETMAXTEXTROWS, WM::USER.0 + 60
	SETMETRICS, WM::USER.0 + 102
	SETPADDING, WM::USER.0 + 87
	SETPARENT, WM::USER.0 + 37
	SETPRESSEDIMAGELIST, WM::USER.0 + 104
	SETREDRAWTEXTFLAGS, WM::USER.0 + 70
	SETROWS, WM::USER.0 + 39
	SETSTATE, WM::USER.0 + 17
	SETSTYLE, WM::USER.0 + 56
	SETTOOLTIPS, WM::USER.0 + 36
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	SETWINDOWTHEME, CCM::SETWINDOWTHEME.0
}

pub_struct_const_nm! { TBN,
	/// Toolbar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -700
	=>
	BEGINADJUST, Self::FIRST.0 - 3
	BEGINDRAG, Self::FIRST.0 - 1
	CUSTHELP, Self::FIRST.0 - 9
	DELETINGBUTTON, Self::FIRST.0 - 15
	DRAGOUT, Self::FIRST.0 - 14
	DRAGOVER, Self::FIRST.0 - 27
	DROPDOWN, Self::FIRST.0 - 10
	DUPACCELERATOR, Self::FIRST.0 - 25
	ENDADJUST, Self::FIRST.0 - 4
	ENDDRAG, Self::FIRST.0 - 2
	GETBUTTONINFO, Self::FIRST.0 - 20
	GETDISPINFO, Self::FIRST.0 - 17
	GETINFOTIP, Self::FIRST.0 - 19
	GETOBJECT, Self::FIRST.0 - 12
	HOTITEMCHANGE, Self::FIRST.0 - 13
	INITCUSTOMIZE, Self::FIRST.0 - 23
	MAPACCELERATOR, Self::FIRST.0 - 28
	QUERYDELETE, Self::FIRST.0 - 7
	QUERYINSERT, Self::FIRST.0 - 6
	RESET, Self::FIRST.0 - 5
	RESTORE, Self::FIRST.0 - 21
	SAVE, Self::FIRST.0 - 22
	TOOLBARCHANGE, Self::FIRST.0 - 8
	WRAPACCELERATOR, Self::FIRST.0 - 26
	WRAPHOTITEM, Self::FIRST.0 - 24
}

pub_struct_const_ws! { TBS,
	/// Trackbar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/trackbar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	AUTOTICKS, 0x0001
	VERT, 0x0002
	HORZ, 0x0000
	TOP, 0x0004
	BOTTOM, 0x0000
	LEFT, 0x0004
	RIGHT, 0x0000
	BOTH, 0x0008
	NOTICKS, 0x0010
	ENABLESELRANGE, 0x0020
	FIXEDLENGTH, 0x0040
	NOTHUMB, 0x0080
	TOOLTIPS, 0x0100
	REVERSED, 0x0200
	DOWNISLEFT, 0x0400
	NOTIFYBEFOREMOVE, 0x0800
	TRANSPARENTBKGND, 0x1000
}

pub_struct_const! { TCIS, u32,
	/// Tab control item
	/// [states](https://docs.microsoft.com/en-us/windows/win32/controls/tab-control-item-states)
	/// (`u32`).
	=>
	BUTTONPRESSED, 0x0001
	HIGHLIGHTED, 0x0002
}

pub_struct_const_wm! { TCM,
	/// Tab control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1300
	=>
	GETIMAGELIST, Self::FIRST.0 + 2
	SETIMAGELIST, Self::FIRST.0 + 3
	GETITEMCOUNT, Self::FIRST.0 + 4
	GETITEM, Self::FIRST.0 + 60
	SETITEM, Self::FIRST.0 + 61
	INSERTITEM, Self::FIRST.0 + 62
	DELETEITEM, Self::FIRST.0 + 8
	DELETEALLITEMS, Self::FIRST.0 + 9
	GETITEMRECT, Self::FIRST.0 + 10
	GETCURSEL, Self::FIRST.0 + 11
	SETCURSEL, Self::FIRST.0 + 12
	HITTEST, Self::FIRST.0 + 13
	SETITEMEXTRA, Self::FIRST.0 + 14
	ADJUSTRECT, Self::FIRST.0 + 40
	SETITEMSIZE, Self::FIRST.0 + 41
	REMOVEIMAGE, Self::FIRST.0 + 42
	SETPADDING, Self::FIRST.0 + 43
	GETROWCOUNT, Self::FIRST.0 + 44
	GETTOOLTIPS, Self::FIRST.0 + 45
	SETTOOLTIPS, Self::FIRST.0 + 46
	GETCURFOCUS, Self::FIRST.0 + 47
	SETCURFOCUS, Self::FIRST.0 + 48
	SETMINTABWIDTH, Self::FIRST.0 + 49
	DESELECTALL, Self::FIRST.0 + 50
	HIGHLIGHTITEM, Self::FIRST.0 + 51
	SETEXTENDEDSTYLE, Self::FIRST.0 + 52
	GETEXTENDEDSTYLE, Self::FIRST.0 + 53
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
}

pub_struct_const_nm! { TCN,
	/// Tab control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -550
	=>
	FOCUSCHANGE, Self::FIRST.0 - 4
	GETOBJECT, Self::FIRST.0 - 3
	KEYDOWN, Self::FIRST.0 - 0
	SELCHANGE, Self::FIRST.0 - 1
	SELCHANGING, Self::FIRST.0 - 2
}

pub_struct_const_ws! { TCS,
	/// Tab control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tab-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	SCROLLOPPOSITE, 0x0001
	BOTTOM, 0x0002
	RIGHT, 0x0002
	MULTISELECT, 0x0004
	FLATBUTTONS, 0x0008
	FORCEICONLEFT, 0x0010
	FORCELABELLEFT, 0x0020
	HOTTRACK, 0x0040
	VERTICAL, 0x0080
	TABS, 0x0000
	BUTTONS, 0x0100
	SINGLELINE, 0x0000
	MULTILINE, 0x0200
	RIGHTJUSTIFY, 0x0000
	FIXEDWIDTH, 0x0400
	RAGGEDRIGHT, 0x0800
	FOCUSONBUTTONDOWN, 0x1000
	OWNERDRAWFIXED, 0x2000
	TOOLTIPS, 0x4000
	FOCUSNEVER, 0x8000
}

pub_struct_const_wsex! { TCS_EX,
	/// Extended tab control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tab-control-extended-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).
	=>
	FLATSEPARATORS, 0x0000_0001
	REGISTERDROP, 0x0000_0002
}

pub_struct_const! { TH32CS, u32,
	/// [`CreateToolhelp32Snapshot`](crate::HPROCESSLIST) `dwFlags` (`u32`).
	=>
	SNAPHEAPLIST, 0x0000_0001
	SNAPPROCESS, 0x0000_0002
	SNAPTHREAD, 0x0000_0004
	SNAPMODULE, 0x0000_0008
	SNAPMODULE32, 0x0000_0010
	SNAPALL, Self::SNAPHEAPLIST.0 | Self::SNAPPROCESS.0 | Self::SNAPTHREAD.0 | Self::SNAPMODULE.0
	INHERIT, 0x8000_0000
}

pub_struct_const! { THREAD_CREATE, u32,
	/// [`CreateThread`](crate::HTHREAD::CreateThread) `dwFlags` (`u32`).
	/// Originally with no prefix.
	=>
	/// Originally just a zero.
	RUN_IMMEDIATELY, 0
	CREATE_SUSPENDED, 0x0000_0004
	STACK_SIZE_PARAM_IS_A_RESERVATION, 0x0001_0000
}

pub_struct_const! { TME, u32,
	/// [`TrackMouseEvent`](crate::TrackMouseEvent) `dwFlags` (`u32`).
	=>
	CANCEL, 0x8000_0000
	HOVER, 0x0000_0001
	LEAVE, 0x0000_0002
	NONCLIENT, 0x0000_0010
	QUERY, 0x4000_0000
}

pub_struct_const! { TPM, u32,
	/// [`TrackPopupMenu`](crate::HMENU::TrackPopupMenu) `uFlags` (`u32`).
	=>
	LEFTBUTTON, 0x0000
	RIGHTBUTTON, 0x0002
	LEFTALIGN, 0x0000
	CENTERALIGN, 0x0004
	RIGHTALIGN, 0x0008
	TOPALIGN, 0x0000
	VCENTERALIGN, 0x0010
	BOTTOMALIGN, 0x0020
	HORIZONTAL, 0x0000
	VERTICAL, 0x0040
	NONOTIFY, 0x0080
	RETURNCMD, 0x0100
	RECURSE, 0x0001
	HORPOSANIMATION, 0x0400
	HORNEGANIMATION, 0x0800
	VERPOSANIMATION, 0x1000
	VERNEGANIMATION, 0x2000
	NOANIMATION, 0x4000
	LAYOUTRTL, 0x8000
	WORKAREA, 0x10000
}

pub_struct_const_wm! { TRBM,
	/// Trackbar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM). Originally has `TBM`
	/// prefix.
	=>
	=>
	GETPOS, WM::USER.0
	GETRANGEMIN, WM::USER.0 + 1
	GETRANGEMAX, WM::USER.0 + 2
	GETTIC, WM::USER.0 + 3
	SETTIC, WM::USER.0 + 4
	SETPOS, WM::USER.0 + 5
	SETRANGE, WM::USER.0 + 6
	SETRANGEMIN, WM::USER.0 + 7
	SETRANGEMAX, WM::USER.0 + 8
	CLEARTICS, WM::USER.0 + 9
	SETSEL, WM::USER.0 + 10
	SETSELSTART, WM::USER.0 + 11
	SETSELEND, WM::USER.0 + 12
	GETPTICS, WM::USER.0 + 14
	GETTICPOS, WM::USER.0 + 15
	GETNUMTICS, WM::USER.0 + 16
	GETSELSTART, WM::USER.0 + 17
	GETSELEND, WM::USER.0 + 18
	CLEARSEL, WM::USER.0 + 19
	SETTICFREQ, WM::USER.0 + 20
	SETPAGESIZE, WM::USER.0 + 21
	GETPAGESIZE, WM::USER.0 + 22
	SETLINESIZE, WM::USER.0 + 23
	GETLINESIZE, WM::USER.0 + 24
	GETTHUMBRECT, WM::USER.0 + 25
	GETCHANNELRECT, WM::USER.0 + 26
	SETTHUMBLENGTH, WM::USER.0 + 27
	GETTHUMBLENGTH, WM::USER.0 + 28
	SETTOOLTIPS, WM::USER.0 + 29
	GETTOOLTIPS, WM::USER.0 + 30
	SETTIPSIDE, WM::USER.0 + 31
	SETBUDDY, WM::USER.0 + 32
	GETBUDDY, WM::USER.0 + 33
	SETPOSNOTIFY, WM::USER.0 + 34
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
}

pub_struct_const_nm! { TRBN,
	/// Trackbar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -1501
	=>
	THUMBPOSCHANGING, Self::FIRST.0 - 1
}

pub_struct_const_wm! { TVM,
	/// Tree view control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1100
	=>
	INSERTITEM, Self::FIRST.0 + 50
	DELETEITEM, Self::FIRST.0 + 1
	EXPAND, Self::FIRST.0 + 2
	GETITEMRECT, Self::FIRST.0 + 4
	GETCOUNT, Self::FIRST.0 + 5
	GETINDENT, Self::FIRST.0 + 6
	SETINDENT, Self::FIRST.0 + 7
	GETIMAGELIST, Self::FIRST.0 + 8
	SETIMAGELIST, Self::FIRST.0 + 9
	GETNEXTITEM, Self::FIRST.0 + 10
	SELECTITEM, Self::FIRST.0 + 11
	GETITEM, Self::FIRST.0 + 62
	SETITEM, Self::FIRST.0 + 63
	EDITLABEL, Self::FIRST.0 + 65
	GETEDITCONTROL, Self::FIRST.0 + 15
	GETVISIBLECOUNT, Self::FIRST.0 + 16
	HITTEST, Self::FIRST.0 + 17
	CREATEDRAGIMAGE, Self::FIRST.0 + 18
	SORTCHILDREN, Self::FIRST.0 + 19
	ENSUREVISIBLE, Self::FIRST.0 + 20
	SORTCHILDRENCB, Self::FIRST.0 + 21
	ENDEDITLABELNOW, Self::FIRST.0 + 22
	GETISEARCHSTRING, Self::FIRST.0 + 64
	SETTOOLTIPS, Self::FIRST.0 + 24
	GETTOOLTIPS, Self::FIRST.0 + 25
	SETINSERTMARK, Self::FIRST.0 + 26
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	SETITEMHEIGHT, Self::FIRST.0 + 27
	GETITEMHEIGHT, Self::FIRST.0 + 28
	SETBKCOLOR, Self::FIRST.0 + 29
	SETTEXTCOLOR, Self::FIRST.0 + 30
	GETBKCOLOR, Self::FIRST.0 + 31
	GETTEXTCOLOR, Self::FIRST.0 + 32
	SETSCROLLTIME, Self::FIRST.0 + 33
	GETSCROLLTIME, Self::FIRST.0 + 34
	SETINSERTMARKCOLOR, Self::FIRST.0 + 37
	GETINSERTMARKCOLOR, Self::FIRST.0 + 38
	SETBORDER, Self::FIRST.0 + 35
	GETITEMSTATE, Self::FIRST.0 + 39
	SETLINECOLOR, Self::FIRST.0 + 40
	GETLINECOLOR, Self::FIRST.0 + 41
	MAPACCIDTOHTREEITEM, Self::FIRST.0 + 42
	MAPHTREEITEMTOACCID, Self::FIRST.0 + 43
	SETEXTENDEDSTYLE, Self::FIRST.0 + 44
	GETEXTENDEDSTYLE, Self::FIRST.0 + 45
	SETAUTOSCROLLINFO, Self::FIRST.0 + 59
	SETHOT, Self::FIRST.0 + 58
	GETSELECTEDCOUNT, Self::FIRST.0 + 70
	SHOWINFOTIP, Self::FIRST.0 + 71
	GETITEMPARTRECT, Self::FIRST.0 + 72
}

pub_struct_const_nm! { TVN,
	/// Tree view control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -400
	=>
	SELCHANGING, Self::FIRST.0 - 50
	SELCHANGED, Self::FIRST.0 - 51
	GETDISPINFO, Self::FIRST.0 - 52
	SETDISPINFO, Self::FIRST.0 - 53
	ITEMEXPANDING, Self::FIRST.0 - 54
	ITEMEXPANDED, Self::FIRST.0 - 55
	BEGINDRAG, Self::FIRST.0 - 56
	BEGINRDRAG, Self::FIRST.0 - 57
	DELETEITEM, Self::FIRST.0 - 58
	BEGINLABELEDIT, Self::FIRST.0 - 59
	ENDLABELEDIT, Self::FIRST.0 - 60
	KEYDOWN, Self::FIRST.0 - 12
	GETINFOTIP, Self::FIRST.0 - 14
	SINGLEEXPAND, Self::FIRST.0 - 15
	ITEMCHANGING, Self::FIRST.0 - 17
	ITEMCHANGED, Self::FIRST.0 - 19
	ASYNCDRAW, Self::FIRST.0 - 20
}

pub_struct_const_ws! { TVS,
	/// Tree view control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-control-window-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	HASBUTTONS, 0x0001
	HASLINES, 0x0002
	LINESATROOT, 0x0004
	EDITLABELS, 0x0008
	DISABLEDRAGDROP, 0x0010
	SHOWSELALWAYS, 0x0020
	RTLREADING, 0x0040
	NOTOOLTIPS, 0x0080
	CHECKBOXES, 0x0100
	TRACKSELECT, 0x0200
	SINGLEEXPAND, 0x0400
	INFOTIP, 0x0800
	FULLROWSELECT, 0x1000
	NOSCROLL, 0x2000
	NONEVENHEIGHT, 0x4000
	NOHSCROLL, 0x8000
}

pub_struct_const_wsex! { TVS_EX,
	/// Extended tree view control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-control-window-extended-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).
	=>
	NOSINGLECOLLAPSE, 0x0001
	MULTISELECT, 0x0002
	DOUBLEBUFFER, 0x0004
	NOINDENTSTATE, 0x0008
	RICHTOOLTIP, 0x0010
	AUTOHSCROLL, 0x0020
	FADEINOUTEXPANDOS, 0x0040
	PARTIALCHECKBOXES, 0x0080
	EXCLUSIONCHECKBOXES, 0x0100
	DIMMEDCHECKBOXES, 0x0200
	DRAWIMAGEASYNC, 0x0400
}
