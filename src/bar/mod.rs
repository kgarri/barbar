mod imp;

use gtk::gdk::Monitor;
use gtk::glib::Object;
use gtk::prelude::{CastNone, DisplayExt, GtkWindowExt, ListModelExtManual, WidgetExt};
use gtk::{gio, glib, Align, Application };
use gtk4_layer_shell::{LayerShell,Layer,Edge};

glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root;
}



impl Bar {
    pub fn new(app: &Application) -> Self {
        Object::builder()
            .property("application", app).build()
    }

    pub fn setup_bar(&self) {
        LayerShell::init_layer_shell(self);
        LayerShell::set_layer(self, Layer::Top);
        LayerShell::auto_exclusive_zone_enable(self);
        LayerShell::set_anchor(self, Edge::Right, true);
        LayerShell::set_anchor(self, Edge::Left, true);
        LayerShell::set_anchor(self, Edge::Top, true);
        self.set_valign(Align::Start);
        self.set_halign(Align::Fill);
        self.set_fullscreened(true);

    }
}
