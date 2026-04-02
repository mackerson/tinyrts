use bevy::prelude::*;
use bevy::pbr::DistanceFog;

mod ai;
mod camera;
mod combat;
mod components;
mod factory;
mod player;
mod possession;
mod resources;
mod scenery;
mod ui;
mod util;

use components::*;
use resources::*;

/// Grass green used for ground, fog, and clear color so edges blend seamlessly.
const GRASS_COLOR: Color = Color::srgb(0.25, 0.35, 0.2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TinyRTS — TA meets Serious Sam".into(),
                resolution: (1280u32, 720u32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(GRASS_COLOR))
        .add_plugins(ui::UiPlugin)
        // State & resources
        .init_state::<CameraMode>()
        .init_resource::<SavedCameraTransform>()
        // Setup
        .add_systems(Startup, (setup_world, scenery::spawn_scenery))
        // Restore camera when returning to RTS
        .add_systems(OnEnter(CameraMode::Rts), possession::restore_camera_on_unpossess)
        // RTS mode systems
        .add_systems(
            Update,
            (
                camera::rts_camera_movement.run_if(in_state(CameraMode::Rts)),
                player::rts_selection.run_if(in_state(CameraMode::Rts)),
            ),
        )
        // Possession mode systems
        .add_systems(
            Update,
            (
                camera::possession_camera_follow.run_if(in_state(CameraMode::Possession)),
                player::player_movement.run_if(in_state(CameraMode::Possession)),
                player::player_shooting.run_if(in_state(CameraMode::Possession)),
            ),
        )
        // Always-running systems
        .add_systems(
            Update,
            (
                ai::ai_find_targets,
                ai::ai_movement,
                ai::ai_clear_dead_targets,
                combat::weapon_firing,
                combat::projectile_movement,
                combat::projectile_hit_detection,
                combat::death_system,
                combat::building_death_system,
                factory::factory_spawn,
                possession::possession_input,
                possession::possessed_unit_death_check,
                hud_system,
            ),
        )
        .run();
}

fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Precompute shared asset handles
    let shared = SharedAssets {
        tank_scene: asset_server.load("models/Tank.glb#Scene0"),
        base_scene: asset_server.load("models/Base Large.glb#Scene0"),
        barracks_scene: asset_server.load("models/Barracks.glb#Scene0"),
        projectile_mesh: meshes.add(Sphere::new(0.15)),
        player_projectile_mesh: meshes.add(Sphere::new(0.2)),
        red_projectile_material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.3),
            emissive: LinearRgba::new(2.0, 1.0, 0.6, 1.0),
            ..default()
        }),
        blue_projectile_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 1.0),
            emissive: LinearRgba::new(0.6, 1.0, 2.0, 1.0),
            ..default()
        }),
        player_projectile_material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.3),
            emissive: LinearRgba::new(1.0, 1.0, 0.3, 1.0),
            ..default()
        }),
    };

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(300.0, 300.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GRASS_COLOR,
            ..default()
        })),
        Transform::from_translation(Vec3::ZERO),
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            illuminance: 12000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -2.0, -1.5), Vec3::Y),
    ));


    // --- Red team ---
    let red_base_pos = Team::Red.base_position();

    commands.spawn((
        SceneRoot(shared.base_scene.clone()),
        Transform::from_translation(red_base_pos)
            .with_scale(Vec3::splat(BASE_SCALE)),
        Building,
        Base,
        Team::Red,
        Health::new(2000.0),
        HitRadius(4.0),
    ));

    commands.spawn((
        SceneRoot(shared.barracks_scene.clone()),
        Transform::from_translation(red_base_pos + Vec3::new(8.0, 0.0, 5.0))
            .with_scale(Vec3::splat(BARRACKS_SCALE)),
        Building,
        Factory {
            spawn_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        },
        Team::Red,
        Health::new(800.0),
        HitRadius(3.0),
    ));

    // --- Blue team (player's team) ---
    let blue_base_pos = Team::Blue.base_position();

    commands.spawn((
        SceneRoot(shared.base_scene.clone()),
        Transform::from_translation(blue_base_pos)
            .with_scale(Vec3::splat(BASE_SCALE)),
        Building,
        Base,
        Team::Blue,
        Health::new(2000.0),
        HitRadius(4.0),
    ));

    commands.spawn((
        SceneRoot(shared.barracks_scene.clone()),
        Transform::from_translation(blue_base_pos + Vec3::new(-8.0, 0.0, -5.0))
            .with_scale(Vec3::splat(BARRACKS_SCALE)),
        Building,
        Factory {
            spawn_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        },
        Team::Blue,
        Health::new(800.0),
        HitRadius(3.0),
    ));

    commands.insert_resource(shared);

    // RTS Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 80.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        RtsCamera,
        // Ambient fill so shadows aren't pitch black
        AmbientLight {
            color: Color::srgb(0.9, 0.95, 1.0),
            brightness: 400.0,
            ..default()
        },
        // Distance fog blends ground edges into the sky color for a seamless horizon
        DistanceFog {
            color: GRASS_COLOR,
            falloff: FogFalloff::Linear {
                start: 100.0,
                end: 180.0,
            },
            ..default()
        },
    ));
}

