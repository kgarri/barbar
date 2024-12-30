use std::cell::{OnceCell, RefCell};
use std::fs::File;

use gtk::prelude::ListModelExtManual;
use gtk::{glib};
use gtk::subclass::prelude::*;
use gtk::gio::{self, Settings};
use gtk::{Entry, ListView, TemplateChild, CompositeTemplate};

use crate::task_list::{TaskObject, TaskData};
use crate::utils::data_path;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/bar/todo_window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<gio::ListStore>>,
    pub settings: OnceCell<Settings>,
}



#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();

        klass.install_action("win.remove-done-tasks", None , |window, _, _| {
            window.remove_done_tasks();
        });
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }

}


impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_settings();
        obj.setup_actions();
        obj.setup_tasks(); 
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

impl WidgetImpl for Window {
}

impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        let backup_data: Vec<TaskData> = self.obj()
            .tasks()
            .iter::<TaskObject>()
            .filter_map(Result::ok)
            .map(|task_object| task_object.task_data())
            .collect(); 

        let file = File::create(data_path("org.gtk_rs.Todo")).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data).expect("Could not write data to json file"); 

        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for Window {}
