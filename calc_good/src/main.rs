extern crate gtk;
use gtk::prelude::*;
use gtk::{Label, Window, WindowType, Button, Grid, Inhibit};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Calculator");
    window.set_default_size(400, 500);

    let grid = Grid::new();
    grid.set_row_spacing(5);
    grid.set_column_spacing(5);

    let entry = Label::new(None);
    entry.set_label("0");
    grid.attach(&entry, 0, 0, 4, 1);

    let buttons = [
        "7", "8", "9", "/",
        "4", "5", "6", "*",
        "1", "2", "3", "-",
        "0", ".", "=", "+",
    ];

    let mut x = 0;
    let mut y = 1;
    for label in buttons.iter() {
        let button = Button::with_label(label);
        button.connect_clicked(clone!(@weak entry => move |_| {
            let text = entry.text().unwrap_or("0".to_string());
            let new_text = format!("{}{}", text, label);
            entry.set_label(&new_text);
        }));
        grid.attach(&button, x, y, 1, 1);

        x += 1;
        if x > 3 {
            x = 0;
            y += 1;
        }
    }

    window.add(&grid);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
