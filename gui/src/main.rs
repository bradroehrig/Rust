extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create a new top-level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rust GUI Example");
    window.set_default_size(400, 300);

    // Create a button
    let button = Button::new_with_label("Click me!");

    // Connect the "clicked" signal of the button to a closure
    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    // Add the button to the window
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&button, true, true, 0);
    window.add(&vbox);

    // Handle window close event
    window.connect_delete_event(|_, _| {
        // Terminate the GTK main loop
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all the widgets
    window.show_all();

    // Start the GTK main loop
    gtk::main();
}
