use std::sync::Arc;

use ::winit::platform::pump_events::PumpStatus;
use calloop::EventLoop;

use smithay::{
    backend::{
        renderer::{
            Color32F, Frame, Renderer,
            element::{
                Kind,
                surface::{WaylandSurfaceRenderElement, render_elements_from_surface_tree},
            },
            gles::GlesRenderer,
            utils::{draw_render_elements, on_commit_buffer_handler},
        },
        winit::{self, WinitEvent},
    },
    delegate_compositor, delegate_seat, delegate_shm, delegate_xdg_shell,
    input::{Seat, SeatHandler, SeatState},
    utils::{Rectangle, Transform},
    wayland::{
        buffer::BufferHandler,
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
        shm::{ShmHandler, ShmState},
        socket::ListeningSocketSource,
    },
};

use wayland_protocols::xdg::shell::server::xdg_toplevel;

use wayland_server::{
    Client, Display, DisplayHandle, Resource,
    backend::ClientData,
    protocol::{wl_buffer::WlBuffer, wl_seat::WlSeat, wl_surface::WlSurface},
};

use smithay::utils::Serial;

// ============================================================
// Client state
// ============================================================

struct ClientState {
    compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}

// ============================================================
// CQDE state
// ============================================================

struct State {
    display_handle: DisplayHandle,

    compositor_state: CompositorState,

    shm_state: ShmState,

    xdg_shell_state: XdgShellState,

    seat_state: SeatState<Self>,

    seat: Seat<Self>,
}

// ============================================================
// Compositor
// ============================================================

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

        println!("Surface committed: {:?}", surface.id());
    }
}

// ============================================================
// SHM
// ============================================================

impl ShmHandler for State {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

// ============================================================
// Buffer
// ============================================================

impl BufferHandler for State {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {
        println!("SHM buffer destroyed: {:?}", buffer.id());
    }
}

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

        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Activated);
        });

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
// Seat
// ============================================================

impl SeatHandler for State {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
}

// ============================================================
// Smithay delegates
// ============================================================

delegate_compositor!(State);
delegate_shm!(State);
delegate_xdg_shell!(State);
delegate_seat!(State);

// ============================================================
// Main
// ============================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================");
    println!(" Starting CirQuare Desktop Environment");
    println!("========================================");

    // --------------------------------------------------------
    // Wayland display
    // --------------------------------------------------------

    let mut display = Display::<State>::new()?;

    let display_handle = display.handle();

    println!("Wayland display created.");

    // --------------------------------------------------------
    // Compositor
    // --------------------------------------------------------

    let compositor_state = CompositorState::new::<State>(&display_handle);

    println!("wl_compositor initialized.");

    // --------------------------------------------------------
    // SHM
    // --------------------------------------------------------

    let shm_state = ShmState::new::<State>(
        &display_handle,
        vec![
            wayland_server::protocol::wl_shm::Format::Argb8888,
            wayland_server::protocol::wl_shm::Format::Xrgb8888,
        ],
    );

    println!("wl_shm initialized.");

    // --------------------------------------------------------
    // XDG Shell
    // --------------------------------------------------------

    let xdg_shell_state = XdgShellState::new::<State>(&display_handle);

    let mut seat_state = SeatState::new();

    let seat = seat_state.new_wl_seat(&display_handle, "cqde");

    println!("xdg_wm_base initialized.");

    // --------------------------------------------------------
    // State
    // --------------------------------------------------------

    let mut state = State {
        display_handle,
        compositor_state,
        shm_state,
        xdg_shell_state,
        seat_state,
        seat,
    };

    // --------------------------------------------------------
    // Event loop
    // --------------------------------------------------------

    let mut event_loop = EventLoop::<State>::try_new()?;

    println!("Event loop initialized.");

    // --------------------------------------------------------
    // Wayland socket
    // --------------------------------------------------------

    let socket = ListeningSocketSource::new_auto()?;

    println!("Wayland socket: {:?}", socket.socket_name());

    // --------------------------------------------------------
    // Client connections
    // --------------------------------------------------------

    event_loop
        .handle()
        .insert_source(socket, |client_stream, _, state| {
            println!("----------------------------------------");
            println!("New Wayland client connected.");

            let client_data = ClientState {
                compositor_state: CompositorClientState::default(),
            };

            state
                .display_handle
                .insert_client(client_stream, Arc::new(client_data))
                .expect("Failed to insert Wayland client");

            println!("Client registered.");
        })?;

    // --------------------------------------------------------
    // Winit renderer/backend
    // --------------------------------------------------------

    println!("Initializing winit backend...");

    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;

    println!("Winit backend initialized.");

    // --------------------------------------------------------
    // Main loop
    // --------------------------------------------------------

    loop {
        // ----------------------------------------------------
        // Winit events
        // ----------------------------------------------------

        let status = winit.dispatch_new_events(|event| match event {
            WinitEvent::Resized { .. } => {}

            WinitEvent::Input(_) => {}

            _ => {}
        });

        match status {
            PumpStatus::Continue => {}

            PumpStatus::Exit(_) => {
                return Ok(());
            }
        }

        // ----------------------------------------------------
        // Wayland clients
        // ----------------------------------------------------

        event_loop.dispatch(Some(std::time::Duration::from_millis(1)), &mut state)?;

        display.dispatch_clients(&mut state)?;

        // ----------------------------------------------------
        // Rendering
        // ----------------------------------------------------

        let size = backend.window_size();

        let damage = Rectangle::from_size(size);

        {
            let (renderer, mut framebuffer) = backend.bind().unwrap();

            // ------------------------------------------------
            // Surface → Render Elements
            // ------------------------------------------------

            let elements = state
                .xdg_shell_state
                .toplevel_surfaces()
                .iter()
                .flat_map(|surface| {
                    render_elements_from_surface_tree(
                        renderer,
                        surface.wl_surface(),
                        (0, 0),
                        1.0,
                        1.0,
                        Kind::Unspecified,
                    )
                })
                .collect::<Vec<WaylandSurfaceRenderElement<GlesRenderer>>>();

            // ------------------------------------------------
            // Render frame
            // ------------------------------------------------

            let mut frame = renderer
                .render(&mut framebuffer, size, Transform::Flipped180)
                .unwrap();

            // ------------------------------------------------
            // Clear
            // ------------------------------------------------

            frame
                .clear(Color32F::new(0.0, 0.0, 0.0, 1.0), &[damage])
                .unwrap();

            // ------------------------------------------------
            // Draw Wayland surfaces
            // ------------------------------------------------

            draw_render_elements(&mut frame, 1.0, &elements, &[damage]).unwrap();

            // ------------------------------------------------
            // Finish frame
            // ------------------------------------------------

            let _ = frame.finish().unwrap();
        }

        // ----------------------------------------------------
        // Submit frame
        // ----------------------------------------------------

        backend.submit(Some(&[damage])).unwrap();

        // ----------------------------------------------------
        // Flush Wayland clients
        // ----------------------------------------------------

        display.flush_clients()?;
    }
}
