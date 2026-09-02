use crate::state::State;

use smithay::{
    delegate_xdg_shell,
    utils::Serial,
    wayland::shell::xdg::{
        PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
    },
};

use wayland_server::{Resource, protocol::wl_seat::WlSeat};

use wayland_protocols::xdg::shell::server::xdg_toplevel;

// ============================================================
// XDG Shell
// ============================================================

impl XdgShellHandler for State {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    // ========================================================
    // New Toplevel
    // ========================================================

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let wl_surface = surface.wl_surface().clone();

        println!("New toplevel surface created: {:?}", wl_surface.id());

        // ====================================================
        // Register window
        // ====================================================

        self.wm.add_window(surface.clone());

        // ====================================================
        // Initial window state
        // ====================================================

        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Activated);

            state.size = Some((600, 400).into());
        });

        // ====================================================
        // Initial configure
        // ====================================================

        surface.send_configure();

        println!("Configure sent.");

        // ====================================================
        // Keyboard focus
        // ====================================================

        self.focus_window(&wl_surface, Serial::from(0));
    }

    // ========================================================
    // Popup
    // ========================================================

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {
        println!("New popup surface created: {:?}", surface.wl_surface().id());
    }

    // ========================================================
    // Popup Grab
    // ========================================================

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        println!("Popup grab requested.");
    }

    // ========================================================
    // Popup Reposition
    // ========================================================

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        token: u32,
    ) {
        println!("Popup reposition requested. token={}", token);
    }
}

// ============================================================
// XDG Shell Delegate
// ============================================================

delegate_xdg_shell!(State);
