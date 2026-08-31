use smithay::{
    delegate_shm,
    wayland::{
        buffer::BufferHandler,
        shm::{ShmHandler, ShmState},
    },
};

use wayland_server::{Resource, protocol::wl_buffer::WlBuffer};

use crate::state::State;

// ============================================================
// SHM Handler
// ============================================================

impl ShmHandler for State {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

// ============================================================
// Buffer Handler
// ============================================================

impl BufferHandler for State {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {
        println!("SHM buffer destroyed: {:?}", buffer.id());
    }
}

// ============================================================
// SHM Delegate
// ============================================================

delegate_shm!(State);
