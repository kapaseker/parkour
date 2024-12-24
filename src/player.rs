use crate::constant::{BOARD_COUNT_X, BRICK_SIZE, GRAVITY, MOVING_SPEED_H, PLAYER_Y, RUNNING_SPEED};
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

#[derive(Component)]
pub struct Player {
    jump_speed: f32,
    y: f32,
    jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jump_speed: 0f32,
            y: PLAYER_Y,
            jumping: false,
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
            // .add_computed_state::<JUMPING>()
            .add_systems(Update, (spawn_animation, moving_knight, setup_moving_animation, display_events))
        ;
    }
}

fn setup_knight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let knight_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/player/knight.glb"));

    let player = Player::default();
    let y = player.y;

    let knight_height = 1.3;

    let mut knight_command = commands.spawn((
        PlayerMark,
        player,
        Transform::from_xyz(0f32, 200.0, 0f32),
        Collider::round_cylinder(knight_height / 2f32, 0.4, 0.4),
        ColliderDebugColor(Hsla::from(PLUM)),
        ActiveEvents::COLLISION_EVENTS,
        ContactForceEventThreshold(30.0),
        KinematicCharacterController {
            custom_mass: Some(5.0),
            up: Vec3::Y,
            offset: CharacterLength::Relative(0.1),
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

fn moving_knight(
    mut commands: Commands,
    time: Res<Time>,
    mut knight: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    ), With<PlayerMark>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut moving_z: Local<f32>,
    mut vertical_movement: Local<f32>,
    mut horizontal_movement: Local<f32>,
) {

    let _ = knight.get_single_mut().and_then(|kn| {

        let delta = time.delta_secs();

        *moving_z = delta * -RUNNING_SPEED;

        let (mut transform, mut controller, output) = kn;

        let moving_y = *vertical_movement * delta;

        output.map(|output| {
            info!("collision size: {}", &output.collisions.len());
            // for collision in &output.collisions {
            //
            // }
        });

        if output.map(|o| o.grounded).unwrap_or(false) {

            *vertical_movement = 0.0;

            //跳跃
            if keyboard.just_pressed(KeyCode::Space) && *horizontal_movement == 0.0 {
                *vertical_movement = 20.0;
            }

            if *vertical_movement == 0.0 {

                if keyboard.just_pressed(KeyCode::ArrowLeft) {
                    if transform.translation.x > -BRICK_SIZE * (BOARD_COUNT_X as f32) {
                        transform.translation.x -= BRICK_SIZE;
                    }
                }

                if keyboard.just_pressed(KeyCode::ArrowRight) {
                    if transform.translation.x < BRICK_SIZE * (BOARD_COUNT_X as f32) {
                        transform.translation.x += BRICK_SIZE;
                    }
                }
            }

        } else {
            *vertical_movement += GRAVITY * delta * controller.custom_mass.unwrap_or(1.0);
        }

        controller.translation = Some(Vec3::new(0.0, moving_y, *moving_z));
        Ok(())
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