mod ex_application;

use ex_application::ExApplication;
use gtk::glib;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let app = ExApplication::new();
    app.run()
}
