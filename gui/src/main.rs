use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    // Initialize GTK
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Create a new window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rust GUI Example");
    window.set_default_size(350, 70);

    // Create a new button
    let button = Button::new_with_label("Click me!");

    // Add the button to the window
    window.add(&button);

    // Connect the button's "clicked" signal to a callback function
    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    // Show the window and all its content
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}
