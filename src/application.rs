use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::PototoghoWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PototoghoApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for PototoghoApplication {
        const NAME: &'static str = "PototoghoApplication";
        type Type = super::PototoghoApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for PototoghoApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.shortcuts", &["<primary>question"]);
        }
    }

    impl ApplicationImpl for PototoghoApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = application
                .active_window()
                .unwrap_or_else(|| PototoghoWindow::new(&*application).upcast());

            window.present();
        }
    }

    impl GtkApplicationImpl for PototoghoApplication {}
    impl AdwApplicationImpl for PototoghoApplication {}
}

glib::wrapper! {
    pub struct PototoghoApplication(ObjectSubclass<imp::PototoghoApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PototoghoApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let shortcuts_action = gio::ActionEntry::builder("shortcuts")
            .activate(move |app: &Self, _, _| app.show_shortcuts())
            .build();

        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        self.add_action_entries([quit_action, shortcuts_action, about_action]);
    }

    fn show_shortcuts(&self) {
        let window = self.active_window().unwrap();

        let group = adw::PreferencesGroup::builder()
            .title("General")
            .build();

        let shortcuts = [
            ("Compare Hashes", "Return"),
            ("Keyboard Shortcuts", "<primary>question"),
            ("Quit", "<primary>q"),
        ];

        for (title, accel) in shortcuts {
            let shortcut_label = gtk::ShortcutLabel::new(accel);
            shortcut_label.set_valign(gtk::Align::Center);

            let row = adw::ActionRow::builder()
                .title(title)
                .build();
            row.add_suffix(&shortcut_label);
            group.add(&row);
        }

        let page = adw::PreferencesPage::new();
        page.add(&group);

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&adw::HeaderBar::new());
        toolbar_view.set_content(Some(&page));

        let dialog = adw::Dialog::builder()
            .title("Keyboard Shortcuts")
            .content_width(420)
            .content_height(300)
            .child(&toolbar_view)
            .build();

        dialog.present(Some(&window));
    }

fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Pototogho")
            .application_icon("dev.labambangpunto.pototogho")
            .developer_name("labambangpunto")
            .version(VERSION)
            .copyright("© 2026 labambangpunto")
            .license_type(gtk::License::Gpl30)
            .build();

        about.add_legal_section(
            "Application Icon",
            Some("© 2026 labambangpunto"),
            gtk::License::Custom,
            Some(
                "Licensed under the <a href=\"https://creativecommons.org/licenses/by-nc-sa/4.0/\">Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International (CC BY-NC-SA 4.0)</a> license.",
            ),
        );

        about.present(Some(&window));
    }
}
