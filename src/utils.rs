use std::path::PathBuf;


use gtk::glib;


pub fn data_path(app_id: &str) -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(app_id);
    std::fs::create_dir_all(&path).expect("Could not create directory.");
    path.push("data.json");
    path
}
