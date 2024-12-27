mod imp;
use gtk::prelude::{ActionMapExt, Cast, CastNone, EntryBufferExtManual, EntryExt, ListItemExt, ListModelExt, SettingsExt, SettingsExtManual};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, Application, CustomFilter, FilterListModel, ListItem, NoSelection, SignalListItemFactory};
use glib::Object; 
use glib::clone;
use gtk::gio::{self, Settings};

use crate::task_list::{TaskData, TaskObject};
use crate::task_row::TaskRow;
use crate::utils::data_path;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn tasks(&self) -> gio::ListStore {
        self.imp()
        .tasks
        .borrow()
        .clone()
        .expect("Could not get current tasks.")
    }

    fn filter(&self) -> Option<CustomFilter> {
        let filter_state: String = self.settings().get("filter");

        let filter_open = CustomFilter::new(|obj| {
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");
            !task_object.is_completed()
        });

        let filter_done = CustomFilter::new(|obj| {
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            task_object.is_completed()
        });

        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        } 

    }


    fn setup_tasks(&self) {
        let model = gio::ListStore::new::<TaskObject>();

        self.imp().tasks.replace(Some(model));
        
        let filter_model = FilterListModel::new(Some(self.tasks()), self.filter());
        let selection_model = NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.set_model(Some(&selection_model));

        self.settings().connect_changed(
            Some("filter"), 
            clone!(
            #[weak(rename_to = window)]
            self,
            #[weak]
            filter_model, 
            move |_,_| {
                    filter_model.set_filter(window.filter().as_ref());
                }
            ),
        );
    }

    fn new_task(&self) {
        let buffer = self.imp().entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }

    fn setup_callbacks(&self) {
        self.imp().entry.connect_activate(clone!(
            #[weak(rename_to = window)]
            self, 
            move|_| {
                window.new_task();
            }));
        self.imp().entry.connect_icon_release(clone!(
        #[weak(rename_to = window)]
        self,
        move |_,_| {
                window.new_task();
            }));
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new(); 

        factory.connect_setup(move |_, list_item | {
            let task_row = TaskRow::new();
            list_item.downcast_ref::<ListItem>()
                .expect("Need to be ListItem")
                .set_child(Some(&task_row));
        });

        factory.connect_bind(move |_,list_item| {
            let task_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<TaskObject>()
                .expect("The item has to be an `TaskObject`.");

            let task_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.bind(&task_object);
        });

        factory.connect_unbind(move |_, list_item| {
            let task_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs tob ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");
            task_row.unbind();
        });

        self.imp().tasks_list.set_factory(Some(&factory));
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    fn setup_actions(&self) {
        let action_filter = self.settings().create_action("filter");
        self.add_action(&action_filter);
    }

    fn remove_done_tasks(&self) {
        let tasks = self.tasks();
        let mut position = 0; 
        while let Some(item) = tasks.item(position) {
            let task_object = item
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            if task_object.is_completed() {
                tasks.remove(position)
            } else {
                position += 1;
            }
        }
    }


    fn restore_data(&self) {
        if let Ok(file) = std::fs::File::open(data_path()) {
            let backup_data:Vec<TaskData> = serde_json::from_reader(file).expect(
                "It Should be possible to read `backup_data` from the json file.",);
            
            let task_objects: Vec<TaskObject> = backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();
            self.tasks().extend_from_slice(&task_objects);
        }
    }
}



