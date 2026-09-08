use smithay::{
    delegate_xdg_decoration,
    wayland::shell::xdg::{
        decoration::XdgDecorationHandler,
        ToplevelSurface,
    },
};

use wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode;

use crate::state::State;

// ============================================================
// XDG Decoration
// ============================================================

impl XdgDecorationHandler for State {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        println!("XDG decoration created: {:?}", toplevel.wl_surface().id());

        self.force_server_side(&toplevel);
    }

    fn request_mode(&mut self, toplevel: ToplevelSurface, mode: Mode) {
        println!(
            "XDG decoration mode requested: {:?} -> {:?}",
            toplevel.wl_surface().id(),
            mode
        );

        // CQDE owns the window frame, so always negotiate
        // server-side decoration regardless of the client's preference.
        self.force_server_side(&toplevel);
    }

    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        println!(
            "XDG decoration mode unset: {:?}",
            toplevel.wl_surface().id()
        );

        self.force_server_side(&toplevel);
    }
}

impl State {
    fn force_server_side(&mut self, toplevel: &ToplevelSurface) {
        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });

        toplevel.send_configure();
    }
}

// ============================================================
// XDG Decoration Delegate
// ============================================================

delegate_xdg_decoration!(State);
