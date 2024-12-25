use std::cell::RefCell;

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::gio;
use gtk::{Entry, ListView, TemplateChild, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/Todo/todo_window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<gio::ListStore>>,
}



#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
