// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../../gir-files-gstreamer
// from ../../gir-files-gtk
// from ../../libclapper-rs
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use clapper_sys as clapper;
use gio_sys as gio;
use glib_sys as glib;
use gtk_sys as gtk;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, off_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Constants
pub const CLAPPER_GTK_MAJOR_VERSION: c_int = 0;
pub const CLAPPER_GTK_MICRO_VERSION: c_int = 0;
pub const CLAPPER_GTK_MINOR_VERSION: c_int = 7;
pub const CLAPPER_GTK_VERSION_S: &[u8] = b"0.7.0\0";

// Flags
pub type ClapperGtkVideoActionMask = c_uint;
pub const CLAPPER_GTK_VIDEO_ACTION_NONE: ClapperGtkVideoActionMask = 0;
pub const CLAPPER_GTK_VIDEO_ACTION_REVEAL_OVERLAYS: ClapperGtkVideoActionMask = 1;
pub const CLAPPER_GTK_VIDEO_ACTION_TOGGLE_PLAY: ClapperGtkVideoActionMask = 2;
pub const CLAPPER_GTK_VIDEO_ACTION_TOGGLE_FULLSCREEN: ClapperGtkVideoActionMask = 4;
pub const CLAPPER_GTK_VIDEO_ACTION_SEEK_REQUEST: ClapperGtkVideoActionMask = 8;
pub const CLAPPER_GTK_VIDEO_ACTION_ANY: ClapperGtkVideoActionMask = 67108863;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkBillboardClass {
    pub parent_class: ClapperGtkContainerClass,
}

impl ::std::fmt::Debug for ClapperGtkBillboardClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkBillboardClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkContainerClass {
    pub parent_class: gtk::GtkWidgetClass,
    pub padding: [gpointer; 4],
}

impl ::std::fmt::Debug for ClapperGtkContainerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkContainerClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkExtraMenuButtonClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for ClapperGtkExtraMenuButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkExtraMenuButtonClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkLeadContainerClass {
    pub parent_class: ClapperGtkContainerClass,
    pub padding: [gpointer; 4],
}

impl ::std::fmt::Debug for ClapperGtkLeadContainerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkLeadContainerClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkNextItemButtonClass {
    pub parent_class: gtk::GtkButtonClass,
}

impl ::std::fmt::Debug for ClapperGtkNextItemButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkNextItemButtonClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkPreviousItemButtonClass {
    pub parent_class: gtk::GtkButtonClass,
}

impl ::std::fmt::Debug for ClapperGtkPreviousItemButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkPreviousItemButtonClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkSeekBarClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for ClapperGtkSeekBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkSeekBarClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkSimpleControlsClass {
    pub parent_class: ClapperGtkContainerClass,
}

impl ::std::fmt::Debug for ClapperGtkSimpleControlsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkSimpleControlsClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkTitleHeaderClass {
    pub parent_class: ClapperGtkLeadContainerClass,
}

impl ::std::fmt::Debug for ClapperGtkTitleHeaderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkTitleHeaderClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkTitleLabelClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for ClapperGtkTitleLabelClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkTitleLabelClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkToggleFullscreenButtonClass {
    pub parent_class: gtk::GtkButtonClass,
}

impl ::std::fmt::Debug for ClapperGtkToggleFullscreenButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkToggleFullscreenButtonClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkTogglePlayButtonClass {
    pub parent_class: gtk::GtkButtonClass,
}

impl ::std::fmt::Debug for ClapperGtkTogglePlayButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkTogglePlayButtonClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkVideoClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for ClapperGtkVideoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkVideoClass @ {self:p}"))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

// Classes
#[repr(C)]
pub struct ClapperGtkBillboard {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkBillboard {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkBillboard @ {self:p}"))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkContainer {
    pub parent_instance: gtk::GtkWidget,
}

impl ::std::fmt::Debug for ClapperGtkContainer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkContainer @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkExtraMenuButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkExtraMenuButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkExtraMenuButton @ {self:p}"))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperGtkLeadContainer {
    pub parent_instance: ClapperGtkContainer,
}

impl ::std::fmt::Debug for ClapperGtkLeadContainer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkLeadContainer @ {self:p}"))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkNextItemButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkNextItemButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkNextItemButton @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkPreviousItemButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkPreviousItemButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkPreviousItemButton @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkSeekBar {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkSeekBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkSeekBar @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkSimpleControls {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkSimpleControls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkSimpleControls @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkTitleHeader {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkTitleHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkTitleHeader @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkTitleLabel {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkTitleLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkTitleLabel @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkToggleFullscreenButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkToggleFullscreenButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkToggleFullscreenButton @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkTogglePlayButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkTogglePlayButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkTogglePlayButton @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
pub struct ClapperGtkVideo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperGtkVideo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperGtkVideo @ {self:p}"))
            .finish()
    }
}

