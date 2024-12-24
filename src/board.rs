use bevy::app::App;
use bevy::color::palettes::basic::RED;
use bevy::color::palettes::css::{PERU, SALMON};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::constant::*;

pub struct AppBoardPlugin;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct PlaneMark;

#[derive(Component)]
pub struct BrickConfig {
    has_barrier: bool,
}

#[derive(Resource)]
pub struct BarrierScene {
    tree: Handle<Scene>,
    cactus: Handle<Scene>,
}

#[derive(Resource)]
pub struct PlaneResource(Entity);

impl Plugin for AppBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, (lifecycle_board));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // spawn the game board
    // let cell_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board/tile_high_forest.glb"));

    let tree_scene: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board/tree_forest.glb"));
    let cactus_scene: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board/tree_desert.glb"));
    
    // ((-(BOARD_COUNT_Z - BOARD_Z_OFFSET))..BOARD_Z_OFFSET).for_each(|j| {
    //     ((-BOARD_COUNT_X)..=BOARD_COUNT_X)
    //         .for_each(|i| {
    // 
    //             let height = random_height();
    //             let gen_barrier = random_barrier();
    // 
    //             let brick_bundle = (
    //                 Transform::from_xyz(i as f32 * 2f32, height, j as f32 * 2f32),
    //                 SceneRoot(cell_scene.clone()),
    //                 RigidBody::Fixed,
    //             );
    //             
    //             let barrier_bundle = (
    //                 Transform::from_xyz(0f32, PLAYER_Y, 0f32),
    //                 SceneRoot(if random_planet() { tree_scene.clone() } else { cactus_scene.clone() }),
    //                 RigidBody::Fixed,
    //             );
    // 
    //             let mut brick_command = commands.spawn((Brick, BrickConfig { has_barrier: gen_barrier }, brick_bundle));
    //             if gen_barrier {
    //                 brick_command.with_children(|parent| {
    //                     parent.spawn(barrier_bundle).with_children(|parent| {
    //                         parent.spawn((
    //                             Collider::cuboid(1.0, 1.5, 1.0),
    //                             Transform::from_xyz(0.0, 1.5, 0.0)
    //                         ));
    //                     });
    //                 });
    //             }
    // 
    //             brick_command.with_children(|parent| {
    //                 parent.spawn((
    //                     Collider::cuboid(1.0, 1.0, 1.0),
    //                     Transform::from_xyz(0.0, 1.0, 0.0)
    //                 ));
    //             });
    //         })
    // });

    // set up sensor plane
    let plane = commands.spawn(
        (
            PlaneMark,
            RigidBody::Fixed,
            Collider::cuboid(5.0, 0.1, 5000.0),
            Mesh3d(meshes.add(Cuboid::new(10.0, 0.4, 10000.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            })),
            Transform::from_xyz(0.0, 2.4, 0.0),
            // Sensor,
            ColliderDebugColor(Hsla::from(SALMON)),
        )
    ).id();

    commands.insert_resource::<PlaneResource>(PlaneResource(plane));
}

///
/// 每隔一段时间，后面需要消去的砖块，移动到前面，嘻嘻嘻
///
fn lifecycle_board(
    mut life_delta: Local<f32>,
    time: Res<Time>,
    mut brick_query: Query<&mut Transform, With<Brick>>,
) {

    *life_delta += time.delta_secs();

    if *life_delta * RUNNING_SPEED > BRICK_SIZE {
        brick_query.iter_mut().sort_by::<&Transform>(|&a, &b| {
            b.translation.z.partial_cmp(&a.translation.z).unwrap()
        }).take(3).for_each(|mut brick| {
            let height = random_height();
            brick.translation.z -= BOARD_COUNT_Z as f32 * BRICK_SIZE;
            brick.translation.y = height;
        });

        *life_delta = 0f32;
    }
}

fn random_height() -> f32 {
    rand::thread_rng().gen_range(-BRICK_HEIGHT_RANDOM..BRICK_HEIGHT_RANDOM) - BRICK_HEIGHT_RANDOM
}

fn random_barrier() -> bool {
    false
    // rand::thread_rng().gen_bool(0.1f64)
}

fn random_planet() -> bool {
    rand::thread_rng().gen_bool(0.5f64)
}