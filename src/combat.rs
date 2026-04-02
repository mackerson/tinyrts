use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::components::*;
use crate::resources::*;
use crate::util::spawn_projectile;

/// Units with targets in range fire projectiles
pub fn weapon_firing(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<SharedAssets>,
    mut shooters: Query<(Entity, &Transform, &mut Weapon, &Team, Option<&AttackTarget>), With<Unit>>,
    buildings: Query<(Entity, &Transform), (With<Building>, Without<Unit>)>,
) {
    let dt = time.delta_secs();

    let mut positions: HashMap<Entity, Vec3> = shooters
        .iter()
        .map(|(e, tf, _, _, _)| (e, tf.translation))
        .collect();

    for (e, tf) in &buildings {
        positions.insert(e, tf.translation);
    }

    for (_shooter_entity, shooter_tf, mut weapon, team, attack_target) in &mut shooters {
        weapon.timer -= dt;

        let Some(AttackTarget(target_entity)) = attack_target else {
            continue;
        };

        let Some(&target_pos) = positions.get(target_entity) else {
            continue;
        };

        let dist = shooter_tf.translation.distance(target_pos);
        if dist > weapon.range || weapon.timer > 0.0 {
            continue;
        }

        weapon.timer = weapon.cooldown;

        let dir = (target_pos - shooter_tf.translation).normalize_or_zero();
        let spawn_pos = shooter_tf.translation + dir * 0.8 + Vec3::Y * 0.5;

        spawn_projectile(&mut commands, &assets, spawn_pos, dir, 30.0, weapon.damage, *team, false);
    }
}

/// Move projectiles and tick their lifetime
pub fn projectile_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    let dt = time.delta_secs();

    for (entity, mut tf, mut proj) in &mut projectiles {
        tf.translation += proj.velocity * dt;
        proj.lifetime -= dt;

        if proj.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Check projectile collisions with enemy units
pub fn projectile_hit_detection(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Projectile), (Without<Unit>, Without<Building>)>,
    mut units: Query<(&Transform, &Team, &mut Health, Option<&HitRadius>), (With<Unit>, Without<Building>)>,
    mut buildings: Query<(&Transform, &Team, &mut Health, &HitRadius), (With<Building>, Without<Unit>)>,
) {
    for (proj_entity, proj_tf, proj) in &projectiles {
        let mut hit = false;

        // Check units
        for (unit_tf, unit_team, mut health, hit_radius) in &mut units {
            if *unit_team == proj.team {
                continue;
            }
            let radius = hit_radius.map_or(PROJECTILE_HIT_RADIUS, |r| r.0);
            let dist = proj_tf.translation.distance(unit_tf.translation);
            if dist < radius {
                health.current -= proj.damage;
                hit = true;
                break;
            }
        }

        // Check buildings if no unit was hit
        if !hit {
            for (bld_tf, bld_team, mut health, hit_radius) in &mut buildings {
                if *bld_team == proj.team {
                    continue;
                }
                let dist = proj_tf.translation.distance(bld_tf.translation);
                if dist < hit_radius.0 {
                    health.current -= proj.damage;
                    hit = true;
                    break;
                }
            }
        }

        if hit {
            commands.entity(proj_entity).despawn();
        }
    }
}

/// Remove dead units
pub fn death_system(
    mut commands: Commands,
    units: Query<(Entity, &Health), With<Unit>>,
) {
    for (entity, health) in &units {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Remove dead buildings
pub fn building_death_system(
    mut commands: Commands,
    buildings: Query<(Entity, &Health), With<Building>>,
) {
    for (entity, health) in &buildings {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
