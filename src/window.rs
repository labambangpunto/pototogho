use std::cell::OnceCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::hash_checker::{compare_hashes, HashComparison};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PototoghoWindow {
        pub first_entry: OnceCell<adw::EntryRow>,
        pub second_entry: OnceCell<adw::EntryRow>,
        pub result_icon: OnceCell<gtk::Image>,
        pub result_label: OnceCell<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PototoghoWindow {
        const NAME: &'static str = "PototoghoWindow";
        type Type = super::PototoghoWindow;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for PototoghoWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.set_title(Some("Pototogho"));
            obj.set_default_size(520, 380);

            // 1. Popover menu model (Keyboard Shortcuts & About)
            let menu = gio::Menu::new();

            let help_section = gio::Menu::new();
            help_section.append(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
            menu.append_section(None, &help_section);

            let about_section = gio::Menu::new();
            about_section.append(Some("About Pototogho"), Some("app.about"));
            menu.append_section(None, &about_section);

            // 2. Hamburger menu button next to the close button
            let menu_button = gtk::MenuButton::builder()
                .icon_name("open-menu-symbolic")
                .tooltip_text("Main Menu")
                .primary(true)
                .menu_model(&menu)
                .build();

            let header_bar = adw::HeaderBar::new();
            header_bar.pack_end(&menu_button);

            // 3. Two hash input fields
            let first_entry = adw::EntryRow::builder()
                .title("First Hash")
                .css_classes(["monospace"])
                .build();

            let second_entry = adw::EntryRow::builder()
                .title("Second Hash")
                .css_classes(["monospace"])
                .build();

            let input_group = adw::PreferencesGroup::builder()
                .title("Hash Comparator")
                .description("Enter two hash values (MD5, SHA-1, SHA-256, etc.) to compare.")
                .build();
            input_group.add(&first_entry);
            input_group.add(&second_entry);

            // 4. "Compare" button
            let compare_button = gtk::Button::builder()
                .label("Compare")
                .halign(gtk::Align::Center)
                .css_classes(["suggested-action", "pill"])
                .build();

            // 5. Result icon and status label below the button
            let result_icon = gtk::Image::builder()
                .pixel_size(48)
                .visible(false)
                .build();

            let result_label = gtk::Label::builder()
                .css_classes(["heading"])
                .visible(false)
                .build();

            let result_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(8)
                .halign(gtk::Align::Center)
                .margin_top(12)
                .build();
            result_box.append(&result_icon);
            result_box.append(&result_label);

            // Trigger comparison on button click or Enter key press
            let window_weak = obj.downgrade();
            compare_button.connect_clicked(move |_| {
                if let Some(window) = window_weak.upgrade() {
                    window.perform_comparison();
                }
            });

            let window_weak_first = obj.downgrade();
            first_entry.connect_entry_activated(move |_| {
                if let Some(window) = window_weak_first.upgrade() {
                    window.perform_comparison();
                }
            });

            let window_weak_second = obj.downgrade();
            second_entry.connect_entry_activated(move |_| {
                if let Some(window) = window_weak_second.upgrade() {
                    window.perform_comparison();
                }
            });

            // Store widget references in imp struct
            self.first_entry.set(first_entry).unwrap();
            self.second_entry.set(second_entry).unwrap();
            self.result_icon.set(result_icon).unwrap();
            self.result_label.set(result_label).unwrap();

            // 6. Arrange layout inside AdwClamp
            let content_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(20)
                .valign(gtk::Align::Center)
                .build();
            content_box.append(&input_group);
            content_box.append(&compare_button);
            content_box.append(&result_box);

            let clamp = adw::Clamp::builder()
                .maximum_size(480)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(16)
                .margin_end(16)
                .child(&content_box)
                .build();

            let toolbar_view = adw::ToolbarView::new();
            toolbar_view.add_top_bar(&header_bar);
            toolbar_view.set_content(Some(&clamp));

            obj.set_content(Some(&toolbar_view));
        }
    }

    impl WidgetImpl for PototoghoWindow {}
    impl WindowImpl for PototoghoWindow {}
    impl ApplicationWindowImpl for PototoghoWindow {}
    impl AdwApplicationWindowImpl for PototoghoWindow {}
}

glib::wrapper! {
    pub struct PototoghoWindow(ObjectSubclass<imp::PototoghoWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl PototoghoWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn perform_comparison(&self) {
        let imp = self.imp();
        let first_text = imp.first_entry.get().unwrap().text();
        let second_text = imp.second_entry.get().unwrap().text();
        let icon = imp.result_icon.get().unwrap();
        let label = imp.result_label.get().unwrap();

        match compare_hashes(&first_text, &second_text) {
            HashComparison::Match => {
                icon.set_icon_name(Some("emblem-ok-symbolic"));
                icon.set_css_classes(&["success"]);
                icon.set_visible(true);

                label.set_label("Hashes match");
                label.set_css_classes(&["heading", "success"]);
                label.set_visible(true);
            }
            HashComparison::Mismatch => {
                icon.set_icon_name(Some("window-close-symbolic"));
                icon.set_css_classes(&["error"]);
                icon.set_visible(true);

                label.set_label("Hashes do not match");
                label.set_css_classes(&["heading", "error"]);
                label.set_visible(true);
            }
            HashComparison::Empty => {
                icon.set_visible(false);

                label.set_label("Enter both hash values first");
                label.set_css_classes(&["dim-label"]);
                label.set_visible(true);
            }
        }
    }
}
