mod task_object;
mod task_row;
mod utils;
mod window;

use adw::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};
use gtk::gdk::Display;
use gtk::gio::prelude::{ActionMapExtManual, ApplicationExt, ApplicationExtManual};
use gtk::gio::{ActionEntry, SimpleActionGroup};
use gtk::glib::clone;
use gtk::{CssProvider, gio, glib};

use crate::window::Window;

const APP_ID: &str = "org.gtk_rs.Todo1";

// ANCHOR: main
fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("gresources").expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css()
    });
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

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk_rs/Todo1/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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
