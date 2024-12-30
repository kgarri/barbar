use gtk::glib::subclass::InitializingObject;
use gtk::{glib, Button, CompositeTemplate, Label};
use gtk::subclass::prelude::*;

use crate::start_todo;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/bar/bar.ui")]
pub struct Bar {
    #[template_child]
    pub label: TemplateChild<Label>,
    #[template_child]
    pub todo_button: TemplateChild<Button>,
}


#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str ="BarBar";
    type Type = super::Bar;
    type ParentType = gtk::ApplicationWindow;


    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Bar {
    #[template_callback]
    fn todo_clicked(_button: &Button) {
        start_todo();
    }
}

impl ObjectImpl for Bar {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_bar();
    }
}

impl WidgetImpl for Bar {}

impl WindowImpl for Bar {
}

impl ApplicationWindowImpl for Bar {}

