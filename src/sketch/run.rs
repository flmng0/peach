use std::time::{Duration, Instant};

use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use super::{Delta, Handler, Settings, Sketch};
use crate::render::Graphics;

pub fn run<H: 'static + Handler>(settings: Settings) -> Result<()> {
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

    let mut delta = Delta::new();
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

                let draw = match sketch.framerate {
                    Some(_) => delta.since_last_draw <= frame_delay,
                    None => true,
                };

                delta.update();

                if draw {
                    sketch.window.request_redraw();
                }
            },
            Event::RedrawRequested(..) => {
                let mut gfx = Graphics::new(sketch.get_clear_color());

                handler.draw(&mut sketch, &mut gfx);
                sketch.renderer.render(gfx).unwrap();

                delta.last_draw_instant = Instant::now();
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
