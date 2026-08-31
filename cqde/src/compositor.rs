use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor,
    wayland::{
        compositor::{self, CompositorClientState, CompositorHandler, CompositorState},
        shell::xdg::SurfaceCachedState,
    },
};

use wayland_server::{Client, Resource, protocol::wl_surface::WlSurface};

use crate::{client::ClientState, state::State};

impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .expect("ClientState missing")
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);

        let geometry = compositor::with_states(surface, |states| {
            states
                .cached_state
                .get::<SurfaceCachedState>()
                .current()
                .geometry
        });

        if let Some(geometry) = geometry {
            self.wm.set_geometry(surface, geometry.loc, geometry.size);

            println!(
                "Surface geometry {:?} -> x={} y={} w={} h={}",
                surface.id(),
                geometry.loc.x,
                geometry.loc.y,
                geometry.size.w,
                geometry.size.h,
            );
        }

        println!("Surface committed: {:?}", surface.id());
    }
}

delegate_compositor!(State);
