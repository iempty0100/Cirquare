use smithay::wayland::compositor::CompositorClientState;
use wayland_server::backend::ClientData;

pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}
