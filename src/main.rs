mod custom_button;
mod todo_window;
mod task_list;
mod task_row;
mod utils;
mod bar;


use gtk::gdk::Monitor;
use gtk::prelude::*; 
use gtk::{glib, Application};
use gtk::gio; 
use gtk4_layer_shell::LayerShell;
use todo_window::Window;

use self::bar::Bar;


const APP_ID: &str = "org.gtk_rs.Todo";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("todo.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run() 
}

fn build_ui(app: &Application) {
        let display = gtk::gdk::Display::default().expect("Could not find a Display."); 
        let monitors = display.monitors();
        for monitor in &monitors {
            let item = monitor.ok().and_downcast::<Monitor>().expect("Item needs to be Monitor");
            let window = Bar::new(app);
            window.set_monitor(&item);
            window.present();
        }
} 

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}
