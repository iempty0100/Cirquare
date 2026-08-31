use smithay::{
    backend::{
        renderer::{
            Color32F, Frame, Renderer,
            element::{
                Kind,
                surface::{WaylandSurfaceRenderElement, render_elements_from_surface_tree},
            },
            gles::GlesRenderer,
            utils::draw_render_elements,
        },
        winit::WinitGraphicsBackend,
    },
    utils::{Rectangle, Transform},
    wayland::compositor::{SurfaceAttributes, TraversalAction, with_surface_tree_downward},
};

use crate::state::State;

// ============================================================
// Rendering
// ============================================================

pub fn render(
    backend: &mut WinitGraphicsBackend<GlesRenderer>,
    state: &mut State,
) -> Result<(), Box<dyn std::error::Error>> {
    // ========================================================
    // Window size
    // ========================================================

    let size = backend.window_size();

    let damage = Rectangle::from_size(size);

    // ========================================================
    // Bind framebuffer
    // ========================================================

    {
        let (renderer, mut framebuffer) = backend.bind()?;

        // ====================================================
        // Surface -> Render Elements
        // ====================================================

        let elements = state
            .xdg_shell_state
            .toplevel_surfaces()
            .iter()
            .flat_map(|surface| {
                let wl_surface = surface.wl_surface();

                let position = state.wm.position(wl_surface).unwrap_or((0, 0).into());

                render_elements_from_surface_tree(
                    renderer,
                    wl_surface,
                    (position.x, position.y),
                    1.0,
                    1.0,
                    Kind::Unspecified,
                )
            })
            .collect::<Vec<WaylandSurfaceRenderElement<GlesRenderer>>>();

        // ====================================================
        // Render frame
        // ====================================================

        let mut frame = renderer.render(&mut framebuffer, size, Transform::Flipped180)?;

        // ====================================================
        // Clear
        // ====================================================

        frame.clear(Color32F::new(0.0, 0.0, 0.0, 1.0), &[damage])?;

        // ====================================================
        // Draw Wayland surfaces
        // ====================================================

        draw_render_elements(&mut frame, 1.0, &elements, &[damage])?;

        // ====================================================
        // Finish rendering
        // ====================================================

        let _ = frame.finish()?;

        // ====================================================
        // Send frame callbacks
        // ====================================================

        let frame_time = state.start_time.elapsed().as_millis() as u32;

        for surface in state.xdg_shell_state.toplevel_surfaces() {
            send_frames_surface_tree(surface.wl_surface(), frame_time);
        }
    }

    // ========================================================
    // Submit frame
    // ========================================================

    backend.submit(Some(&[damage]))?;

    Ok(())
}

// ============================================================
// Frame Callback Handling
// ============================================================

fn send_frames_surface_tree(surface: &wayland_server::protocol::wl_surface::WlSurface, time: u32) {
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |_surface, states, &()| {
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}
