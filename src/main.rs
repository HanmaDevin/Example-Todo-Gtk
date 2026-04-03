mod collection_object;
mod config;
mod task_object;
mod task_row;
mod utils;
mod window;

use adw::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};
use gtk::gio::prelude::{ActionMapExtManual, ApplicationExt, ApplicationExtManual};
use gtk::gio::{ActionEntry, SimpleActionGroup};
use gtk::glib::clone;
use gtk::{gio, glib};

use crate::config::app_id;
use crate::window::Window;

// ANCHOR: main
fn main() -> glib::ExitCode {
    // Load resources from installed location
    let res = gio::Resource::load(config::resources_file()).expect("Could not load gresource file");
    gio::resources_register(&res);

    // Create a new application
    let app = adw::Application::builder().application_id(app_id()).build();

    app.connect_startup(setup_shortcuts);
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and present it
    let window = Window::new(app);

    // Add action "close" to `window` taking no parameter
    let action_close = ActionEntry::builder("close")
        .activate(clone!(
            #[weak]
            window,
            move |_, _, _| {
                window.close();
            }
        ))
        .build();

    // Create a new action group and add actions to it
    let actions = SimpleActionGroup::new();
    actions.add_action_entries([action_close]);
    window.insert_action_group("custom-group", Some(&actions));

    window.present();
}
