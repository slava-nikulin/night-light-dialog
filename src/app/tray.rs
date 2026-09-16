use ksni::menu::{RadioGroup, RadioItem};

use crate::app::core::{
    config::load_current_temp,
    presets::Preset,
    redshift::apply_temperature,
};

#[derive(Debug)]
pub struct NightLightTray {
    presets: Vec<Preset>,
    selected: usize,
}

impl NightLightTray {
    pub fn new(presets: Vec<Preset>) -> Self {
        let selected = load_current_temp()
            .and_then(|current| presets.iter().position(|preset| preset.value == current))
            .unwrap_or(0);

        Self { presets, selected }
    }

    fn sync_selection(&mut self) {
        if let Some(current) = load_current_temp()
            && let Some(index) = self
                .presets
                .iter()
                .position(|preset| preset.value == current)
        {
            self.selected = index;
        }
    }
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

        vec![RadioGroup {
            selected: self.selected,
            options,
            select: Box::new(|tray: &mut Self, index| {
                let Some(preset) = tray.presets.get(index) else {
                    return;
                };

                apply_temperature(preset.value);
                tray.selected = index;
            }),
        }
        .into()]
    }

    fn menu_about_to_show(&mut self) {
        self.sync_selection();
    }
}
