use bevy::prelude::*;

use crate::components::*;
use crate::resources::SharedAssets;

/// Face a horizontal direction without pitching up/down
pub fn look_at_flat(tf: &mut Transform, dir: Vec3) {
    let flat_dir = Vec3::new(dir.x, 0.0, dir.z);
    if flat_dir.length_squared() > 0.0 {
        let look_target = tf.translation + flat_dir;
        tf.look_at(look_target, Vec3::Y);
    }
}

/// Spawn a projectile entity with shared asset handles
pub fn spawn_projectile(
    commands: &mut Commands,
    assets: &SharedAssets,
    pos: Vec3,
    dir: Vec3,
    speed: f32,
    damage: f32,
    team: Team,
    is_player: bool,
) {
    let mesh = if is_player {
        assets.player_projectile_mesh.clone()
    } else {
        assets.projectile_mesh.clone()
    };

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(assets.projectile_material(&team, is_player)),
        Transform::from_translation(pos),
        Projectile {
            velocity: dir * speed,
            damage,
            team,
            lifetime: 2.0,
        },
    ));
}
