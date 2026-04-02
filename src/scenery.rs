use bevy::prelude::*;
use rand::prelude::*;

/// Radius around each team base to keep clear of scenery.
const BASE_EXCLUSION_RADIUS: f32 = 20.0;

/// Half-extent of the scatter area (matches the 300x300 ground plane).
const SCATTER_HALF: f32 = 140.0;

/// Scatter definition: (path, count, scale_range)
struct ScatterEntry {
    paths: &'static [&'static str],
    count: u32,
    scale: (f32, f32),
}

const TREES: ScatterEntry = ScatterEntry {
    paths: &[
        "models/nature/Pine.glb#Scene0",
        "models/nature/Pine-699sFuLCN2.glb#Scene0",
        "models/nature/Pine-79gmlLnweB.glb#Scene0",
        "models/nature/Pine-rfnxJv0Rqa.glb#Scene0",
        "models/nature/Pine-Zt62gceKXZ.glb#Scene0",
        "models/nature/Tree.glb#Scene0",
        "models/nature/Tree-aVOxaHRPWe.glb#Scene0",
        "models/nature/Tree-QVOop92WmG.glb#Scene0",
        "models/nature/Tree-qZtx0AHhcy.glb#Scene0",
        "models/nature/Tree-t9KbsfYdXz.glb#Scene0",
    ],
    count: 40,
    scale: (0.8, 1.4),
};

const DEAD_TREES: ScatterEntry = ScatterEntry {
    paths: &[
        "models/nature/Dead Tree.glb#Scene0",
        "models/nature/Dead Tree-CD4edbPSGm.glb#Scene0",
        "models/nature/Dead Tree-Mcd2zYqyww.glb#Scene0",
        "models/nature/Dead Tree-MlmK5488ou.glb#Scene0",
        "models/nature/Dead Tree-n8FhMgMldD.glb#Scene0",
    ],
    count: 8,
    scale: (0.7, 1.2),
};

const TWISTED_TREES: ScatterEntry = ScatterEntry {
    paths: &[
        "models/nature/Twisted Tree.glb#Scene0",
        "models/nature/Twisted Tree-7PDBpElkQr.glb#Scene0",
        "models/nature/Twisted Tree-8oraKn9m0x.glb#Scene0",
        "models/nature/Twisted Tree-9aWlx82xUf.glb#Scene0",
        "models/nature/Twisted Tree-GVTsMmuzv7.glb#Scene0",
    ],
    count: 6,
    scale: (0.7, 1.1),
};

const ROCKS: ScatterEntry = ScatterEntry {
    paths: &[
        "models/nature/Rock Medium.glb#Scene0",
        "models/nature/Rock Medium-JQxF95498B.glb#Scene0",
        "models/nature/Rock Medium-s1OJ3bBzqc.glb#Scene0",
        "models/nature/Pebble Round.glb#Scene0",
        "models/nature/Pebble Round-icVsN3lmVy.glb#Scene0",
        "models/nature/Pebble Square.glb#Scene0",
        "models/nature/Pebble Square-6juX57sLHe.glb#Scene0",
    ],
    count: 30,
    scale: (0.6, 1.5),
};

const BUSHES: ScatterEntry = ScatterEntry {
    paths: &[
        "models/nature/Bush.glb#Scene0",
        "models/nature/Bush with Flowers.glb#Scene0",
        "models/nature/Plant.glb#Scene0",
        "models/nature/Plant Big.glb#Scene0",
        "models/nature/Plant Big-MbhbP7JrTI.glb#Scene0",
        "models/nature/Fern.glb#Scene0",
    ],
    count: 25,
    scale: (0.8, 1.6),
};

const GROUND_COVER: ScatterEntry = ScatterEntry {
    paths: &[
        "models/nature/Grass.glb#Scene0",
        "models/nature/Grass Wispy.glb#Scene0",
        "models/nature/Tall Grass.glb#Scene0",
        "models/nature/Clover.glb#Scene0",
        "models/nature/Mushroom.glb#Scene0",
        "models/nature/Flower Single.glb#Scene0",
        "models/nature/Flower Group.glb#Scene0",
    ],
    count: 40,
    scale: (0.8, 1.8),
};

/// Team base positions to exclude from scatter.
const BASE_POSITIONS: [Vec3; 2] = [
    Vec3::new(0.0, 0.0, -45.0), // Red
    Vec3::new(0.0, 0.0, 45.0),  // Blue
];

fn is_near_base(pos: Vec3) -> bool {
    BASE_POSITIONS
        .iter()
        .any(|base| base.distance(pos) < BASE_EXCLUSION_RADIUS)
}

pub fn spawn_scenery(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = StdRng::seed_from_u64(42);

    let entries = [&TREES, &DEAD_TREES, &TWISTED_TREES, &ROCKS, &BUSHES, &GROUND_COVER];

    for entry in entries {
        let mut spawned = 0;
        let mut attempts = 0;
        while spawned < entry.count && attempts < entry.count * 4 {
            attempts += 1;

            let x = rng.random_range(-SCATTER_HALF..SCATTER_HALF);
            let z = rng.random_range(-SCATTER_HALF..SCATTER_HALF);
            let pos = Vec3::new(x, 0.0, z);

            if is_near_base(pos) {
                continue;
            }

            let path = entry.paths[rng.random_range(0..entry.paths.len())];
            let scale = rng.random_range(entry.scale.0..entry.scale.1);
            let yaw = rng.random_range(0.0..std::f32::consts::TAU);

            commands.spawn((
                SceneRoot(asset_server.load(path)),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(yaw))
                    .with_scale(Vec3::splat(scale)),
            ));

            spawned += 1;
        }
    }
}
