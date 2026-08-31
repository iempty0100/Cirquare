use smithay::{
    utils::{Logical, Point, Rectangle, Size},
    wayland::shell::xdg::ToplevelSurface,
};

use wayland_server::{Resource, protocol::wl_surface::WlSurface};

#[derive(Debug, Clone)]
pub struct Window {
    pub toplevel: ToplevelSurface,
    pub surface: WlSurface,

    // Position of the wl_surface in compositor space.
    pub position: Point<i32, Logical>,

    // Size requested by the compositor through XDG configure.
    pub size: Size<i32, Logical>,

    // Client-reported window geometry.
    //
    // This corresponds to:
    // xdg_surface.set_window_geometry(x, y, width, height)
    pub geometry_offset: Point<i32, Logical>,
    pub geometry_size: Size<i32, Logical>,
}

pub struct WindowManager {
    windows: Vec<Window>,
    focused: Option<WlSurface>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeEdge {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused: None,
        }
    }

    // ========================================================
    // Add window
    // ========================================================

    pub fn add_window(&mut self, surface: ToplevelSurface) {
        let wl_surface = surface.wl_surface().clone();

        let index = self.windows.len() as i32;

        let position = Point::from((50 + index * 30, 50 + index * 30));

        let size = Size::from((600, 400));

        // Before the client sends its first geometry,
        // use the whole toplevel area as a temporary fallback.
        let geometry_offset = Point::from((0, 0));
        let geometry_size = size;

        println!(
            "WM: adding window {:?} at ({}, {}) size={}x{}",
            wl_surface.id(),
            position.x,
            position.y,
            size.w,
            size.h,
        );

        self.windows.push(Window {
            toplevel: surface,
            surface: wl_surface.clone(),
            position,
            size,
            geometry_offset,
            geometry_size,
        });

        self.focus(&wl_surface);
    }

    // ========================================================
    // Remove window
    // ========================================================

    pub fn remove_window(&mut self, surface: &WlSurface) {
        self.windows.retain(|window| window.surface != *surface);

        if self.focused.as_ref() == Some(surface) {
            self.focused = self.windows.last().map(|window| window.surface.clone());
        }

        println!("WM: removed window {:?}", surface.id());
    }

    // ========================================================
    // Focus
    // ========================================================

    pub fn focus(&mut self, surface: &WlSurface) {
        if let Some(index) = self
            .windows
            .iter()
            .position(|window| window.surface == *surface)
        {
            let window = self.windows.remove(index);

            self.windows.push(window);

            self.focused = Some(surface.clone());

            println!("WM: focus -> {:?}", surface.id());
        }
    }

    pub fn focused(&self) -> Option<&WlSurface> {
        self.focused.as_ref()
    }

    // ========================================================
    // Hit Test
    // ========================================================

    pub fn window_at(&self, location: Point<f64, Logical>) -> Option<WlSurface> {
        self.windows
            .iter()
            .rev()
            .find(|window| {
                let geometry_position = Point::from((
                    window.position.x + window.geometry_offset.x,
                    window.position.y + window.geometry_offset.y,
                ));

                let geometry = Rectangle::new(geometry_position, window.geometry_size);

                geometry.contains(location.to_i32_round())
            })
            .map(|window| window.surface.clone())
    }

    // ========================================================
    // Position
    // ========================================================

    pub fn position(&self, surface: &WlSurface) -> Option<Point<i32, Logical>> {
        self.windows
            .iter()
            .find(|window| window.surface == *surface)
            .map(|window| window.position)
    }

    pub fn set_position(&mut self, surface: &WlSurface, position: Point<i32, Logical>) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.surface == *surface)
        {
            window.position = position;

            println!(
                "WM: window {:?} -> ({}, {})",
                surface.id(),
                position.x,
                position.y
            );
        }
    }

    // ========================================================
    // Size
    // ========================================================

    pub fn size(&self, surface: &WlSurface) -> Option<Size<i32, Logical>> {
        self.windows
            .iter()
            .find(|window| window.surface == *surface)
            .map(|window| window.size)
    }

    pub fn set_size(&mut self, surface: &WlSurface, size: Size<i32, Logical>) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.surface == *surface)
        {
            // WM's requested size
            window.size = size;

            // Do NOT modify geometry_size here.
            //
            // geometry_size belongs to the client and will be
            // updated after the client commits its new geometry.

            window.toplevel.with_pending_state(|state| {
                state.size = Some(size);
            });

            window.toplevel.send_configure();

            println!(
                "WM: window {:?} resized -> {}x{}",
                surface.id(),
                size.w,
                size.h
            );
        }
    }

    // ========================================================
    // Client Geometry
    // ========================================================

    pub fn set_geometry(
        &mut self,
        surface: &WlSurface,
        offset: Point<i32, Logical>,
        size: Size<i32, Logical>,
    ) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.surface == *surface)
        {
            window.geometry_offset = offset;
            window.geometry_size = size;

            println!(
                "WM: geometry {:?} -> offset=({}, {}) size={}x{}",
                surface.id(),
                offset.x,
                offset.y,
                size.w,
                size.h,
            );
        }
    }

    pub fn geometry_offset(&self, surface: &WlSurface) -> Option<Point<i32, Logical>> {
        self.windows
            .iter()
            .find(|window| window.surface == *surface)
            .map(|window| window.geometry_offset)
    }

    pub fn geometry_size(&self, surface: &WlSurface) -> Option<Size<i32, Logical>> {
        self.windows
            .iter()
            .find(|window| window.surface == *surface)
            .map(|window| window.geometry_size)
    }

    // ========================================================
    // Resize Edge
    // ========================================================

    pub fn resize_edge(
        &self,
        surface: &WlSurface,
        location: Point<f64, Logical>,
    ) -> Option<ResizeEdge> {
        let window = self
            .windows
            .iter()
            .find(|window| window.surface == *surface)?;

        let left = (window.position.x + window.geometry_offset.x) as f64;

        let top = (window.position.y + window.geometry_offset.y) as f64;

        let right = left + window.geometry_size.w as f64;

        let bottom = top + window.geometry_size.h as f64;

        let x = location.x;
        let y = location.y;

        const BORDER: f64 = 8.0;

        let near_left = x >= left - BORDER && x <= left + BORDER;

        let near_right = x >= right - BORDER && x <= right + BORDER;

        let near_top = y >= top - BORDER && y <= top + BORDER;

        let near_bottom = y >= bottom - BORDER && y <= bottom + BORDER;

        match (near_left, near_right, near_top, near_bottom) {
            // Corners
            (true, false, true, false) => Some(ResizeEdge::NorthWest),

            (false, true, true, false) => Some(ResizeEdge::NorthEast),

            (true, false, false, true) => Some(ResizeEdge::SouthWest),

            (false, true, false, true) => Some(ResizeEdge::SouthEast),

            // Edges
            (false, false, true, false) => Some(ResizeEdge::North),

            (false, false, false, true) => Some(ResizeEdge::South),

            (true, false, false, false) => Some(ResizeEdge::West),

            (false, true, false, false) => Some(ResizeEdge::East),

            _ => None,
        }
    }

    // ========================================================
    // Windows
    // ========================================================

    pub fn windows(&self) -> impl Iterator<Item = &Window> {
        self.windows.iter()
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}
