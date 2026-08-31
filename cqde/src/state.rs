use std::time::Instant;

use smithay::{
    input::{Seat, SeatState},
    utils::{Logical, Point, Size},
    wayland::{compositor::CompositorState, shell::xdg::XdgShellState, shm::ShmState},
};

use wayland_server::{DisplayHandle, protocol::wl_surface::WlSurface};

use crate::wm::{ResizeEdge, WindowManager};

pub struct State {
    pub display_handle: DisplayHandle,

    pub compositor_state: CompositorState,
    pub shm_state: ShmState,
    pub xdg_shell_state: XdgShellState,

    pub seat_state: SeatState<Self>,
    pub seat: Seat<Self>,

    // ========================================================
    // Focus
    // ========================================================
    pub focused_surface: Option<WlSurface>,

    // ========================================================
    // Cursor
    // ========================================================
    pub cursor_position: Point<f64, Logical>,

    // ========================================================
    // Window Manager
    // ========================================================
    pub wm: WindowManager,

    // ========================================================
    // Window Dragging
    // ========================================================
    pub dragging: bool,
    pub drag_surface: Option<WlSurface>,
    pub drag_offset: Point<f64, Logical>,

    // ========================================================
    // Window Resizing
    // ========================================================
    pub resizing: bool,
    pub resize_surface: Option<WlSurface>,
    pub resize_edge: Option<ResizeEdge>,

    pub resize_start: Point<f64, Logical>,

    pub resize_initial_position: Point<i32, Logical>,
    pub resize_initial_size: Size<i32, Logical>,

    // ========================================================
    // Frame timing
    // ========================================================
    pub start_time: Instant,
}
