extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Inhibit, Label, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rust Calculator");
    window.set_default_size(300, 400);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let display = Label::new(None);
    vbox.pack_start(&display, true, true, 0);

    let button_grid = gtk::Grid::new();
    for (i, &label) in ["7", "8", "9", "/", "4", "5", "6", "*", "1", "2", "3", "-", "0", ".", "=", "+"]
        .iter()
        .enumerate()
    {
        let button = Button::with_label(label);
        button.connect_clicked(move |_| {
            let text = display.get_text().to_string();
            display.set_text(&format!("{}{}", text, label));
        });

        let col = i % 4;
        let row = i / 4;
        button_grid.attach(&button, col as i32, row as i32, 1, 1);
    }
    vbox.pack_start(&button_grid, true, true, 0);

    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        // Stop the GTK main loop
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
