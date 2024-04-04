use gio::ActionEntry;
use glib::clone;
use gtk::gio::SimpleActionGroup;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.Actions1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.set_accels_for_action("custom-group.close", &["<Ctrl>W"]);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(360)
        .build();

    let action_close = ActionEntry::builder("close")
        .activate(clone!(@weak window => move |_, _, _| {
            window.close();
        }))
        .build();

    let actions = SimpleActionGroup::new();
    actions.add_action_entries([action_close]);
    window.insert_action_group("custom-group", Some(&actions));

    window.present();
}
