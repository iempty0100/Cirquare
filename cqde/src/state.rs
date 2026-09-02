use std::time::Instant;

use smithay::{
    input::{Seat, SeatState, keyboard::KeyboardHandle},
    utils::{Logical, Point, Serial, Size},
    wayland::{compositor::CompositorState, shell::xdg::XdgShellState, shm::ShmState},
};

use wayland_server::{DisplayHandle, Resource, protocol::wl_surface::WlSurface};

use crate::wm::{ResizeEdge, WindowManager};

pub struct State {
    pub display_handle: DisplayHandle,

    pub compositor_state: CompositorState,
    pub shm_state: ShmState,
    pub xdg_shell_state: XdgShellState,

    pub seat_state: SeatState<Self>,
    pub seat: Seat<Self>,

    // ============================================================
    // Keyboard
    // ============================================================

    // KeyboardHandle is created after State itself is initialized.
    pub keyboard: Option<KeyboardHandle<Self>>,

    // ============================================================
    // Focus
    // ============================================================
    pub focused_surface: Option<WlSurface>,

    // ============================================================
    // Cursor
    // ============================================================
    pub cursor_position: Point<f64, Logical>,

    // ============================================================
    // Window Manager
    // ============================================================
    pub wm: WindowManager,

    // ============================================================
    // Window Dragging
    // ============================================================
    pub dragging: bool,
    pub drag_surface: Option<WlSurface>,
    pub drag_offset: Point<f64, Logical>,

    // ============================================================
    // Window Resizing
    // ============================================================
    pub resizing: bool,
    pub resize_surface: Option<WlSurface>,
    pub resize_edge: Option<ResizeEdge>,

    pub resize_start: Point<f64, Logical>,
    pub resize_initial_position: Point<i32, Logical>,
    pub resize_initial_size: Size<i32, Logical>,

    // ============================================================
    // Frame timing
    // ============================================================
    pub start_time: Instant,
}

impl State {
    // ============================================================
    // Focus Window
    // ============================================================

    pub fn focus_window(&mut self, surface: &WlSurface, serial: Serial) {
        // --------------------------------------------------------
        // Make sure this surface is actually managed.
        // --------------------------------------------------------

        if self.wm.position(surface).is_none() {
            return;
        }

        // --------------------------------------------------------
        // WM focus / Z-order
        // --------------------------------------------------------

        self.wm.focus(surface);

        // --------------------------------------------------------
        // Compositor-side focus
        // --------------------------------------------------------

        self.focused_surface = Some(surface.clone());

        // --------------------------------------------------------
        // Wayland keyboard focus
        // --------------------------------------------------------

        //
        // Clone the handle first because set_focus() needs
        // &mut State at the same time.
        //
        if let Some(keyboard) = self.keyboard.clone() {
            keyboard.set_focus(self, Some(surface.clone()), serial);
        }

        println!("Focus changed: {:?}", surface.id());
    }

    // ============================================================
    // Clear Focus
    // ============================================================

    pub fn clear_focus(&mut self, serial: Serial) {
        self.focused_surface = None;

        if let Some(keyboard) = self.keyboard.clone() {
            keyboard.set_focus(self, None, serial);
        }

        println!("Focus cleared.");
    }
}
