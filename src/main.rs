mod custom_button;
mod todo_window;
mod task_list;
mod task_row;


use gtk::prelude::*; 
use gtk::{glib, Application};
use gtk::gio; 
use todo_window::Window;


const APP_ID: &str = "org.gtk_rs.Todo";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("todo.gresource")
        .expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run() 
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
