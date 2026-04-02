use bevy::prelude::*;

use crate::components::*;
use crate::resources::CameraMode;

// --- Marker components for UI nodes ---

#[derive(Component)]
pub struct TopBar;

#[derive(Component)]
pub struct ModeIndicator;

#[derive(Component)]
pub struct UnitCountText;

#[derive(Component)]
pub struct ControlsHint;

/// Panels only shown in RTS mode (future: build panel, minimap)
#[derive(Component)]
pub struct RtsPanel;

#[derive(Component)]
pub struct BottomBar;

// --- Colors ---

const BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const RED_COLOR: Color = Color::srgb(1.0, 0.3, 0.25);
const BLUE_COLOR: Color = Color::srgb(0.3, 0.5, 1.0);
const GOLD_COLOR: Color = Color::srgb(1.0, 0.85, 0.3);

pub fn setup_ui(mut commands: Commands) {
    // --- Top bar ---
    commands
        .spawn((
            TopBar,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                height: Val::Px(36.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BG_COLOR.into()),
        ))
        .with_children(|parent| {
            // Mode indicator (left)
            parent.spawn((
                ModeIndicator,
                Text::new("RTS"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(GOLD_COLOR.into()),
            ));

            // Unit counts (center)
            parent.spawn((
                UnitCountText,
                Text::new("Red: 0  |  Blue: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(TEXT_COLOR.into()),
            ));

            // Placeholder for future elements (right)
            parent.spawn((
                Node::default(),
            ));
        });

    // --- Bottom bar (controls hint + future build panel slot) ---
    commands
        .spawn((
            BottomBar,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                height: Val::Px(32.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BG_COLOR.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                ControlsHint,
                Text::new("[Click] Select  |  [F] Possess  |  [Tab] Unpossess  |  [WASD] Move  |  [Scroll] Zoom"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(TEXT_COLOR.into()),
            ));
        });

    // --- Right panel placeholder (future: minimap) ---
    // Left panel placeholder (future: build queue)
    // These will be added as RtsPanel-tagged nodes later
}

pub fn update_unit_counts(
    mut text_query: Query<&mut Text, With<UnitCountText>>,
    units: Query<&Team, With<Unit>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let mut red = 0u32;
    let mut blue = 0u32;
    for team in &units {
        match team {
            Team::Red => red += 1,
            Team::Blue => blue += 1,
        }
    }

    **text = format!("Red: {}  |  Blue: {}", red, blue);
}

pub fn update_mode_indicator(
    camera_mode: Res<State<CameraMode>>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<ModeIndicator>>,
) {
    let Ok((mut text, mut color)) = text_query.single_mut() else {
        return;
    };

    match camera_mode.get() {
        CameraMode::Rts => {
            **text = "RTS VIEW".into();
            color.0 = GOLD_COLOR.into();
        }
        CameraMode::Possession => {
            **text = "POSSESSED".into();
            color.0 = RED_COLOR.into();
        }
    }
}

pub fn update_controls_hint(
    camera_mode: Res<State<CameraMode>>,
    mut text_query: Query<&mut Text, With<ControlsHint>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    match camera_mode.get() {
        CameraMode::Rts => {
            **text = "[Click] Select  |  [F] Possess  |  [WASD] Pan  |  [Scroll] Zoom".into();
        }
        CameraMode::Possession => {
            **text = "[WASD] Move  |  [Mouse] Aim  |  [Click] Shoot  |  [Tab] Unpossess".into();
        }
    }
}

pub fn show_rts_panels(mut panels: Query<&mut Visibility, With<RtsPanel>>) {
    for mut vis in &mut panels {
        *vis = Visibility::Inherited;
    }
}

pub fn hide_rts_panels(mut panels: Query<&mut Visibility, With<RtsPanel>>) {
    for mut vis in &mut panels {
        *vis = Visibility::Hidden;
    }
}
