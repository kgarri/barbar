mod custom_button;

use gtk::prelude::*; 
use gtk::{glib, Application, ApplicationWindow, Switch, Box};
use glib::closure_local;
use custom_button::CustomButton;


const APP_ID: &str = "org.gtk_rs.HelloWorld1";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run() 
}

fn build_ui(app: &Application) {
    let gtk_box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let switch_1 = Switch::new();
    let switch_2 = Switch::new();

    switch_1.set_margin_start(200);
    switch_1.set_margin_end(200);

    switch_2.set_margin_bottom(100); 
    switch_2.set_margin_start(200);
    switch_2.set_margin_end(200);

    switch_1
        .bind_property("active", &switch_2, "active")
        .bidirectional()
        .build(); 

    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();

    let button_3 = CustomButton::new();

    button_1
        .bind_property("number", &button_2, "number")
        .transform_to(|_, number: i32|{
                let incremeneted_number = number + 1;
                Some(incremeneted_number.to_value())
        })
        .transform_from(|_, number: i32 | {
        let decremented_number = number - 1;
        Some(decremented_number.to_value())
        })
        .bidirectional()
        .sync_create()
        .build();

    button_1.set_margin_top(100);
    button_1.set_margin_start(100);
    button_1.set_margin_end(100);

    button_1.connect_closure(
        "max-number-reached", 
        false, 
        closure_local!(move |_button: CustomButton, number: i32 | {
            println!("The maxium number {} has been reached!", number);
        }), 
    );
    
    button_2.set_margin_start(100);
    button_2.set_margin_end(100);
    

    button_3.set_margin_top(100);
    button_3.set_margin_start(100);
    button_3.set_margin_end(100);


    gtk_box.append(&button_1);
    gtk_box.append(&button_2);
    gtk_box.append(&switch_1);
    gtk_box.append(&switch_2);
    gtk_box.append(&button_3);
    
    
    let window=ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();
}
