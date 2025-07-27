use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box as GtkBox, Button, CheckButton, CssProvider, ListBox, ListBoxRow,
    Orientation, gdk,
};
use std::cell::Cell;
use std::rc::Rc;

use crate::app::core::{config::load_current_temp, presets::Preset, redshift::apply_temperature};

pub struct WindowBuilder<'a> {
    pub app: &'a gtk4::Application,
    pub presets: Vec<Preset>,
}

impl<'a> WindowBuilder<'a> {
    pub fn build(self) {
        // Main application window.
        let win = ApplicationWindow::builder()
            .application(self.app)
            .title("  T emperature")
            .resizable(false)
            .build();
        win.set_icon_name(Some("weather-clear-night-symbolic"));

        // Extra-compact CSS for all widgets and list rows.
        let css = r#"
            box#root { padding: 3px; }
            list, listbox, listboxrow, row {
                margin: 0; padding: 0; min-height: 0; min-width: 0;
            }
            checkbutton {
                margin: 0; padding: 1px 4px;
                min-height: 0; min-width: 0;
            }
            button { margin-top: 2px; padding: 2px 8px; }
            * { font-size: 13px; } /* Optionally, make font smaller */
        "#;
        let provider = CssProvider::new();
        provider.load_from_string(css);
        let display = gdk::Display::default().expect("No display");
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Root vertical container (minimal spacing).
        let root = GtkBox::new(Orientation::Vertical, 2);
        root.set_widget_name("root");

        // ListBox for radio rows.
        let list = ListBox::new();
        list.set_selection_mode(gtk4::SelectionMode::Single);
        list.set_activate_on_single_click(true);
        // This will ensure there is never a persistent row selection highlight:
        list.connect_selected_rows_changed(|list| {
            if let Some(row) = list.selected_row() {
                list.unselect_row(&row);
            }
        });

        // Shared state between handlers.
        let current = Rc::new(Cell::new(load_current_temp())); // Option<u32>
        let suppress = Rc::new(Cell::new(true));
        let mut head: Option<CheckButton> = None;
        let mut items: Vec<(u32, CheckButton, ListBoxRow)> = Vec::with_capacity(self.presets.len());

        for p in &self.presets {
            let btn = CheckButton::with_label(&p.label);
            if let Some(ref h) = head {
                btn.set_group(Some(h));
            } else {
                head = Some(btn.clone());
            }
            btn.set_halign(gtk4::Align::Fill);
            btn.set_hexpand(true);

            // Only CheckButton; avoid hbox for ultra-compactness.
            let row = ListBoxRow::new();
            row.set_child(Some(&btn));
            list.append(&row);

            items.push((p.value, btn, row));
        }

        // Set start selection, but suppress events.
        suppress.set(true);
        if let Some(cur) = current.get() {
            if let Some((_, btn, _)) = items.iter().find(|(v, _, _)| *v == cur) {
                btn.set_active(true);
            }
        }
        suppress.set(false);

        // Connect handlers for radio toggled.
        for (val, btn, _) in &items {
            let cur = Rc::clone(&current);
            let sup = Rc::clone(&suppress);
            let v = *val;
            btn.connect_toggled(move |b| {
                if sup.get() || !b.is_active() {
                    return;
                }
                if cur.get() == Some(v) {
                    return;
                }
                apply_temperature(v);
                cur.set(Some(v));
            });
        }

        // Click on row (not just on radio).
        let items_by_index: Vec<CheckButton> = items.iter().map(|(_, b, _)| b.clone()).collect();
        list.connect_row_activated(move |_lb, row| {
            let idx = row.index();
            if idx >= 0 {
                if let Some(btn) = items_by_index.get(idx as usize) {
                    btn.set_active(true); // triggers toggled handler
                }
            }
        });

        // "Close" button.
        let close = Button::with_label("Close");
        {
            let w = win.clone();
            close.connect_clicked(move |_| w.close());
        }

        // Layout, no scroll windows.
        root.append(&list);
        root.append(&close);
        win.set_child(Some(&root));

        // No explicit set_default_size: let GTK choose, since all margins/paddings are minimized.

        win.present();
    }
}
