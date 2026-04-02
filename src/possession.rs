use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

use crate::components::*;
use crate::resources::*;

/// Press F to possess selected unit, Tab to unpossess
pub fn possession_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<CameraMode>>,
    mut saved_cam: ResMut<SavedCameraTransform>,
    selected: Query<Entity, With<Selected>>,
    current_possessed: Query<Entity, With<PlayerControlled>>,
    mut cursor_opts: Query<&mut CursorOptions>,
    camera: Query<&Transform, With<RtsCamera>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        if current_possessed.iter().next().is_some() {
            unpossess(&mut commands, &mut next_state, &current_possessed, &mut cursor_opts);
        } else if let Some(entity) = selected.iter().next() {
            if let Ok(cam_tf) = camera.single() {
                saved_cam.0 = Some(*cam_tf);
            }

            commands.entity(entity)
                .insert(PlayerControlled)
                .remove::<AiBrain>()
                .remove::<Selected>();
            next_state.set(CameraMode::Possession);

            if let Ok(mut opts) = cursor_opts.single_mut() {
                opts.grab_mode = CursorGrabMode::Locked;
                opts.visible = false;
            }
        }
    }

    if keys.just_pressed(KeyCode::Tab) {
        if current_possessed.iter().next().is_some() {
            unpossess(&mut commands, &mut next_state, &current_possessed, &mut cursor_opts);
        }
    }
}

fn unpossess(
    commands: &mut Commands,
    next_state: &mut ResMut<NextState<CameraMode>>,
    current_possessed: &Query<Entity, With<PlayerControlled>>,
    cursor_opts: &mut Query<&mut CursorOptions>,
) {
    for entity in current_possessed.iter() {
        commands.entity(entity)
            .remove::<PlayerControlled>()
            .insert(AiBrain);
    }
    next_state.set(CameraMode::Rts);

    if let Ok(mut opts) = cursor_opts.single_mut() {
        opts.grab_mode = CursorGrabMode::None;
        opts.visible = true;
    }
}

/// On entering RTS mode, restore the saved camera position
pub fn restore_camera_on_unpossess(
    mut saved_cam: ResMut<SavedCameraTransform>,
    mut camera: Query<&mut Transform, With<RtsCamera>>,
) {
    if let Some(saved) = saved_cam.0.take() {
        if let Ok(mut cam_tf) = camera.single_mut() {
            *cam_tf = saved;
        }
    }
}

/// If the possessed unit dies, return to RTS mode
pub fn possessed_unit_death_check(
    camera_mode: Res<State<CameraMode>>,
    mut next_state: ResMut<NextState<CameraMode>>,
    possessed: Query<Entity, With<PlayerControlled>>,
    mut cursor_opts: Query<&mut CursorOptions>,
) {
    if *camera_mode.get() != CameraMode::Possession {
        return;
    }

    if possessed.iter().next().is_none() {
        next_state.set(CameraMode::Rts);

        if let Ok(mut opts) = cursor_opts.single_mut() {
            opts.grab_mode = CursorGrabMode::None;
            opts.visible = true;
        }
    }
}
