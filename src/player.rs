use std::cmp::PartialEq;
use crate::constant::{BOARD_COUNT_X, BOARD_SIZE, GRAVITY, GROUND_CHECKER_TIMER, MOVING_H_TIME, MOVING_SPEED_H, PLAYER_Y, RUNNING_SPEED};
use bevy::asset::Handle;
use bevy::color::palettes::css::{PERU, PLUM};
use bevy::input::common_conditions::input_just_pressed;
use bevy::log::info;
use bevy::math::Quat;
use bevy::prelude::*;
use bevy::transform;
use bevy::utils::info;
use bevy_rapier3d::prelude::*;
use nalgebra::abs;
use std::time::Duration;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
pub struct MainCamera;

const JUMP_SPEED: f32 = 12f32;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerMark;


///移动的方向
#[derive(Eq, PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    NONE
}


#[derive(Component)]
pub struct Knight {
    moving_direction: Direction,
    up_speed: f32,
    on_ground: bool,
    /// lane >= -[BOARD_COUNT_X] and lane <= [BOARD_COUNT_X]
    lane : i32,
    target_lane: i32,
}

impl Default for Knight {
    fn default() -> Self {
        Self {
            moving_direction: Direction::NONE,
            up_speed:0.0,
            on_ground: false,
            lane: 0,
            target_lane: 0,
        }
    }
}

#[derive(Resource)]
pub struct KnightScene(Handle<Gltf>, bool);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct JUMPING;

#[derive(Resource)]
struct KnightAnimations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_knight)
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Update, (spawn_animation, setup_moving_animation))
            .add_systems(FixedUpdate, (check_moving_event, moving_knight).chain())
            .add_systems(PostUpdate, (display_events))
        ;
    }
}

fn setup_knight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let knight_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/player/knight.glb"));

    let player = Knight::default();

    let knight_height = 1.3;

    let mut knight_command = commands.spawn((
        PlayerMark,
        player,
        Transform::from_xyz(0f32, 20.0, 0f32),
        Collider::round_cylinder(knight_height / 2f32, 0.4, 0.4),
        ColliderDebugColor(Hsla::from(PLUM)),
        ActiveEvents::COLLISION_EVENTS,
        ContactForceEventThreshold(30.0),
        KinematicCharacterController {
            custom_mass: Some(5.0),
            up: Vec3::Y,
            offset: CharacterLength::Relative(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.4),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: true,
            }),
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: false,
            snap_to_ground: Some(CharacterLength::Relative(0.1)),
            ..default()
        },
    ));

    knight_command.with_children(|parent| {
        parent.spawn((
            SceneRoot(knight_scene.clone()),
            Transform::from_xyz(0f32, -knight_height, 0f32).with_rotation(Quat::from_rotation_y(180f32.to_radians())),
        ));

        parent.spawn((
            MainCamera,
            Camera3d::default(),
            Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            PanOrbitCamera::default(),
            // Transform::from_xyz(12.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
        ));
    });
}

fn setup_moving_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(72).from_asset("models/player/knight.glb"),
                GltfAssetLabel::Animation(73).from_asset("models/player/knight.glb"),

                //dodge_left
                GltfAssetLabel::Animation(29).from_asset("models/player/knight.glb"),
                //dodge_right
                GltfAssetLabel::Animation(30).from_asset("models/player/knight.glb"),

                //jump start
                GltfAssetLabel::Animation(42).from_asset("models/player/knight.glb"),
                //jump idle
                GltfAssetLabel::Animation(40).from_asset("models/player/knight.glb"),
                //jump land
                GltfAssetLabel::Animation(41).from_asset("models/player/knight.glb"),
            ]
                .into_iter()
                .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    commands.insert_resource(KnightAnimations {
        animations,
        graph: graph.clone(),
    });
}

fn spawn_animation(
    mut commands: Commands,
    animations: Res<KnightAnimations>,
    mut players: Query<(Entity, &Name, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, name, mut player) in &mut players {
        info!("animation name: {}", name);

        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[1], Duration::ZERO)
            .repeat()
            .set_speed(3f32);

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

fn  check_moving_event(
    time: Res<Time>,
    mut knight: Query<(&mut Knight, Option<&KinematicCharacterControllerOutput>),With<PlayerMark>, >,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut grounded_timer: Local<f32>,
) {
    let _ = knight.get_single_mut().map(|(mut knight, output)| {

        let mut jump = false;

        if knight.moving_direction == Direction::NONE {
            if keyboard.just_pressed(KeyCode::Space) {
                jump = true;
            } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
                if knight.target_lane == knight.lane && knight.lane > -BOARD_COUNT_X {
                    knight.target_lane -= 1;
                    knight.moving_direction = Direction::LEFT;
                    info!("moving left");
                }
            } else if keyboard.just_pressed(KeyCode::ArrowRight) {
                if knight.target_lane == knight.lane && knight.lane < BOARD_COUNT_X {
                    knight.target_lane += 1;
                    knight.moving_direction = Direction::RIGHT;
                    info!("moving right");
                }
            }
        }

        if output.map(|output| output.grounded).unwrap_or(false) {
            *grounded_timer = GROUND_CHECKER_TIMER;
            knight.up_speed = 0.0;
            if knight.moving_direction == Direction::UP {
                knight.moving_direction = Direction::NONE;
            }
        }

        if *grounded_timer > 0.0 {
            *grounded_timer -= time.delta_secs();
            if jump {
                knight.up_speed = 20.0;
                *grounded_timer = 0.0;
                knight.moving_direction = Direction::UP;
            }
        }
    });
}


