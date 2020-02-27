extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label, Orientation};
use std::env;

fn main() {
    // Application need an ID
    let app = Application::new("com.shinglyu.catsay-gui", gio::ApplicationFlags::empty())
        .expect("Failed to initialize GTK.");

    // Callback when the app starts
    app.connect_startup(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);

        window.connect_delete_event(|win, _| {
            win.destroy();
            Inhibit(false)
        });
        let layout_box = Box::new(Orientation::Vertical, 0);

        let label = Label::new("Meow!\n     \\\n      \\");
        //label.set_xalign(0.0);
        layout_box.add(&label);

        let cat_image = Image::new_from_file("./images/cat.png");
        layout_box.add(&cat_image);
        window.add(&layout_box);

        window.show_all();
    });

    // Callback when the app activates, do nothing
    app.connect_activate(|_| {});
    // Run the app
    app.run(&env::args().collect::<Vec<_>>());
}
