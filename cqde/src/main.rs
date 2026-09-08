mod client;
mod compositor;
mod decoration;
mod input;
mod renderer;
mod shell;
mod state;
mod wayland;
mod wm;

use std::sync::Arc;
use std::time::Duration;

use calloop::EventLoop;

use smithay::{
    backend::{renderer::gles::GlesRenderer, winit},
    input::SeatState,
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        shell::xdg::{XdgShellState, decoration::XdgDecorationState},
        shm::ShmState,
        socket::ListeningSocketSource,
    },
};

use wayland_server::Display;

use crate::{client::ClientState, state::State, wm::WindowManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================");
    println!(" Starting CirQuare Desktop Environment");
    println!("========================================");

    // ============================================================
    // Wayland Display
    // ============================================================

    let mut display = Display::<State>::new()?;
    let display_handle = display.handle();

    println!("Wayland display created.");

    // ============================================================
    // Compositor
    // ============================================================

    let compositor_state = CompositorState::new::<State>(&display_handle);

    println!("wl_compositor initialized.");

    // ============================================================
    // SHM
    // ============================================================

    let shm_state = ShmState::new::<State>(
        &display_handle,
        vec![
            wayland_server::protocol::wl_shm::Format::Argb8888,
            wayland_server::protocol::wl_shm::Format::Xrgb8888,
        ],
    );

    println!("wl_shm initialized.");

    // ============================================================
    // XDG Shell
    // ============================================================

    let xdg_shell_state = XdgShellState::new::<State>(&display_handle);

    println!("xdg_wm_base initialized.");

    // ============================================================
    // XDG Decoration
    // ============================================================

    let xdg_decoration_state = XdgDecorationState::new::<State>(&display_handle);

    println!("zxdg_decoration_manager_v1 initialized.");

    // ============================================================
    // Seat
    // ============================================================

    let mut seat_state = SeatState::<State>::new();

    let seat = seat_state.new_wl_seat(&display_handle, "cqde");

    println!("wl_seat initialized.");

    // ============================================================
    // State
    // ============================================================

    let mut state = State {
        display_handle: display_handle.clone(),

        compositor_state,
        shm_state,
        xdg_shell_state,
        xdg_decoration_state,

        seat_state,
        seat,

        keyboard: None,

        focused_surface: None,

        cursor_position: (0.0, 0.0).into(),

        wm: WindowManager::new(),

        dragging: false,
        drag_surface: None,
        drag_offset: (0.0, 0.0).into(),

        resizing: false,
        resize_surface: None,
        resize_edge: None,

        resize_start: (0.0, 0.0).into(),
        resize_initial_position: (0, 0).into(),
        resize_initial_size: (600, 400).into(),

        start_time: std::time::Instant::now(),
    };

    // ============================================================
    // Keyboard
    // ============================================================

    let keyboard = state
        .seat
        .add_keyboard(Default::default(), 200, 200)
        .expect("Failed to create keyboard");

    state.keyboard = Some(keyboard);

    // ============================================================
    // Pointer
    // ============================================================

    let pointer = state.seat.add_pointer();

    println!("Keyboard initialized.");
    println!("Pointer initialized.");

    // ============================================================
    // Event Loop
    // ============================================================

    let mut event_loop = EventLoop::<State>::try_new()?;

    println!("Event loop initialized.");

    // ============================================================
    // Wayland Socket
    // ============================================================

    let socket = ListeningSocketSource::new_auto()?;

    println!("Wayland display socket: {:?}", socket.socket_name());

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

    // ============================================================
    // Winit Backend
    // ============================================================

    println!("Initializing winit backend...");

    let (mut backend, mut winit) = winit::init::<GlesRenderer>()?;

    println!("Winit backend initialized.");

    // ============================================================
    // Main Loop
    // ============================================================

    loop {
        if !input::process_winit_events(&backend, &mut winit, &mut state, &pointer) {
            return Ok(());
        }

        event_loop.dispatch(Some(Duration::from_millis(1)), &mut state)?;

        display.dispatch_clients(&mut state)?;

        renderer::render(&mut backend, &mut state)?;

        display.flush_clients()?;
    }
}
