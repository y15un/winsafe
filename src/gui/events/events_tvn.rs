use crate::co;
use crate::comctl::decl::{NMMOUSE, NMTREEVIEW, NMTVCUSTOMDRAW};
use crate::gui::base::Base;
use crate::gui::events::base_events_proxy::BaseEventsProxy;
use crate::kernel::decl::ErrResult;

/// Exposes tree view control
/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-notifications).
///
/// These event methods are just proxies to the
/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent window, who
/// is the real responsible for the child event handling.
///
/// You cannot directly instantiate this object, it is created internally by the
/// control.
#[cfg_attr(docsrs, doc(cfg(feature = "gui")))]
pub struct TreeViewEvents(BaseEventsProxy);

impl TreeViewEvents {
	pub(in crate::gui) fn new(parent_base: &Base, ctrl_id: u16) -> Self {
		Self(BaseEventsProxy::new(parent_base, ctrl_id))
	}

	pub_fn_nfy_ret0_param! { tvn_delete_item, co::TVN::DELETEITEM, NMTREEVIEW,
		/// [`TVN_DELETEITEM`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-deleteitem)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that an item is being
		/// deleted.
	}

	pub_fn_nfy_ret0_param! { tvn_item_changed, co::TVN::ITEMCHANGED, NMTREEVIEW,
		/// [`TVN_ITEMCHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemchanged)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that item attributes
		/// have changed.
	}

	pub_fn_nfy_retbool_param! { tvn_item_changing, co::TVN::ITEMCHANGING, NMTREEVIEW,
		/// [`TVN_ITEMCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemchanging)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that item attributes
		/// are about to change.
	}

	pub_fn_nfy_ret0_param! { tvn_item_expanded, co::TVN::ITEMEXPANDED, NMTREEVIEW,
		/// [`TVN_ITEMEXPANDED`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemexpanded)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that a parent item's
		/// list of child items has expanded or collapsed.
	}

	pub_fn_nfy_retbool_param! { tvn_item_expanding, co::TVN::ITEMEXPANDING, NMTREEVIEW,
		/// [`TVN_ITEMEXPANDING`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-itemexpanding)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that a parent item's
		/// list of child items is about to expand or collapse.
	}

	pub_fn_nfy_ret0_param! { tvn_sel_changed, co::TVN::SELCHANGED, NMTREEVIEW,
		/// [`TVN_SELCHANGED`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-selchanged)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that the selection has
		/// changed from one item to another.
	}

	pub_fn_nfy_retbool_param! { tvn_sel_changing, co::TVN::SELCHANGING, NMTREEVIEW,
		/// [`TVN_SELCHANGING`](https://docs.microsoft.com/en-us/windows/win32/controls/tvn-selchanging)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that the selection is
		/// about to change from one item to another.
	}

	pub_fn_nfy_reti32! { nm_click, co::NM::CLICK,
		/// [`NM_CLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-click-tree-view)
		/// notification.
		///
		/// Notifies the parent window of a tree-view control that the user has
		/// clicked the left mouse button within the control.
	}

	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw-tree-view)
	/// notification.
	///
	/// Sent by a tree-view control to notify its parent window about drawing operations.
	pub fn nm_custom_draw<F>(&self, func: F)
		where F: Fn(&NMTVCUSTOMDRAW) -> ErrResult<co::CDRF> + 'static,
	{
		self.0.add_nfy(co::NM::CUSTOMDRAW,
			move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<NMTVCUSTOMDRAW>() })?.0 as _)));
	}

	pub_fn_nfy_reti32! { nm_dbl_clk, co::NM::DBLCLK,
		/// [`NM_DBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-dblclk-tree-view)
		/// notification.
		///
		/// Notifies the parent window of a tree-view control that the user has
		/// double-clicked the left mouse button within the control.
	}

	pub_fn_nfy_ret0! { nm_kill_focus, co::NM::KILLFOCUS,
		/// [`NM_KILLFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-killfocus-tree-view)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that the control has
		/// lost the input focus.
	}

	pub_fn_nfy_reti32! { nm_r_click, co::NM::RCLICK,
		/// [`NM_RCLICK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rclick-tree-view)
		/// notification.
		///
		/// Notifies the parent window of a tree-view control that the user has
		/// clicked the right mouse button within the control.
	}

	pub_fn_nfy_reti32! { nm_r_dbl_clk, co::NM::RDBLCLK,
		/// [`NM_RDBLCLK`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-rdblclk-tree-view)
		/// notification.
		///
		/// Notifies the parent of a tree-view control that the user has
		/// double-clicked the right mouse button within the control.
	}

	pub_fn_nfy_ret0! { nm_return, co::NM::RETURN,
		/// [`NM_RETURN`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-return-tree-view-)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that the control has
		/// the input focus and that the user has pressed the ENTER key.
	}

	pub_fn_nfy_reti32_param! { nm_set_cursor, co::NM::SETCURSOR, NMMOUSE,
		/// [`NM_MOUSE`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setcursor-tree-view-)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that the control is
		/// setting the cursor in response to a
		/// [wm::SetCursor](crate::msg::wm::SetCursor) message.
	}

	pub_fn_nfy_ret0! { nm_set_focus, co::NM::SETFOCUS,
		/// [`NM_SETFOCUS`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-setfocus-tree-view-)
		/// notification.
		///
		/// Notifies a tree-view control's parent window that the control has
		/// received the input focus.
	}
}
