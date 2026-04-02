use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;

use crate::components::*;
use crate::resources::*;
use crate::util::spawn_projectile;

/// In RTS mode: click to select units
pub fn rts_selection(
    mouse: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    windows: Query<&Window>,
    mut commands: Commands,
    units: Query<(Entity, &Transform, &Team), With<Unit>>,
    selected: Query<Entity, With<Selected>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((cam, cam_tf)) = camera.single() else {
        return;
    };

    let Ok(ray) = cam.viewport_to_world(cam_tf, cursor_pos) else {
        return;
    };

    // Intersect with y=0 plane
    let t = -ray.origin.y / ray.direction.y;
    if t < 0.0 {
        return;
    }
    let ground_point = ray.origin + *ray.direction * t;

    for entity in &selected {
        commands.entity(entity).remove::<Selected>();
    }

    let mut closest: Option<(Entity, f32)> = None;
    for (entity, tf, team) in &units {
        if *team != Team::Blue {
            continue;
        }
        let dist = tf.translation.distance(ground_point);
        if dist < 3.0 {
            if closest.is_none() || dist < closest.unwrap().1 {
                closest = Some((entity, dist));
            }
        }
    }

    if let Some((entity, _)) = closest {
        commands.entity(entity).insert(Selected);
    }
}

/// In possession mode: WASD to move, mouse to look
pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut units: Query<(&mut Transform, &MoveSpeed), With<PlayerControlled>>,
) {
    let Ok((mut tf, speed)) = units.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    // Steering: mouse + A/D rotate the tank
    let turn_speed = 2.5;
    let mut yaw_delta = -mouse_motion.delta.x * 0.003;
    if keys.pressed(KeyCode::KeyA) {
        yaw_delta += turn_speed * dt;
    }
    if keys.pressed(KeyCode::KeyD) {
        yaw_delta -= turn_speed * dt;
    }
    tf.rotate_y(yaw_delta);

    // Throttle: W/S drive forward/backward along facing direction
    let mut throttle: f32 = 0.0;
    if keys.pressed(KeyCode::KeyW) {
        throttle += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        throttle -= 0.6; // Reverse is slower
    }

    if throttle.abs() > 0.0 {
        let forward = tf.forward().as_vec3();
        let movement = forward * throttle * speed.0 * PLAYER_SPEED_MULT * dt;
        tf.translation += Vec3::new(movement.x, 0.0, movement.z);
        tf.translation.y = 0.0;
    }
}

/// In possession mode: click to shoot
pub fn player_shooting(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<SharedAssets>,
    mut units: Query<(&Transform, &mut Weapon, &Team), With<PlayerControlled>>,
) {
    let Ok((tf, mut weapon, team)) = units.single_mut() else {
        return;
    };

    weapon.timer -= time.delta_secs();

    if mouse.pressed(MouseButton::Left) && weapon.timer <= 0.0 {
        weapon.timer = weapon.cooldown * PLAYER_FIRE_RATE_MULT;

        let dir = tf.forward().as_vec3();
        let spawn_pos = tf.translation + dir * 1.0 + Vec3::Y * 0.5;

        spawn_projectile(
            &mut commands,
            &assets,
            spawn_pos,
            dir,
            45.0,
            weapon.damage * PLAYER_DAMAGE_MULT,
            *team,
            true,
        );
    }
}
