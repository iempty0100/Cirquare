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

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        println!(
            "New toplevel surface created: {:?}",
            surface.wl_surface().id()
        );

        // ========================================================
        // Register window with Window Manager
        // ========================================================

        self.wm.add_window(surface.clone());

        // ========================================================
        // Initial window state
        // ========================================================

        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Activated);

            // Initial size proposed by CQDE.
            state.size = Some((600, 400).into());
        });

        // ========================================================
        // Initial configure
        // ========================================================

        surface.send_configure();

        println!("Configure sent.");
    }

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {
        println!("New popup surface created: {:?}", surface.wl_surface().id());
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        println!("Popup grab requested.");
    }

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
