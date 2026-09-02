use smithay::{
    backend::{
        input::{
            AbsolutePositionEvent, ButtonState, Event, InputEvent, KeyboardKeyEvent,
            PointerButtonEvent,
        },
        renderer::gles::GlesRenderer,
        winit::{WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    input::{
        Seat, SeatHandler, SeatState,
        keyboard::FilterResult,
        pointer::{ButtonEvent, CursorImageStatus, MotionEvent, PointerHandle},
    },
    utils::{Logical, Point, Serial},
};

use wayland_server::{Resource, protocol::wl_surface::WlSurface};

use crate::{state::State, wm::ResizeEdge};

// ============================================================
// Constants
// ============================================================

const LEFT_BUTTON: u32 = 0x110;

// ============================================================
// Seat Handler
// ============================================================

impl SeatHandler for State {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, focused: Option<&WlSurface>) {
        match focused {
            Some(surface) => {
                println!("Keyboard/pointer focus: {:?}", surface.id());
            }

            None => {
                println!("Keyboard/pointer focus cleared.");
            }
        }
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {
        // Cursor rendering will be implemented later.
    }
}

smithay::delegate_seat!(State);

// ============================================================
// Resize
// ============================================================

fn handle_resize(state: &mut State, location: Point<f64, Logical>) {
    if !state.resizing {
        return;
    }

    let (Some(surface), Some(edge)) = (state.resize_surface.clone(), state.resize_edge) else {
        return;
    };

    let dx = location.x - state.resize_start.x;
    let dy = location.y - state.resize_start.y;

    let mut position = state.resize_initial_position;
    let mut new_size = state.resize_initial_size;

    const MIN_WIDTH: i32 = 200;
    const MIN_HEIGHT: i32 = 120;

    match edge {
        ResizeEdge::North => {
            let height = state.resize_initial_size.h - dy as i32;

            if height >= MIN_HEIGHT {
                position.y = state.resize_initial_position.y + dy as i32;

                new_size.h = height;
            }
        }

        ResizeEdge::South => {
            new_size.h = (state.resize_initial_size.h as f64 + dy) as i32;
        }

        ResizeEdge::East => {
            new_size.w = (state.resize_initial_size.w as f64 + dx) as i32;
        }

        ResizeEdge::West => {
            let width = state.resize_initial_size.w - dx as i32;

            if width >= MIN_WIDTH {
                position.x = state.resize_initial_position.x + dx as i32;

                new_size.w = width;
            }
        }

        ResizeEdge::NorthEast => {
            let height = state.resize_initial_size.h - dy as i32;

            if height >= MIN_HEIGHT {
                position.y = state.resize_initial_position.y + dy as i32;

                new_size.h = height;
            }

            new_size.w = (state.resize_initial_size.w as f64 + dx) as i32;
        }

        ResizeEdge::NorthWest => {
            let width = state.resize_initial_size.w - dx as i32;

            let height = state.resize_initial_size.h - dy as i32;

            if width >= MIN_WIDTH {
                position.x = state.resize_initial_position.x + dx as i32;

                new_size.w = width;
            }

            if height >= MIN_HEIGHT {
                position.y = state.resize_initial_position.y + dy as i32;

                new_size.h = height;
            }
        }

        ResizeEdge::SouthEast => {
            new_size.w = (state.resize_initial_size.w as f64 + dx) as i32;

            new_size.h = (state.resize_initial_size.h as f64 + dy) as i32;
        }

        ResizeEdge::SouthWest => {
            let width = state.resize_initial_size.w - dx as i32;

            if width >= MIN_WIDTH {
                position.x = state.resize_initial_position.x + dx as i32;

                new_size.w = width;
            }

            new_size.h = (state.resize_initial_size.h as f64 + dy) as i32;
        }
    }

    new_size.w = new_size.w.max(MIN_WIDTH);
    new_size.h = new_size.h.max(MIN_HEIGHT);

    state.wm.set_position(&surface, position);
    state.wm.set_size(&surface, new_size);
}

// ============================================================
// Drag
// ============================================================

fn handle_drag(state: &mut State, location: Point<f64, Logical>) {
    if !state.dragging {
        return;
    }

    let Some(surface) = state.drag_surface.clone() else {
        return;
    };

    let new_position = (
        (location.x - state.drag_offset.x) as i32,
        (location.y - state.drag_offset.y) as i32,
    )
        .into();

    state.wm.set_position(&surface, new_position);
}

// ============================================================
// Pointer Motion
// ============================================================

fn handle_pointer_motion(
    state: &mut State,
    pointer: &PointerHandle<State>,
    location: Point<f64, Logical>,
    time: u32,
) {
    state.cursor_position = location;

    // An active resize takes priority.
    if state.resizing {
        handle_resize(state, location);
    }
    // Otherwise, an active drag is processed.
    else if state.dragging {
        handle_drag(state, location);
    }

    // Pointer focus is determined from the current window
    // under the cursor. This does NOT start dragging/resizing.
    let focus = state
        .wm
        .window_at(location)
        .map(|surface| (surface, location));

    let motion_event = MotionEvent {
        location,
        serial: Serial::from(0),
        time,
    };

    pointer.motion(state, focus, &motion_event);
}

// ============================================================
// Start Resize
// ============================================================

fn start_resize(
    state: &mut State,
    surface: &WlSurface,
    edge: ResizeEdge,
    location: Point<f64, Logical>,
) {
    let (Some(position), Some(size)) = (state.wm.position(surface), state.wm.size(surface)) else {
        return;
    };

    state.resizing = true;
    state.resize_surface = Some(surface.clone());
    state.resize_edge = Some(edge);

    state.resize_start = location;
    state.resize_initial_position = position;
    state.resize_initial_size = size;

    state.dragging = false;
    state.drag_surface = None;

    println!("WM: resize start {:?}, edge={:?}", surface.id(), edge);
}

// ============================================================
// Start Drag
// ============================================================

fn start_drag(state: &mut State, surface: &WlSurface, location: Point<f64, Logical>) {
    let Some(position) = state.wm.position(surface) else {
        return;
    };

    state.dragging = true;
    state.drag_surface = Some(surface.clone());

    state.drag_offset = (
        location.x - position.x as f64,
        location.y - position.y as f64,
    )
        .into();

    state.resizing = false;
    state.resize_surface = None;
    state.resize_edge = None;

    println!(
        "WM: drag start {:?}, offset=({}, {})",
        surface.id(),
        state.drag_offset.x,
        state.drag_offset.y
    );
}

// ============================================================
// Stop Pointer Action
// ============================================================

fn stop_pointer_action(state: &mut State) {
    if state.resizing {
        state.resizing = false;
        state.resize_surface = None;
        state.resize_edge = None;

        println!("WM: resize end");
    }

    if state.dragging {
        state.dragging = false;
        state.drag_surface = None;

        println!("WM: drag end");
    }
}

// ============================================================
// Mouse Button
// ============================================================

fn handle_pointer_button<B: smithay::backend::input::InputBackend>(
    state: &mut State,
    pointer: &PointerHandle<State>,
    event: &dyn PointerButtonEvent<B>,
) {
    let serial = Serial::from(0);
    let location = state.cursor_position;

    let button = event.button_code();
    let button_state = event.state();
    let time = event.time() as u32;

    // --------------------------------------------------------
    // Left button
    // --------------------------------------------------------

    if button == LEFT_BUTTON {
        match button_state {
            ButtonState::Pressed => {
                if let Some(surface) = state.wm.window_at(location) {
                    // Always focus the clicked window.
                    state.focus_window(&surface, serial);

                    // Resize has priority over drag.
                    if let Some(edge) = state.wm.resize_edge(&surface, location) {
                        start_resize(state, &surface, edge, location);
                    }
                    // Drag is allowed only from the titlebar.
                    else if state.wm.is_titlebar(&surface, location) {
                        start_drag(state, &surface, location);
                    }
                    // Client area:
                    // focus only, no drag.
                }
            }

            ButtonState::Released => {
                stop_pointer_action(state);
            }

            _ => {}
        }
    }

    // --------------------------------------------------------
    // Wayland pointer button event
    // --------------------------------------------------------

    let button_event = ButtonEvent {
        serial,
        time,
        button,
        state: button_state,
    };

    pointer.button(state, &button_event);

    println!(
        "Mouse button: button={} state={:?} x={} y={}",
        button, button_state, location.x, location.y,
    );
}

// ============================================================
// Winit Event Processing
// ============================================================

pub fn process_winit_events(
    backend: &WinitGraphicsBackend<GlesRenderer>,
    winit: &mut WinitEventLoop,
    state: &mut State,
    pointer: &PointerHandle<State>,
) -> bool {
    let status = winit.dispatch_new_events(|event| {
        match event {
            // ====================================================
            // Window Resize
            // ====================================================
            WinitEvent::Resized { size, .. } => {
                println!("Window resized: {:?}", size);
            }

            // ====================================================
            // Input
            // ====================================================
            WinitEvent::Input(event) => {
                match event {
                    // ------------------------------------------------
                    // Keyboard
                    // ------------------------------------------------
                    InputEvent::Keyboard { event } => {
                        let serial = Serial::from(0);

                        if let Some(keyboard) = state.keyboard.clone() {
                            keyboard.input::<(), _>(
                                state,
                                event.key_code(),
                                event.state(),
                                serial,
                                event.time() as u32,
                                |_, _, _| FilterResult::Forward,
                            );
                        }

                        println!(
                            "Keyboard input: key={:?} state={:?}",
                            event.key_code(),
                            event.state()
                        );
                    }

                    // ------------------------------------------------
                    // Mouse motion
                    // ------------------------------------------------
                    InputEvent::PointerMotionAbsolute { event } => {
                        let size = backend.window_size();

                        let x = event.x_transformed(size.w);
                        let y = event.y_transformed(size.h);

                        let location = Point::<f64, Logical>::from((x, y));

                        handle_pointer_motion(state, pointer, location, event.time() as u32);
                    }

                    // ------------------------------------------------
                    // Mouse button
                    // ------------------------------------------------
                    InputEvent::PointerButton { event } => {
                        handle_pointer_button(state, pointer, &event);
                    }

                    // ------------------------------------------------
                    // Other input
                    // ------------------------------------------------
                    _ => {}
                }
            }

            // ====================================================
            // Other Winit events
            // ====================================================
            _ => {}
        }
    });

    match status {
        ::winit::platform::pump_events::PumpStatus::Continue => true,

        ::winit::platform::pump_events::PumpStatus::Exit(_) => false,
    }
}