fn moving_knight(
    mut commands: Commands,
    time: Res<Time>,
    mut knight_query: Query<(
        &mut Knight,
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    ), With<PlayerMark>>,
    mut target_moving_x: Local<f32>,
    mut start_moving_x: Local<bool>,
) {

    let _ = knight_query.get_single_mut().map(|kn| {

        let delta = time.delta_secs();

        let (mut knight, mut transform, mut controller, output) = kn;

        let mut moving_y = 0.0;

        let mut moving_x = 0.0;

        if knight.target_lane != knight.lane {
            moving_x = (knight.target_lane - knight.lane).signum() as f32 * MOVING_SPEED_H * delta;
        }

        let snap_to_right_place = ((moving_x > 0.0) && ((knight.target_lane as f32) * BOARD_SIZE > transform.translation.x)) || ((moving_x < 0.0) && ((knight.target_lane as f32) * BOARD_SIZE < transform.translation.x));

        if snap_to_right_place {
            info!("moving end");
            knight.lane = knight.target_lane;
            transform.translation.x = (knight.target_lane as f32) * BOARD_SIZE;
            knight.moving_direction = Direction::NONE;
            moving_x = 0.0;
        }

        // match knight.moving_direction {
        //
        //     Direction::LEFT => {
        //
        //         if transform.translation.x > -BOARD_SIZE * (BOARD_COUNT_X as f32) && !*start_moving_x {
        //             *start_moving_x = true;
        //             *target_moving_x = transform.translation.x - BOARD_SIZE;
        //         }
        //
        //         if *start_moving_x {
        //             moving_x = -MOVING_SPEED_H * delta;
        //         }
        //
        //         info!("moving_x: {}", moving_x);
        //
        //         if transform.translation.x <= *target_moving_x {
        //             *start_moving_x = false;
        //             knight.moving_direction = Direction::NONE;
        //             transform.translation.x = *target_moving_x;
        //             moving_x = 0.0;
        //
        //             info!("end moving");
        //         }
        //     }
        //
        //     Direction::RIGHT => {
        //
        //         if transform.translation.x < BOARD_SIZE * (BOARD_COUNT_X as f32) && !*start_moving_x {
        //             *start_moving_x = true;
        //             *target_moving_x = transform.translation.x + BOARD_SIZE;
        //         }
        //
        //         if *start_moving_x {
        //             moving_x = MOVING_SPEED_H * delta;
        //         }
        //
        //         info!("moving_x: {}", moving_x);
        //
        //         if transform.translation.x >= *target_moving_x {
        //             *start_moving_x = false;
        //             knight.moving_direction = Direction::NONE;
        //             transform.translation.x = *target_moving_x;
        //             moving_x = 0.0;
        //
        //             info!("end moving");
        //         }
        //     }
        //
        //     Direction::UP => {
        //
        //
        //
        //     }
        //
        //     Direction::NONE => {
        //
        //     }
        // }

        // match knight.moving_direction {
        //     Direction::LEFT | Direction::RIGHT => {
        //
        //         if !*start_moving_x {
        //             *start_moving_x = true;
        //             *target_moving_x = BOARD_SIZE;
        //             info!("start moving");
        //         }
        //
        //         if *start_moving_x {
        //
        //             let last_target = *target_moving_x;
        //
        //             *target_moving_x -= moving_x.abs();
        //
        //             if *target_moving_x <= 0.0 {
        //                 *start_moving_x = false;
        //                 *target_moving_x = 0.0;
        //                 moving_x = moving_x.signum() * last_target;
        //                 knight.moving_direction = Direction::NONE;
        //                 info!("left moving: {}", moving_x);
        //             }
        //         }
        //     }
        //     _ => {}
        // }
        //
        // info!("translation X: {:?}", transform.translation.x);

        moving_y = knight.up_speed * delta;
        knight.up_speed += GRAVITY * delta * controller.custom_mass.unwrap_or(1.0);

        controller.translation = Some(Vec3::new(moving_x, moving_y, delta * -RUNNING_SPEED));
    });
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}