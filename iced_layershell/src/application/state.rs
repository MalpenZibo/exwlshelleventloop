use crate::application::Application;
use crate::{Appearance, DefaultStyle};
use iced_core::{mouse as IcedMouse, Color, Point, Size};
use iced_graphics::Viewport;
use layershellev::keyboard::ModifiersState;

use crate::event::WindowEvent;

pub struct State<A: Application>
where
    A::Theme: DefaultStyle,
{
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
    pub fn new(application: &A, window: &layershellev::WindowStateSimple) -> Self {
        let scale_factor = application.scale_factor();
        let theme = application.theme();
        let appearance = application.style(&theme);

        let viewport = {
            let (width, height) = window.main_window().get_size();

            let viewport = Viewport::with_physical_size(
                iced_core::Size::new(width, height),
                1. * scale_factor,
            );
            if let Some(wp_viewport) = window.main_window().get_wp_viewport() {
                wp_viewport.set_destination((width / 2) as i32, (height / 2) as i32);
            }
            viewport
        };
        Self {
            scale_factor,
            viewport,
            wp_viewport: window.main_window().get_wp_viewport().cloned(),
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
        self.viewport = Viewport::with_physical_size(
            iced_core::Size::new(width, height),
            1. * self.scale_factor(),
        );
        if let Some(wp_viewport) = self.wp_viewport.as_ref() {
            wp_viewport.set_destination((width / 2) as i32, (height / 2) as i32);
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
        let new_scale_factor = application.scale_factor();
        if self.scale_factor != new_scale_factor {
            let size = self.physical_size();
            self.viewport =
                Viewport::with_physical_size(size, 1. * new_scale_factor);

            if let Some(wp_viewport) = self.wp_viewport.as_ref() {
                wp_viewport.set_destination((size.width / 2) as i32, (size.height / 2) as i32);
            };
            self.viewport_version = self.viewport_version.wrapping_add(1);
            self.scale_factor = new_scale_factor;
        }
        self.theme = application.theme();
        self.appearance = application.style(&self.theme);
    }
}
