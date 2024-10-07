use crate::multi_window::Application;
use crate::{Appearance, DefaultStyle};
use iced_core::{mouse as IcedMouse, Color, Point, Size};
use iced_graphics::Viewport;
use layershellev::keyboard::ModifiersState;

use crate::event::WindowEvent;
use iced::window;

pub struct State<A: Application>
where
    A::Theme: DefaultStyle,
{
    id: window::Id,
    scale_factor: f64,
    viewport: Viewport,
    wp_viewport: Option<layershellev::WpViewport>,
    viewport_version: usize,
    theme: A::Theme,
    appearance: Appearance,
    mouse_position: Option<Point>,
    modifiers: ModifiersState,
}

impl<A: Application> State<A>
where
    A::Theme: DefaultStyle,
{
    pub fn new(
        id: window::Id,
        application: &A,
        size: (u32, u32),
        wp_viewport: Option<layershellev::WpViewport>,
    ) -> Self {
        let scale_factor = application.scale_factor(id);
        let theme = application.theme();
        let appearance = application.style(&theme);

        let viewport = {
            let (width, height) = size;
            println!("width: {}, height: {}", width, height);
            println!("scale_factor: {}", scale_factor);
            let viewport = Viewport::with_physical_size(
                iced_core::Size::new(width * 2, height * 2),
                2.
            );
            if let Some(wp_viewport) = wp_viewport.as_ref() {
                wp_viewport.set_destination((width) as i32, (height) as i32);
            }

            viewport
        };

        Self {
            id,
            scale_factor,
            viewport,
            wp_viewport,
            viewport_version: 0,
            theme,
            appearance,
            mouse_position: None,
            modifiers: ModifiersState::default(),
        }
    }
    pub fn modifiers(&self) -> ModifiersState {
        self.modifiers
    }
    pub fn update_view_port(&mut self, width: u32, height: u32) {
        println!("update_view_port: width: {}, height: {}", width, height);
        self.viewport = Viewport::with_physical_size(
            iced_core::Size::new(width * 2, height * 2),
            2.,
        );
        if let Some(wp_viewport) = self.wp_viewport.as_ref() {
            wp_viewport.set_destination((width) as i32, (height) as i32);
        }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn physical_size(&self) -> Size<u32> {
        self.viewport.physical_size()
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.viewport.logical_size()
    }

    pub fn scale_factor(&self) -> f64 {
        self.viewport.scale_factor()
    }

    pub fn text_color(&self) -> Color {
        self.appearance.text_color
    }

    pub fn background_color(&self) -> Color {
        self.appearance.background_color
    }

    pub fn theme(&self) -> &A::Theme {
        &self.theme
    }

    pub fn mouse_position(&self) -> Option<&Point> {
        self.mouse_position.as_ref()
    }

    pub fn cursor(&self) -> IcedMouse::Cursor {
        self.mouse_position
            .map(IcedMouse::Cursor::Available)
            .unwrap_or(IcedMouse::Cursor::Unavailable)
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorLeft => {
                self.mouse_position = None;
            }
            WindowEvent::CursorMoved { x, y } => {
                self.mouse_position = Some(Point::new(*x as f32, *y as f32));
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = *modifiers;
            }
            _ => {}
        }
    }

    pub fn synchronize(&mut self, application: &A) {
        let new_scale_factor = application.scale_factor(self.id);
        if self.scale_factor != new_scale_factor {
            let size = self.physical_size() * 2;
            println!("synchronize: width: {}, height: {}", size.width, size.height);
            self.viewport = Viewport::with_physical_size(size, 2.);
            if let Some(wp_viewport) = self.wp_viewport.as_ref() {
                wp_viewport.set_destination(size.width as i32, size.height as i32);
            }
            self.viewport_version = self.viewport_version.wrapping_add(1);
            self.scale_factor = new_scale_factor;
        }
        self.theme = application.theme();
        self.appearance = application.style(&self.theme);
    }
}
