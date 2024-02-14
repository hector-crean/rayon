use bevy::prelude::*;
use bevy_mod_picking::{backends::raycast::RaycastPickable, PickableBundle};
use bevy_mod_raycast::deferred::RaycastMesh;

// Must insert this bundle into any entity, which we want to make draggeable
#[derive(Bundle, Default)]
pub struct TransformableBundle {
    pickable_bundle: PickableBundle,
    transform_controller: TransformController,
}

#[derive(Resource)]
pub struct TransformControllerSettings {
    pub enabled: bool,
}
impl Default for TransformControllerSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Component)]
pub struct TransformController {
    pub enabled: bool,
    pub drag_start_pointer_position: Option<Vec3>,
    pub drag_start_entity_position: Option<Vec3>,
}
impl Default for TransformController {
    fn default() -> Self {
        Self {
            enabled: true,
            drag_start_pointer_position: None,
            drag_start_entity_position: None,
        }
    }
}
