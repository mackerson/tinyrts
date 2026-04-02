# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo run                # Dev build (opt-level 1 for crate, 3 for deps)
cargo run --release      # Full optimization
cargo build              # Build without running
```

No test suite yet. No linter configured beyond default `rustc` warnings.

## Architecture

Bevy 0.18 ECS game — an RTS/FPS hybrid where you command units top-down then possess any unit for first-person tank combat. ~1,500 lines of Rust.

### Authority Handoff (core pattern)

The central design is a **component swap** that transfers control between AI and player:

- **Possess**: remove `AiBrain`, insert `PlayerControlled` → camera follows unit, player drives
- **Unpossess**: remove `PlayerControlled`, insert `AiBrain` → camera restores, AI resumes

All combat, health, and death systems are control-agnostic — they query `With<Unit>` regardless of who's driving. The `CameraMode` state (`Rts` / `Possession`) gates which input/camera systems run via `run_if(in_state(...))`.

### Parent/Child Scene Pattern

Units use a two-entity hierarchy to decouple game logic from visual models:

```
Parent: Transform (position, facing) + Unit + Team + Health + Weapon + AiBrain
  └─ Child: SceneRoot(tank.glb) + Transform(rotation offset + scale)
```

The model's "forward" (-X axis) is rotated to align with Bevy's -Z forward via `MODEL_YAW_OFFSET`. Game systems only touch the parent's Transform.

### Query Disjointness

Bevy can't prove `With<Unit>` and `With<Building>` are disjoint. Any system that mutably queries both **must** add explicit `Without<>` filters:

```rust
mut units: Query<..., (With<Unit>, Without<Building>)>,
mut buildings: Query<..., (With<Building>, Without<Unit>)>,
```

Systems that read both positions use a **snapshot pattern** — collect positions into a `HashMap<Entity, Vec3>` before mutable iteration to avoid conflicting borrows.

### SharedAssets Resource

Mesh and material handles are created once in `setup_world()` and stored in the `SharedAssets` resource. All spawns clone handles from this resource. **Never call `meshes.add()` or `materials.add()` per-spawn** — this leaks assets.

### Module Layout

- `main.rs` — App setup, world spawning, gizmo HUD (health bars, selection rings)
- `components.rs` — All ECS components (Team, Unit, Building, Health, Weapon, markers)
- `resources.rs` — CameraMode state, SharedAssets, gameplay constants
- `ai.rs` — Target scanning (units preferred over buildings), movement, dead target cleanup
- `camera.rs` — RTS pan/zoom + third-person follow cam
- `combat.rs` — Weapon firing, projectile movement, hit detection (units + buildings), death
- `factory.rs` — Timer-based unit spawning with parent/child scene hierarchy
- `player.rs` — RTS click-select, tank steering (W/S throttle, A/D yaw), shooting
- `possession.rs` — F/Tab mode switching, camera save/restore, death-unpossess
- `util.rs` — `look_at_flat()`, `spawn_projectile()` helpers
- `ui/` — Screen-space HUD plugin (unit counts, mode indicator, controls hint)

### Gameplay Constants (resources.rs)

All tuning values are named constants: `PERCEPTION_RADIUS`, `PROJECTILE_HIT_RADIUS`, `PLAYER_SPEED_MULT`, `PLAYER_FIRE_RATE_MULT`, `PLAYER_DAMAGE_MULT`, model scale constants. Change these to tune gameplay.

## Bevy 0.18 API Notes

- `EventReader` → `MessageReader` (or use `AccumulatedMouseMotion`/`AccumulatedMouseScroll` resources)
- `Window.cursor_options` → `CursorOptions` is a separate required component, query it independently
- `WindowResolution::from()` takes `(u32, u32)` not `(f32, f32)`
- `single()`/`single_mut()` return `Result`, use `let Ok(x) = query.single() else { return; }`
- `run_if()` must be applied to individual systems, not tuples
- `emissive` field on `StandardMaterial` expects `LinearRgba`, not `Color`
- `despawn()` is recursive by default (no `despawn_recursive` needed)

## Asset Pipeline

glTF models in `assets/models/` loaded via `asset_server.load("models/Tank.glb#Scene0")`. Models use Blender's scale=100 export convention. Tank, Base Large, Barracks are active; SWAT, House, Geodesic Dome, Solar Panel are available but unwired.
