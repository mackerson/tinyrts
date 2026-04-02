use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseScroll;

use crate::components::*;

/// RTS camera: WASD to pan, scroll to zoom
pub fn rts_camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    mut camera: Query<&mut Transform, With<RtsCamera>>,
) {
    let Ok(mut tf) = camera.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let pan_speed = 40.0;
    let zoom_speed = 5.0;

    let mut pan = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        pan.z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        pan.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        pan.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        pan.x += 1.0;
    }

    if pan.length_squared() > 0.0 {
        pan = pan.normalize() * pan_speed * dt;
        tf.translation += pan;
    }

    let scroll_y = scroll.delta.y;
    if scroll_y.abs() > 0.0 {
        tf.translation.y = (tf.translation.y - scroll_y * zoom_speed).clamp(15.0, 80.0);
    }
}

/// Possession camera: follows behind the possessed unit
pub fn possession_camera_follow(
    mut camera: Query<&mut Transform, With<RtsCamera>>,
    units: Query<&Transform, (With<PlayerControlled>, Without<RtsCamera>)>,
) {
    let Ok(unit_tf) = units.single() else {
        return;
    };

    let unit_pos = unit_tf.translation;
    let unit_rot = unit_tf.rotation;

    let Ok(mut cam_tf) = camera.single_mut() else {
        return;
    };

    let offset = unit_rot * Vec3::new(0.0, 6.0, 12.0);
    let target_pos = unit_pos + offset;

    cam_tf.translation = cam_tf.translation.lerp(target_pos, 0.1);
    cam_tf.look_at(unit_pos + Vec3::Y * 1.5, Vec3::Y);
}
