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
            .wm
            .windows()
            .rev()
            .flat_map(|window| {
                let wl_surface = &window.surface;

                let position = window.position;

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

        for window in state.wm.windows() {
            send_frames_surface_tree(&window.surface, frame_time);
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
