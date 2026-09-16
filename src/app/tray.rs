use ksni::menu::{RadioGroup, RadioItem};

use crate::app::core::{config::load_current_temp, presets::Preset, redshift::apply_temperature};

#[derive(Debug)]
pub struct NightLightTray {
    presets: Vec<Preset>,
    selected: Option<usize>,
}

impl NightLightTray {
    pub fn new(presets: Vec<Preset>) -> Self {
        let selected = selected_index(&presets);
        Self { presets, selected }
    }

    fn sync_selection(&mut self) {
        self.selected = selected_index(&self.presets);
    }
}

fn selected_index(presets: &[Preset]) -> Option<usize> {
    load_current_temp().and_then(|current| {
        presets
            .iter()
            .position(|preset| preset.value == current)
    })
}

impl ksni::Tray for NightLightTray {
    const MENU_ON_ACTIVATE: bool = true;

    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn title(&self) -> String {
        "Night Light".into()
    }

    fn icon_name(&self) -> String {
        "weather-clear-night-symbolic".into()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        let options = self
            .presets
            .iter()
            .map(|preset| RadioItem {
                label: preset.label.clone(),
                ..Default::default()
            })
            .collect();

        vec![
            RadioGroup {
                selected: self.selected.unwrap_or(self.presets.len()),
                options,
                select: Box::new(|tray: &mut Self, index| {
                    let Some(value) = tray.presets.get(index).map(|preset| preset.value) else {
                        return;
                    };

                    match apply_temperature(value) {
                        Ok(()) => tray.selected = Some(index),
                        Err(error) => eprintln!("failed to apply temperature {value}: {error}"),
                    }
                }),
            }
            .into(),
        ]
    }

    fn menu_about_to_show(&mut self) {
        self.sync_selection();
    }
}
