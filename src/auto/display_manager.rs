// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use Display;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DisplayManager(Object<ffi::GdkDisplayManager>);

    match fn {
        get_type => || ffi::gdk_display_manager_get_type(),
    }
}

impl DisplayManager {
    pub fn get() -> DisplayManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gdk_display_manager_get())
        }
    }
}

pub trait DisplayManagerExt {
    fn get_default_display(&self) -> Option<Display>;

    fn list_displays(&self) -> Vec<Display>;

    fn open_display(&self, name: &str) -> Option<Display>;

    fn set_default_display(&self, display: &Display);

    fn get_property_default_display(&self) -> Option<Display>;

    fn set_property_default_display(&self, default_display: Option<&Display>);

    fn connect_display_opened<F: Fn(&Self, &Display) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DisplayManager> + IsA<glib::object::Object>> DisplayManagerExt for O {
    fn get_default_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_display_manager_get_default_display(self.to_glib_none().0))
        }
    }

    fn list_displays(&self) -> Vec<Display> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_display_manager_list_displays(self.to_glib_none().0))
        }
    }

    fn open_display(&self, name: &str) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_display_manager_open_display(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn set_default_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_display_manager_set_default_display(self.to_glib_none().0, display.to_glib_none().0);
        }
    }

    fn get_property_default_display(&self) -> Option<Display> {
        let mut value = Value::from(None::<&Display>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "default-display".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_default_display(&self, default_display: Option<&Display>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "default-display".to_glib_none().0, Value::from(default_display).to_glib_none().0);
        }
    }

    fn connect_display_opened<F: Fn(&Self, &Display) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Display) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "display-opened",
                transmute(display_opened_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_default_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::default-display",
                transmute(notify_default_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn display_opened_trampoline<P>(this: *mut ffi::GdkDisplayManager, display: *mut ffi::GdkDisplay, f: glib_ffi::gpointer)
where P: IsA<DisplayManager> {
    callback_guard!();
    let f: &&(Fn(&P, &Display) + 'static) = transmute(f);
    f(&DisplayManager::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(display))
}

unsafe extern "C" fn notify_default_display_trampoline<P>(this: *mut ffi::GdkDisplayManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DisplayManager> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DisplayManager::from_glib_borrow(this).downcast_unchecked())
}
