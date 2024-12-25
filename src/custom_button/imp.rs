use std::cell::Cell;
use std::sync::OnceLock;

use glib::Properties;
use glib::subclass::Signal;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;


#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get,set)]
    number: Cell<i32>,
}


#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS : OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![Signal::builder("max-number-reached")
            .param_types([i32::static_type()])
            .build()]
        })
    }
    fn constructed(&self) {
        self.parent_constructed();
        
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .sync_create()
            .build();
    }
}

impl WidgetImpl for CustomButton {}


static MAX_NUMBER: i32 = 8;

impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let obj = self.obj();
        let incremented_number = obj.number() + 1;

        if incremented_number == MAX_NUMBER {
            obj.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            obj.set_number(0);
        } else {
        self.obj().set_number(incremented_number);
        }
    }
}