fn hud_system(
    mut gizmos: Gizmos,
    units: Query<(&Transform, &Health, &Team, Option<&Selected>, Option<&PlayerControlled>), With<Unit>>,
    buildings: Query<(&Transform, &Health, &Team), With<Building>>,
) {
    // Building health bars (wider, higher)
    for (tf, health, team) in &buildings {
        let health_pct = health.current / health.max;
        let bar_width = 4.0;
        let bar_pos = tf.translation + Vec3::Y * 6.0;

        let team_color = team.color();
        gizmos.line(
            bar_pos - Vec3::X * bar_width * 0.5,
            bar_pos + Vec3::X * bar_width * 0.5,
            Color::srgb(0.15, 0.15, 0.15),
        );

        let bar_color = if health_pct > 0.5 {
            Color::srgb(0.2, 0.8, 0.2)
        } else if health_pct > 0.25 {
            Color::srgb(0.8, 0.6, 0.1)
        } else {
            Color::srgb(0.8, 0.2, 0.2)
        };

        gizmos.line(
            bar_pos - Vec3::X * bar_width * 0.5,
            bar_pos - Vec3::X * bar_width * 0.5 + Vec3::X * bar_width * health_pct,
            bar_color,
        );

        // Team marker below health bar
        let marker_pos = bar_pos - Vec3::Y * 0.3;
        gizmos.line(
            marker_pos - Vec3::X * 0.5,
            marker_pos + Vec3::X * 0.5,
            team_color,
        );
    }

    for (tf, health, team, selected, possessed) in &units {
        let health_pct = health.current / health.max;
        let bar_width = 1.5;
        let bar_pos = tf.translation + Vec3::Y * 2.5;

        // Team-colored diamond above unit
        let team_color = team.color();
        let diamond_pos = bar_pos + Vec3::Y * 0.5;
        let diamond_size = 0.3;
        gizmos.line(diamond_pos + Vec3::Y * diamond_size, diamond_pos + Vec3::X * diamond_size, team_color);
        gizmos.line(diamond_pos + Vec3::X * diamond_size, diamond_pos - Vec3::Y * diamond_size, team_color);
        gizmos.line(diamond_pos - Vec3::Y * diamond_size, diamond_pos - Vec3::X * diamond_size, team_color);
        gizmos.line(diamond_pos - Vec3::X * diamond_size, diamond_pos + Vec3::Y * diamond_size, team_color);

        // Selection ring
        if selected.is_some() {
            let ring_color = Color::srgb(1.0, 1.0, 0.3);
            let segments = 16;
            let radius = 1.5;
            let ring_y = tf.translation.y + 0.1;
            for i in 0..segments {
                let a0 = (i as f32 / segments as f32) * std::f32::consts::TAU;
                let a1 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
                gizmos.line(
                    Vec3::new(tf.translation.x + a0.cos() * radius, ring_y, tf.translation.z + a0.sin() * radius),
                    Vec3::new(tf.translation.x + a1.cos() * radius, ring_y, tf.translation.z + a1.sin() * radius),
                    ring_color,
                );
            }
        }

        // Possession glow ring
        if possessed.is_some() {
            let glow_color = Color::srgb(1.0, 0.8, 0.2);
            let segments = 16;
            let radius = 1.8;
            let ring_y = tf.translation.y + 0.1;
            for i in 0..segments {
                let a0 = (i as f32 / segments as f32) * std::f32::consts::TAU;
                let a1 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
                gizmos.line(
                    Vec3::new(tf.translation.x + a0.cos() * radius, ring_y, tf.translation.z + a0.sin() * radius),
                    Vec3::new(tf.translation.x + a1.cos() * radius, ring_y, tf.translation.z + a1.sin() * radius),
                    glow_color,
                );
            }
        }

        // Health bar background
        gizmos.line(
            bar_pos - Vec3::X * bar_width * 0.5,
            bar_pos + Vec3::X * bar_width * 0.5,
            Color::srgb(0.2, 0.2, 0.2),
        );

        // Health bar fill
        let bar_color = if health_pct > 0.5 {
            Color::srgb(0.2, 0.8, 0.2)
        } else {
            Color::srgb(0.8, 0.2, 0.2)
        };

        gizmos.line(
            bar_pos - Vec3::X * bar_width * 0.5,
            bar_pos - Vec3::X * bar_width * 0.5 + Vec3::X * bar_width * health_pct,
            bar_color,
        );
    }
}
