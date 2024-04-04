use gio::ActionEntry;
use glib::clone;
use gtk::gio::SimpleActionGroup;
use gtk::{gio, glib, Application, ApplicationWindow, Button, Label};
use gtk::{prelude::*, Align, Orientation};

const APP_ID: &str = "org.gtk_rs.Actions1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.set_accels_for_action("custom-group.close", &["<Ctrl>W"]);
    app.run()
}

fn build_ui(app: &Application) {
    let original_state = 0;
    let label = Label::builder()
        .label(format!("Counter: {original_state}"))
        .build();

    let button = Button::builder().label("Press me!").build();
    button.connect_clicked(move |button| {
        let parameter = 1;
        button
            .activate_action("win.count", Some(&parameter.to_variant()))
            .expect("The action does not exist");
    });

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(360)
        .child(&gtk_box)
        .build();

    let action_count = ActionEntry::builder("count")
        .parameter_type(Some(&i32::static_variant_type()))
        .state(original_state.to_variant())
        .activate(move |_, action, parameter| {
            let mut state = action
                .state()
                .expect("Could not get state.")
                .get::<i32>()
                .expect("The variant needs to be of type `i32`.");

            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The variant needs to be of type `i32`.");

            state += parameter;
            action.set_state(&state.to_variant());

            label.set_label(&format!("Counter: {state}"));
        })
        .build();

    window.add_action_entries([action_count]);

    window.present();
}
