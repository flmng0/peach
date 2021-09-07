pub mod render;
pub mod sketch;
pub mod types;

pub use {lyon_tessellation as tess, rgb};

pub mod prelude {
    #[cfg(feature = "force_f32")]
    pub use std::f32::consts::*;
    #[cfg(not(feature = "force_f32"))]
    pub use std::f64::consts::*;

    pub use crate::render::*;
    pub use crate::sketch::*;
    pub use crate::types::*;

    #[rustfmt::skip]
    pub mod colors {
        use super::Color;

        pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        pub const RED  : Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
        pub const BLUE : Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    }
}

use std::time::{Duration, Instant};

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use self::render::Graphics;
use self::sketch::{Handler, Settings, Sketch};

pub fn run<H: 'static + Handler>(settings: Settings) -> Result<()> {
    wgpu_subscriber::initialize_default_subscriber(None);

    let mut builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(settings.size.width, settings.size.height))
        .with_decorations(settings.decorations);

    if let Some(title) = settings.title {
        builder = builder.with_title(title);
    }

    let event_loop = EventLoop::new();
    let window = builder.build(&event_loop)?;

    let mut sketch = Sketch::new(window, settings);
    let mut handler = H::setup(&mut sketch);

    let mut last_draw_time = Instant::now();
    let mut frame_delay = Duration::default();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                if sketch.framerate_dirty {
                    if let Some(fps) = sketch.framerate {
                        frame_delay = Duration::from_secs(1) / fps;
                    }
                    sketch.framerate_dirty = false;
                }

                let delta = last_draw_time.elapsed();
                let draw = match sketch.framerate {
                    Some(_) => delta >= frame_delay,
                    None => true,
                };

                if draw {
                    sketch.window.request_redraw();
                }
            },
            Event::RedrawRequested(..) => {
                let mut gfx = Graphics::new(sketch.get_clear_color());

                handler.draw(&mut sketch, &mut gfx);
                sketch.renderer.render(gfx).unwrap();

                last_draw_time = Instant::now();
            },
            Event::LoopDestroyed => {
                handler.quit();
            },
            Event::WindowEvent { event, .. } => sketch.handle_event(&mut handler, event),
            _ => {},
        }

        if sketch.has_stopped() {
            *control_flow = ControlFlow::Exit;
        }
    })
}
