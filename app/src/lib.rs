pub mod scenes;
pub mod state;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_cameras::{
    pan_orbit_camera::{OrbitCameraController, OrbitCameraControllerPlugin},
    CameraMode,
};
use bevy_mod_picking::{
    debug::DebugPickingPlugin, prelude::low_latency_window_plugin, DefaultPickingPlugins,
};
use scenes::scene_1::Scene1Plugin;
use state::camera::CameraModeImpl;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
    Canvas3d,
}

#[derive(Component)]
pub struct MainCamera;

pub struct AppPlugin;

impl AppPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0., 0., -2.).looking_at(Vec3::ZERO, Vec3::Y),
                // projection: Projection::Orthographic(OrthographicProjection::default()),
                ..default()
            },
            MainCamera,
            OrbitCameraController::default(),
        ));
    }
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            OrbitCameraControllerPlugin::<CameraModeImpl>::default(),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
            Scene1Plugin,
        ))
        .add_state::<AppState>()
        .add_systems(Startup, Self::setup);
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn cleanup<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
