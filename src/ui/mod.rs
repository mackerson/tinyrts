mod hud;

use bevy::prelude::*;

use crate::resources::CameraMode;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hud::setup_ui)
            .add_systems(Update, (
                hud::update_unit_counts,
                hud::update_mode_indicator,
                hud::update_controls_hint,
            ))
            // Show/hide panels based on camera mode
            .add_systems(OnEnter(CameraMode::Rts), hud::show_rts_panels)
            .add_systems(OnEnter(CameraMode::Possession), hud::hide_rts_panels);
    }
}
