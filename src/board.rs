use crate::constant::*;
use crate::player::KnightMark;
use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::time::Duration;

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
pub struct BoardLifeTimer(Timer);

// #[derive(Resource)]
// pub struct PlaneResource(Entity);

impl Plugin for AppBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, (lifecycle_board)).insert_resource(BoardLifeTimer(Timer::new(Duration::from_secs(3), TimerMode::Repeating)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // spawn the game board
    let cell_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board/tile_high_forest.glb"));

    let tree_scene: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board/tree_forest.glb"));
    let cactus_scene: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/board/tree_desert.glb"));
    
    ((-(BOARD_COUNT_Z - BOARD_Z_OFFSET))..BOARD_Z_OFFSET).for_each(|j| {
        ((-BOARD_COUNT_X)..=BOARD_COUNT_X)
            .for_each(|i| {

                let height = random_height();
                let gen_barrier = random_barrier();

                let brick_bundle = (
                    Transform::from_xyz(i as f32 * 2f32, height, j as f32 * 2f32),
                    SceneRoot(cell_scene.clone()),
                    RigidBody::Fixed,
                );

                let barrier_bundle = (
                    Transform::from_xyz(0f32, PLAYER_Y, 0f32),
                    SceneRoot(if random_planet() { tree_scene.clone() } else { cactus_scene.clone() }),
                    RigidBody::Fixed,
                );

                let mut brick_command = commands.spawn((Brick, BrickConfig { has_barrier: gen_barrier }, brick_bundle));
                if gen_barrier {
                    brick_command.with_children(|parent| {
                        parent.spawn(barrier_bundle).with_children(|parent| {
                            parent.spawn((
                                Collider::cuboid(0.8, 1.5, 0.8),
                                Transform::from_xyz(0.0, 1.5, 0.0)
                            ));
                        });
                    });
                }

                brick_command.with_children(|parent| {
                    parent.spawn((
                        Collider::cuboid(1.0, 1.0, 1.0),
                        Transform::from_xyz(0.0, 1.0, 0.0)
                    ));
                });
            })
    });
}

///
/// 每隔一段时间，后面需要消去的砖块，移动到前面，嘻嘻嘻
///
fn lifecycle_board(
    time:Res<Time>,
    mut life_timer: ResMut<BoardLifeTimer>,
    mut player_brick_query: ParamSet<(Query<&Transform, With<KnightMark>>, Query<&mut Transform, With<Brick>>)>,
) {

    if life_timer.0.tick(time.delta()).just_finished() {

        let knight_query = player_brick_query.p0();

        if let Ok(&knight) = knight_query.get_single() {

            let knight_z = knight.translation.z;

            let mut brick_query = player_brick_query.p1();
            brick_query.iter_mut().filter(|brick| {
                brick.translation.z > (BOARD_Z_OFFSET as f32 + knight_z)
            }).for_each(|mut brick| {
                let height = random_height();
                brick.translation.z -= BOARD_COUNT_Z as f32 * BOARD_SIZE;
                brick.translation.y = height;
            });
        }
    }
}

fn random_height() -> f32 {
    rand::thread_rng().gen_range(-BOARD_HEIGHT_RANDOM..BOARD_HEIGHT_RANDOM) - BOARD_HEIGHT_RANDOM
}

fn random_barrier() -> bool {
    // false
    rand::thread_rng().gen_bool(0.1f64)
}

fn random_planet() -> bool {
    rand::thread_rng().gen_bool(0.5f64)
}