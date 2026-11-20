mod application;
mod config;
mod hash_checker;
mod window;

use self::application::PototoghoApplication;
use self::window::PototoghoWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Gagal mengatur bindtextdomain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Gagal mengatur encoding UTF-8");
    textdomain(GETTEXT_PACKAGE).expect("Gagal mengatur textdomain");

    let app = PototoghoApplication::new(
        "dev.labambangpunto.pototogho",
        &gio::ApplicationFlags::empty(),
    );

    app.run()
}
