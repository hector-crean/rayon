use bevy::prelude::*;
use bevy_mod_picking::{events::PointerCancel, prelude::*};

#[derive(Event, Debug)]
pub enum EntityPointerEvent {
    Drag {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Drag,
    },
    DragStart {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: DragStart,
    },
    DragEnd {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: DragEnd,
    },
    DragOver {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: DragOver,
    },
    DragLeave {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: DragLeave,
    },
    Drop {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Drop,
    },
    Out {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Out,
    },
    Over {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Over,
    },
    Click {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Click,
    },
    Down {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Down,
    },
    Up {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Up,
    },
    Move {
        entity: Entity,
        pointer_id: PointerId,
        pointer_position: Vec2,
        data: Move,
    },
}

impl From<ListenerInput<Pointer<Drag>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        EntityPointerEvent::Drag {
            entity: event.target,
            data: Drag {
                button: event.button,
                distance: event.distance,
                delta: event.delta,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}
impl From<ListenerInput<Pointer<DragStart>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<DragStart>>) -> Self {
        EntityPointerEvent::DragStart {
            entity: event.target,
            data: DragStart {
                hit: event.hit.clone(),
                button: event.button,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}
impl From<ListenerInput<Pointer<DragEnd>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        EntityPointerEvent::DragEnd {
            entity: event.target,
            data: DragEnd {
                distance: event.distance,
                button: event.button,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Over>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        EntityPointerEvent::Over {
            entity: event.target,
            data: Over {
                hit: event.hit.clone(),
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<DragLeave>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<DragLeave>>) -> Self {
        EntityPointerEvent::DragLeave {
            entity: event.target,
            data: DragLeave {
                button: event.button,
                dragged: event.target,
                hit: event.hit.clone(),
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Drop>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Drop>>) -> Self {
        EntityPointerEvent::Drop {
            entity: event.target,
            data: Drop {
                button: event.button,
                dropped: event.target,
                hit: event.hit.clone(),
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Out>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        EntityPointerEvent::Out {
            entity: event.target,
            data: Out {
                hit: event.hit.clone(),
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Click>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        EntityPointerEvent::Click {
            entity: event.target,
            data: Click {
                hit: event.hit.clone(),
                button: event.button,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Down>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        EntityPointerEvent::Down {
            entity: event.target,
            data: Down {
                hit: event.hit.clone(),
                button: event.button,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Up>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Up>>) -> Self {
        EntityPointerEvent::Up {
            entity: event.target,
            data: Up {
                hit: event.hit.clone(),
                button: event.button,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

impl From<ListenerInput<Pointer<Move>>> for EntityPointerEvent {
    fn from(event: ListenerInput<Pointer<Move>>) -> Self {
        EntityPointerEvent::Move {
            entity: event.target,
            data: Move {
                hit: event.hit.clone(),
                delta: event.delta,
            },
            pointer_id: event.pointer_id,
            pointer_position: event.pointer_location.position,
        }
    }
}

#[derive(Event, Debug)]
pub enum TransformEvent {
    Translate((Entity, Vec3)),
    Rotate((Entity, Quat)),
    Scale((Entity, Vec3)),
}
