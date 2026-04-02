use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::{SharedAssets, TANK_SCALE};

/// Model rotation offset: tank gun points along -X, we need it along -Z
const MODEL_YAW_OFFSET: f32 = -std::f32::consts::FRAC_PI_2;

/// Factories spawn units on a timer
pub fn factory_spawn(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<SharedAssets>,
    mut factories: Query<(&Transform, &Team, &mut Factory)>,
) {
    for (factory_tf, team, mut factory) in &mut factories {
        factory.spawn_timer.tick(time.delta());

        if !factory.spawn_timer.just_finished() {
            continue;
        }

        let mut rng = rand::rng();
        let offset = Vec3::new(
            rng.random_range(-3.0..3.0),
            0.0,
            rng.random_range(-3.0..3.0),
        );

        let spawn_pos = factory_tf.translation + offset;

        // Parent entity: game logic transform (position, facing, etc.)
        // Child entity: visual model with rotation offset to align gun to -Z
        commands.spawn((
            Transform::from_translation(spawn_pos),
            Visibility::default(),
            Unit,
            *team,
            Health::new(100.0),
            Weapon::default(),
            MoveSpeed(8.0),
            AiBrain,
        )).with_child((
            SceneRoot(assets.tank_scene.clone()),
            Transform::from_rotation(Quat::from_rotation_y(MODEL_YAW_OFFSET))
                .with_scale(Vec3::splat(TANK_SCALE)),
        ));
    }
}
