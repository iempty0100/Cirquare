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
        keyboard::{FilterResult, KeyboardHandle},
        pointer::{ButtonEvent, CursorImageStatus, MotionEvent, PointerHandle},
    },
    utils::{Logical, Point, Serial},
};

use wayland_server::{Resource, protocol::wl_surface::WlSurface};

use crate::{state::State, wm::ResizeEdge};

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
        if let Some(surface) = focused {
            println!("Keyboard/pointer focus: {:?}", surface.id());
        } else {
            println!("Keyboard/pointer focus cleared.");
        }
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, _image: CursorImageStatus) {
        // Cursor rendering will be implemented later.
    }
}

// ============================================================
// Seat Delegate
// ============================================================

smithay::delegate_seat!(State);

// ============================================================
// Winit Event Processing
// ============================================================

pub fn process_winit_events(
    backend: &WinitGraphicsBackend<GlesRenderer>,
    winit: &mut WinitEventLoop,
    state: &mut State,
    keyboard: &KeyboardHandle<State>,
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
                    // ==================================================
                    // Keyboard
                    // ==================================================
                    InputEvent::Keyboard { event } => {
                        let serial = Serial::from(0);

                        keyboard.input::<(), _>(
                            state,
                            event.key_code(),
                            event.state(),
                            serial,
                            event.time() as u32,
                            |_, _, _| FilterResult::Forward,
                        );

                        println!(
                            "Keyboard input: key={:?} state={:?}",
                            event.key_code(),
                            event.state()
                        );
                    }

                    // ==================================================
                    // Mouse Movement
                    // ==================================================
                    InputEvent::PointerMotionAbsolute { event } => {
                        let size = backend.window_size();

                        let x = event.x_transformed(size.w);
                        let y = event.y_transformed(size.h);

                        let location: Point<f64, Logical> = (x, y).into();

                        state.cursor_position = location;

                        // ==================================================
                        // Resize
                        // ==================================================

                        if state.resizing {
                            if let (Some(surface), Some(edge)) =
                                (state.resize_surface.clone(), state.resize_edge)
                            {
                                let dx = location.x - state.resize_start.x;
                                let dy = location.y - state.resize_start.y;

                                let mut position = state.resize_initial_position;
                                let mut new_size = state.resize_initial_size;

                                const MIN_WIDTH: i32 = 200;
                                const MIN_HEIGHT: i32 = 120;

                                match edge {
                                    // ------------------------------------------
                                    // North
                                    // ------------------------------------------
                                    ResizeEdge::North => {
                                        let height = state.resize_initial_size.h - dy as i32;

                                        if height >= MIN_HEIGHT {
                                            position.y =
                                                state.resize_initial_position.y + dy as i32;

                                            new_size.h = height;
                                        }
                                    }

                                    // ------------------------------------------
                                    // South
                                    // ------------------------------------------
                                    ResizeEdge::South => {
                                        new_size.h =
                                            (state.resize_initial_size.h as f64 + dy) as i32;
                                    }

                                    // ------------------------------------------
                                    // East
                                    // ------------------------------------------
                                    ResizeEdge::East => {
                                        new_size.w =
                                            (state.resize_initial_size.w as f64 + dx) as i32;
                                    }

                                    // ------------------------------------------
                                    // West
                                    // ------------------------------------------
                                    ResizeEdge::West => {
                                        let width = state.resize_initial_size.w - dx as i32;

                                        if width >= MIN_WIDTH {
                                            position.x =
                                                state.resize_initial_position.x + dx as i32;

                                            new_size.w = width;
                                        }
                                    }

                                    // ------------------------------------------
                                    // North East
                                    // ------------------------------------------
                                    ResizeEdge::NorthEast => {
                                        let height = state.resize_initial_size.h - dy as i32;

                                        if height >= MIN_HEIGHT {
                                            position.y =
                                                state.resize_initial_position.y + dy as i32;

                                            new_size.h = height;
                                        }

                                        new_size.w =
                                            (state.resize_initial_size.w as f64 + dx) as i32;
                                    }

                                    // ------------------------------------------
                                    // North West
                                    // ------------------------------------------
                                    ResizeEdge::NorthWest => {
                                        let width = state.resize_initial_size.w - dx as i32;

                                        let height = state.resize_initial_size.h - dy as i32;

                                        if width >= MIN_WIDTH {
                                            position.x =
                                                state.resize_initial_position.x + dx as i32;

                                            new_size.w = width;
                                        }

                                        if height >= MIN_HEIGHT {
                                            position.y =
                                                state.resize_initial_position.y + dy as i32;

                                            new_size.h = height;
                                        }
                                    }

                                    // ------------------------------------------
                                    // South East
                                    // ------------------------------------------
                                    ResizeEdge::SouthEast => {
                                        new_size.w =
                                            (state.resize_initial_size.w as f64 + dx) as i32;

                                        new_size.h =
                                            (state.resize_initial_size.h as f64 + dy) as i32;
                                    }

                                    // ------------------------------------------
                                    // South West
                                    // ------------------------------------------
                                    ResizeEdge::SouthWest => {
                                        let width = state.resize_initial_size.w - dx as i32;

                                        if width >= MIN_WIDTH {
                                            position.x =
                                                state.resize_initial_position.x + dx as i32;

                                            new_size.w = width;
                                        }

                                        new_size.h =
                                            (state.resize_initial_size.h as f64 + dy) as i32;
                                    }
                                }

                                new_size.w = new_size.w.max(MIN_WIDTH);
                                new_size.h = new_size.h.max(MIN_HEIGHT);

                                state.wm.set_position(&surface, position);
                                state.wm.set_size(&surface, new_size);
                            }
                        }
                        // ==================================================
                        // Drag
                        // ==================================================
                        else if state.dragging {
                            if let Some(surface) = state.drag_surface.clone() {
                                let new_position = (
                                    (location.x - state.drag_offset.x) as i32,
                                    (location.y - state.drag_offset.y) as i32,
                                )
                                    .into();

                                state.wm.set_position(&surface, new_position);
                            }
                        }

                        // ==================================================
                        // Normal pointer focus
                        // ==================================================

                        let focus = state
                            .wm
                            .window_at(location)
                            .map(|surface| (surface, location));

                        let motion_event = MotionEvent {
                            location,
                            serial: Serial::from(0),
                            time: event.time() as u32,
                        };

                        pointer.motion(state, focus, &motion_event);
                    }

                    // ==================================================
                    // Mouse Button
                    // ==================================================
                    InputEvent::PointerButton { event } => {
                        let serial = Serial::from(0);
                        let location = state.cursor_position;

                        // ==================================================
                        // LEFT BUTTON
                        // ==================================================

                        if event.button_code() == 0x110 {
                            // ==================================================
                            // Pressed
                            // ==================================================

                            if event.state() == ButtonState::Pressed {
                                // ------------------------------------------
                                // Find window
                                // ------------------------------------------

                                if let Some(surface) = state.wm.window_at(location) {
                                    // ------------------------------------------
                                    // Focus
                                    // ------------------------------------------

                                    state.wm.focus(&surface);
                                    state.focused_surface = Some(surface.clone());

                                    println!("Focus changed: {:?}", surface.id());

                                    // ------------------------------------------
                                    // Check resize edge
                                    // ------------------------------------------

                                    if let Some(edge) = state.wm.resize_edge(&surface, location) {
                                        if let (Some(position), Some(size)) =
                                            (state.wm.position(&surface), state.wm.size(&surface))
                                        {
                                            state.resizing = true;
                                            state.resize_surface = Some(surface.clone());

                                            state.resize_edge = Some(edge);

                                            state.resize_start = location;

                                            state.resize_initial_position = position;

                                            state.resize_initial_size = size;

                                            // Disable dragging
                                            state.dragging = false;
                                            state.drag_surface = None;

                                            println!(
                                                "WM: resize start {:?}, edge={:?}",
                                                surface.id(),
                                                edge
                                            );
                                        }
                                    }
                                    // ------------------------------------------
                                    // Otherwise start dragging
                                    // ------------------------------------------
                                    else if let Some(position) = state.wm.position(&surface) {
                                        state.dragging = true;

                                        state.drag_surface = Some(surface.clone());

                                        state.drag_offset = (
                                            location.x - position.x as f64,
                                            location.y - position.y as f64,
                                        )
                                            .into();

                                        println!(
                                            "WM: drag start {:?}, offset=({}, {})",
                                            surface.id(),
                                            state.drag_offset.x,
                                            state.drag_offset.y
                                        );
                                    }
                                }
                            }
                            // ==================================================
                            // Released
                            // ==================================================
                            else if event.state() == ButtonState::Released {
                                // ------------------------------------------
                                // End resize
                                // ------------------------------------------

                                if state.resizing {
                                    state.resizing = false;

                                    state.resize_surface = None;
                                    state.resize_edge = None;

                                    println!("WM: resize end");
                                }

                                // ------------------------------------------
                                // End drag
                                // ------------------------------------------

                                if state.dragging {
                                    state.dragging = false;
                                    state.drag_surface = None;

                                    println!("WM: drag end");
                                }
                            }
                        }

                        // ==================================================
                        // Wayland Pointer Event
                        // ==================================================

                        let button_event = ButtonEvent {
                            serial,
                            time: event.time() as u32,
                            button: event.button_code(),
                            state: event.state(),
                        };

                        pointer.button(state, &button_event);

                        println!(
                            "Mouse button: button={} state={:?} x={} y={}",
                            event.button_code(),
                            event.state(),
                            location.x,
                            location.y,
                        );
                    }

                    // ==================================================
                    // Other Input
                    // ==================================================
                    _ => {}
                }
            }

            // ====================================================
            // Other Winit Events
            // ====================================================
            _ => {}
        }
    });

    match status {
        ::winit::platform::pump_events::PumpStatus::Continue => true,

        ::winit::platform::pump_events::PumpStatus::Exit(_) => false,
    }
}