#[link(name = "clapper-gtk-0.0")]
extern "C" {

    //=========================================================================
    // ClapperGtkVideoActionMask
    //=========================================================================
    pub fn clapper_gtk_video_action_mask_get_type() -> GType;

    //=========================================================================
    // ClapperGtkBillboard
    //=========================================================================
    pub fn clapper_gtk_billboard_get_type() -> GType;
    pub fn clapper_gtk_billboard_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_billboard_announce_speed(billboard: *mut ClapperGtkBillboard);
    pub fn clapper_gtk_billboard_announce_volume(billboard: *mut ClapperGtkBillboard);
    pub fn clapper_gtk_billboard_pin_message(
        billboard: *mut ClapperGtkBillboard,
        icon_name: *const c_char,
        message: *const c_char,
    );
    pub fn clapper_gtk_billboard_post_message(
        billboard: *mut ClapperGtkBillboard,
        icon_name: *const c_char,
        message: *const c_char,
    );
    pub fn clapper_gtk_billboard_unpin_pinned_message(billboard: *mut ClapperGtkBillboard);

    //=========================================================================
    // ClapperGtkContainer
    //=========================================================================
    pub fn clapper_gtk_container_get_type() -> GType;
    pub fn clapper_gtk_container_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_container_get_adaptive_height(container: *mut ClapperGtkContainer) -> c_int;
    pub fn clapper_gtk_container_get_adaptive_width(container: *mut ClapperGtkContainer) -> c_int;
    pub fn clapper_gtk_container_get_child(
        container: *mut ClapperGtkContainer,
    ) -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_container_get_height_target(container: *mut ClapperGtkContainer) -> c_int;
    pub fn clapper_gtk_container_get_width_target(container: *mut ClapperGtkContainer) -> c_int;
    pub fn clapper_gtk_container_set_adaptive_height(
        container: *mut ClapperGtkContainer,
        height: c_int,
    );
    pub fn clapper_gtk_container_set_adaptive_width(
        container: *mut ClapperGtkContainer,
        width: c_int,
    );
    pub fn clapper_gtk_container_set_child(
        container: *mut ClapperGtkContainer,
        child: *mut gtk::GtkWidget,
    );
    pub fn clapper_gtk_container_set_height_target(
        container: *mut ClapperGtkContainer,
        height: c_int,
    );
    pub fn clapper_gtk_container_set_width_target(
        container: *mut ClapperGtkContainer,
        width: c_int,
    );

    //=========================================================================
    // ClapperGtkExtraMenuButton
    //=========================================================================
    pub fn clapper_gtk_extra_menu_button_get_type() -> GType;
    pub fn clapper_gtk_extra_menu_button_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_extra_menu_button_get_can_open_subtitles(
        button: *mut ClapperGtkExtraMenuButton,
    ) -> gboolean;
    pub fn clapper_gtk_extra_menu_button_get_speed_visible(
        button: *mut ClapperGtkExtraMenuButton,
    ) -> gboolean;
    pub fn clapper_gtk_extra_menu_button_get_volume_visible(
        button: *mut ClapperGtkExtraMenuButton,
    ) -> gboolean;
    pub fn clapper_gtk_extra_menu_button_set_can_open_subtitles(
        button: *mut ClapperGtkExtraMenuButton,
        allowed: gboolean,
    );
    pub fn clapper_gtk_extra_menu_button_set_speed_visible(
        button: *mut ClapperGtkExtraMenuButton,
        visible: gboolean,
    );
    pub fn clapper_gtk_extra_menu_button_set_volume_visible(
        button: *mut ClapperGtkExtraMenuButton,
        visible: gboolean,
    );

    //=========================================================================
    // ClapperGtkLeadContainer
    //=========================================================================
    pub fn clapper_gtk_lead_container_get_type() -> GType;
    pub fn clapper_gtk_lead_container_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_lead_container_get_blocked_actions(
        lead_container: *mut ClapperGtkLeadContainer,
    ) -> ClapperGtkVideoActionMask;
    pub fn clapper_gtk_lead_container_get_leading(
        lead_container: *mut ClapperGtkLeadContainer,
    ) -> gboolean;
    pub fn clapper_gtk_lead_container_set_blocked_actions(
        lead_container: *mut ClapperGtkLeadContainer,
        actions: ClapperGtkVideoActionMask,
    );
    pub fn clapper_gtk_lead_container_set_leading(
        lead_container: *mut ClapperGtkLeadContainer,
        leading: gboolean,
    );

    //=========================================================================
    // ClapperGtkNextItemButton
    //=========================================================================
    pub fn clapper_gtk_next_item_button_get_type() -> GType;
    pub fn clapper_gtk_next_item_button_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // ClapperGtkPreviousItemButton
    //=========================================================================
    pub fn clapper_gtk_previous_item_button_get_type() -> GType;
    pub fn clapper_gtk_previous_item_button_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // ClapperGtkSeekBar
    //=========================================================================
    pub fn clapper_gtk_seek_bar_get_type() -> GType;
    pub fn clapper_gtk_seek_bar_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_seek_bar_get_reveal_labels(seek_bar: *mut ClapperGtkSeekBar) -> gboolean;
    pub fn clapper_gtk_seek_bar_get_seek_method(
        seek_bar: *mut ClapperGtkSeekBar,
    ) -> clapper::ClapperPlayerSeekMethod;
    pub fn clapper_gtk_seek_bar_set_reveal_labels(
        seek_bar: *mut ClapperGtkSeekBar,
        reveal: gboolean,
    );
    pub fn clapper_gtk_seek_bar_set_seek_method(
        seek_bar: *mut ClapperGtkSeekBar,
        method: clapper::ClapperPlayerSeekMethod,
    );

    //=========================================================================
    // ClapperGtkSimpleControls
    //=========================================================================
    pub fn clapper_gtk_simple_controls_get_type() -> GType;
    pub fn clapper_gtk_simple_controls_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_simple_controls_get_extra_menu_button(
        controls: *mut ClapperGtkSimpleControls,
    ) -> *mut ClapperGtkExtraMenuButton;
    pub fn clapper_gtk_simple_controls_get_fullscreenable(
        controls: *mut ClapperGtkSimpleControls,
    ) -> gboolean;
    pub fn clapper_gtk_simple_controls_get_seek_method(
        controls: *mut ClapperGtkSimpleControls,
    ) -> clapper::ClapperPlayerSeekMethod;
    pub fn clapper_gtk_simple_controls_set_fullscreenable(
        controls: *mut ClapperGtkSimpleControls,
        fullscreenable: gboolean,
    );
    pub fn clapper_gtk_simple_controls_set_seek_method(
        controls: *mut ClapperGtkSimpleControls,
        method: clapper::ClapperPlayerSeekMethod,
    );

    //=========================================================================
    // ClapperGtkTitleHeader
    //=========================================================================
    pub fn clapper_gtk_title_header_get_type() -> GType;
    pub fn clapper_gtk_title_header_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_title_header_get_current_title(
        header: *mut ClapperGtkTitleHeader,
    ) -> *const c_char;
    pub fn clapper_gtk_title_header_get_fallback_to_uri(
        header: *mut ClapperGtkTitleHeader,
    ) -> gboolean;
    pub fn clapper_gtk_title_header_set_fallback_to_uri(
        header: *mut ClapperGtkTitleHeader,
        enabled: gboolean,
    );

    //=========================================================================
    // ClapperGtkTitleLabel
    //=========================================================================
    pub fn clapper_gtk_title_label_get_type() -> GType;
    pub fn clapper_gtk_title_label_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_title_label_get_current_title(
        label: *mut ClapperGtkTitleLabel,
    ) -> *const c_char;
    pub fn clapper_gtk_title_label_get_fallback_to_uri(
        label: *mut ClapperGtkTitleLabel,
    ) -> gboolean;
    pub fn clapper_gtk_title_label_get_media_item(
        label: *mut ClapperGtkTitleLabel,
    ) -> *mut clapper::ClapperMediaItem;
    pub fn clapper_gtk_title_label_set_fallback_to_uri(
        label: *mut ClapperGtkTitleLabel,
        enabled: gboolean,
    );
    pub fn clapper_gtk_title_label_set_media_item(
        label: *mut ClapperGtkTitleLabel,
        item: *mut clapper::ClapperMediaItem,
    );

    //=========================================================================
    // ClapperGtkToggleFullscreenButton
    //=========================================================================
    pub fn clapper_gtk_toggle_fullscreen_button_get_type() -> GType;
    pub fn clapper_gtk_toggle_fullscreen_button_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // ClapperGtkTogglePlayButton
    //=========================================================================
    pub fn clapper_gtk_toggle_play_button_get_type() -> GType;
    pub fn clapper_gtk_toggle_play_button_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // ClapperGtkVideo
    //=========================================================================
    pub fn clapper_gtk_video_get_type() -> GType;
    pub fn clapper_gtk_video_new() -> *mut gtk::GtkWidget;
    pub fn clapper_gtk_video_add_fading_overlay(
        video: *mut ClapperGtkVideo,
        widget: *mut gtk::GtkWidget,
    );
    pub fn clapper_gtk_video_add_overlay(video: *mut ClapperGtkVideo, widget: *mut gtk::GtkWidget);
    pub fn clapper_gtk_video_get_auto_inhibit(video: *mut ClapperGtkVideo) -> gboolean;
    pub fn clapper_gtk_video_get_fade_delay(video: *mut ClapperGtkVideo) -> c_uint;
    pub fn clapper_gtk_video_get_inhibited(video: *mut ClapperGtkVideo) -> gboolean;
    pub fn clapper_gtk_video_get_player(video: *mut ClapperGtkVideo)
        -> *mut clapper::ClapperPlayer;
    pub fn clapper_gtk_video_get_touch_fade_delay(video: *mut ClapperGtkVideo) -> c_uint;
    pub fn clapper_gtk_video_set_auto_inhibit(video: *mut ClapperGtkVideo, inhibit: gboolean);
    pub fn clapper_gtk_video_set_fade_delay(video: *mut ClapperGtkVideo, delay: c_uint);
    pub fn clapper_gtk_video_set_touch_fade_delay(video: *mut ClapperGtkVideo, delay: c_uint);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn clapper_gtk_get_player_from_ancestor(
        widget: *mut gtk::GtkWidget,
    ) -> *mut clapper::ClapperPlayer;
    pub fn clapper_gtk_get_resource() -> *mut gio::GResource;

}
