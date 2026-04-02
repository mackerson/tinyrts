use bevy::prelude::*;

use crate::components::Team;

/// Which camera mode we're in
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraMode {
    #[default]
    Rts,
    Possession,
}

/// Saved RTS camera transform to restore on unpossess
#[derive(Resource, Default)]
pub struct SavedCameraTransform(pub Option<Transform>);

/// Precomputed asset handles to avoid per-spawn leaks
#[derive(Resource)]
pub struct SharedAssets {
    // Scenes
    pub tank_scene: Handle<Scene>,
    pub base_scene: Handle<Scene>,
    pub barracks_scene: Handle<Scene>,
    // Projectiles (still use meshes — they're simple spheres)
    pub projectile_mesh: Handle<Mesh>,
    pub player_projectile_mesh: Handle<Mesh>,
    pub red_projectile_material: Handle<StandardMaterial>,
    pub blue_projectile_material: Handle<StandardMaterial>,
    pub player_projectile_material: Handle<StandardMaterial>,
}

impl SharedAssets {
    pub fn projectile_material(&self, team: &Team, is_player: bool) -> Handle<StandardMaterial> {
        if is_player {
            return self.player_projectile_material.clone();
        }
        match team {
            Team::Red => self.red_projectile_material.clone(),
            Team::Blue => self.blue_projectile_material.clone(),
        }
    }
}

// --- Constants ---

pub const PERCEPTION_RADIUS: f32 = 25.0;
pub const PROJECTILE_HIT_RADIUS: f32 = 1.0;
pub const PLAYER_SPEED_MULT: f32 = 1.5;
pub const PLAYER_FIRE_RATE_MULT: f32 = 0.5;
pub const PLAYER_DAMAGE_MULT: f32 = 1.5;

// Model scales (tuned to match game world)
pub const TANK_SCALE: f32 = 0.18;
pub const BASE_SCALE: f32 = 0.7;
pub const BARRACKS_SCALE: f32 = 2.0;
