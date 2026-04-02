use bevy::prelude::*;
use bevy::platform::collections::HashMap;

use crate::components::*;
use crate::resources::PERCEPTION_RADIUS;
use crate::util::look_at_flat;

const BUILDING_AGGRO_RADIUS: f32 = 15.0;

/// AI units without a target scan for the nearest enemy unit, or enemy building if close enough
pub fn ai_find_targets(
    mut commands: Commands,
    seekers: Query<(Entity, &Transform, &Team), (With<AiBrain>, With<Unit>, Without<AttackTarget>)>,
    enemy_units: Query<(Entity, &Transform, &Team), With<Unit>>,
    enemy_buildings: Query<(Entity, &Transform, &Team), With<Building>>,
) {
    for (seeker_entity, seeker_tf, seeker_team) in &seekers {
        let mut closest: Option<(Entity, f32)> = None;

        // Prefer enemy units
        for (enemy_entity, enemy_tf, enemy_team) in &enemy_units {
            if enemy_team == seeker_team {
                continue;
            }
            let dist = seeker_tf.translation.distance(enemy_tf.translation);
            if dist < PERCEPTION_RADIUS {
                if closest.is_none() || dist < closest.unwrap().1 {
                    closest = Some((enemy_entity, dist));
                }
            }
        }

        // If no enemy units nearby, target enemy buildings within aggro range
        if closest.is_none() {
            for (bld_entity, bld_tf, bld_team) in &enemy_buildings {
                if bld_team == seeker_team {
                    continue;
                }
                let dist = seeker_tf.translation.distance(bld_tf.translation);
                if dist < BUILDING_AGGRO_RADIUS {
                    if closest.is_none() || dist < closest.unwrap().1 {
                        closest = Some((bld_entity, dist));
                    }
                }
            }
        }

        if let Some((target, _)) = closest {
            commands.entity(seeker_entity).insert(AttackTarget(target));
        }
    }
}

/// AI units move toward their attack target or toward the enemy base
pub fn ai_movement(
    time: Res<Time>,
    mut units: Query<
        (Entity, &mut Transform, &Team, &MoveSpeed, &Weapon, Option<&AttackTarget>),
        (With<AiBrain>, With<Unit>, Without<PlayerControlled>),
    >,
    buildings: Query<(Entity, &Transform), (With<Building>, Without<Unit>)>,
) {
    let dt = time.delta_secs();

    // Snapshot positions of all potential targets (units + buildings)
    let mut positions: HashMap<Entity, Vec3> = units
        .iter()
        .map(|(e, tf, _, _, _, _)| (e, tf.translation))
        .collect();

    for (e, tf) in &buildings {
        positions.insert(e, tf.translation);
    }

    for (_entity, mut tf, team, speed, weapon, attack_target) in &mut units {
        let destination = if let Some(AttackTarget(target_entity)) = attack_target {
            if let Some(&target_pos) = positions.get(target_entity) {
                let dist = tf.translation.distance(target_pos);
                if dist <= weapon.range {
                    let dir = (target_pos - tf.translation).normalize_or_zero();
                    look_at_flat(&mut tf, dir);
                    continue;
                }
                target_pos
            } else {
                team.opposite().base_position()
            }
        } else {
            team.opposite().base_position()
        };

        let dir = (destination - tf.translation).normalize_or_zero();
        let dir = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();

        if dir.length_squared() > 0.0 {
            tf.translation += dir * speed.0 * dt;
            tf.translation.y = 0.0;
            look_at_flat(&mut tf, dir);
        }
    }
}

/// Clear attack targets that reference dead entities (units or buildings)
pub fn ai_clear_dead_targets(
    mut commands: Commands,
    units: Query<(Entity, &AttackTarget), With<Unit>>,
    alive_units: Query<Entity, With<Unit>>,
    alive_buildings: Query<Entity, With<Building>>,
) {
    for (entity, AttackTarget(target)) in &units {
        if alive_units.get(*target).is_err() && alive_buildings.get(*target).is_err() {
            commands.entity(entity).remove::<AttackTarget>();
        }
    }
}
