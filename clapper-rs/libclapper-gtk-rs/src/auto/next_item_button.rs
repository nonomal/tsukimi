// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from ..\gir-files-gstreamer
// from ..\gir-files-gtk
// from ..\libclapper-rs
// DO NOT EDIT

// use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "ClapperGtkNextItemButton")]
    pub struct NextItemButton(Object<ffi::ClapperGtkNextItemButton, ffi::ClapperGtkNextItemButtonClass>) @extends gtk::Button, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::clapper_gtk_next_item_button_get_type(),
    }
}

impl NextItemButton {
    #[doc(alias = "clapper_gtk_next_item_button_new")]
    pub fn new() -> NextItemButton {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::clapper_gtk_next_item_button_new()).unsafe_cast()
        }
    }
}

impl Default for NextItemButton {
    fn default() -> Self {
        Self::new()
    }
}
