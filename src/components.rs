use bevy::prelude::*;

// --- Team & Identity ---

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    Red,
    Blue,
}

impl Team {
    pub fn opposite(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Team::Red => Color::srgb(0.9, 0.2, 0.15),
            Team::Blue => Color::srgb(0.15, 0.3, 0.9),
        }
    }

    pub fn base_position(&self) -> Vec3 {
        match self {
            Team::Red => Vec3::new(0.0, 0.0, -45.0),
            Team::Blue => Vec3::new(0.0, 0.0, 45.0),
        }
    }
}

// --- Unit Components ---

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
    pub timer: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            damage: 15.0,
            range: 12.0,
            cooldown: 0.8,
            timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct MoveSpeed(pub f32);

// --- Control Components ---

/// Marker: this unit is driven by AI
#[derive(Component)]
pub struct AiBrain;

/// Marker: this unit is currently possessed by the player
#[derive(Component)]
pub struct PlayerControlled;

/// Marker: this unit is selected in RTS view
#[derive(Component)]
pub struct Selected;

// --- Orders ---

#[derive(Component)]
pub struct AttackTarget(pub Entity);


// --- Structures ---

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct Base;

#[derive(Component)]
pub struct Factory {
    pub spawn_timer: Timer,
}

/// Hit radius for projectile collision (units default to PROJECTILE_HIT_RADIUS)
#[derive(Component)]
pub struct HitRadius(pub f32);

// --- Projectiles ---

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub damage: f32,
    pub team: Team,
    pub lifetime: f32,
}

// --- Camera ---

#[derive(Component)]
pub struct RtsCamera;

