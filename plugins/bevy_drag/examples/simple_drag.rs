//! https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
//!
//!
//!

use bevy::prelude::*;
use bevy_cameras::{
    pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin},
    CameraMode,
};
use bevy_drag::{Transformable, TransformablePlugin};

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // When any of this entity's children are interacted with using a pointer, those events will
        // propagate up the entity hierarchy until they reach this parent. By referring to the
        // `target` entity instead of the `listener` entity, we can do things to specific target
        // entities, even though they lack `OnPointer` components.
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::WHITE.into()),
                ..Default::default()
            },
            Transformable::default(),
        ))
        .with_children(|parent| {
            for i in 1..=5 {
                parent.spawn((
                    // As noted above, we are adding children here but we don't need to add an event
                    // listener. Events on children will bubble up to the parent!
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                        material: materials.add(Color::RED.into()),
                        transform: Transform::from_xyz(0.0, 1.0 + 0.5 * i as f32, 0.0),
                        ..Default::default()
                    },
                    Transformable::default(),
                ));
            }
        });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0., 0., -2.).looking_at(Vec3::ZERO, Vec3::Y),
            // projection: Projection::Orthographic(OrthographicProjection::default()),
            ..default()
        })
        .insert(OrbitCameraController::default());
}

#[derive(Debug, PartialEq, Eq)]
pub enum CameraModes {
    //     Orbiting: The camera rotates around a target object or point of interest. The camera's movement is constrained to a certain distance from the target and a fixed angle of inclination.
    Orbiting {
        target: Option<Entity>,
        // distance: i32,
        // elevation: i32,
        // azimuth: i32,
    },
    // Following: The camera follows a target object or character as it moves through the scene. The camera's movement is constrained to a certain distance and angle from the target.
    Following {
        target: Entity,
        // offset: Vec3,
    },
    // First-person: The camera is positioned at the player's eye level and follows the player's movements. The camera's movement is generally limited to the player's movements and the player's field of view.
    FirstPerson {
        // yaw: f32,
        // pitch: f32,
    },
    // Third-person: The camera is positioned behind the player and follows the player's movements. The camera's movement is generally limited to a certain distance and angle from the player.
    ThirdPerson {},
    // Top-down: The camera is positioned directly above the scene and provides a bird's-eye view of the action. The camera's movement is generally limited to panning and zooming.
    TopDown,
    // Cinematic: The camera is used to create a cinematic effect, such as a cutscene or dramatic reveal. The camera's movement is generally scripted and may include special effects such as depth of field or motion blur.
    Cinematic,
}

impl Default for CameraModes {
    fn default() -> Self {
        Self::Orbiting { target: None }
    }
}

#[derive(Debug, PartialEq, Eq, Resource, Default)]
pub struct CameraModeImpl {
    locked: bool,
    mode: CameraModes,
}

impl CameraMode for CameraModeImpl {
    fn is_locked(&self) -> bool {
        self.locked
    }

    fn lock(&mut self) {
        self.locked = true;
    }

    fn unlock(&mut self) {
        self.locked = false
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
            TransformablePlugin::<CameraModeImpl>::default(),
            OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
        ))
        .add_systems(Startup, (setup_scene, setup_camera))
        .run();
}
