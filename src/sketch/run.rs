use crate::render::Graphics;

use super::{Delta, Handler, Settings, Sketch};

use std::time::{Duration, Instant};

use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run<H: 'static + Handler>(settings: Settings) -> Result<()> {
    let mut builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(settings.size.width, settings.size.height))
        .with_decorations(settings.decorations);

    if let Some(title) = settings.title {
        builder = builder.with_title(title);
    }

    let event_loop = EventLoop::new();
    let window = builder.build(&event_loop)?;

    let mut sketch = Sketch::new(window, settings.framerate);
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

                handler.update(&mut sketch, delta);
                delta.last_update_instant = Instant::now();

                if draw {
                    sketch.window.request_redraw();
                }
            }
            Event::RedrawRequested(..) => {
                let mut gfx = Graphics::new();

                handler.draw(&mut sketch, &mut gfx);
                delta.last_draw_instant = Instant::now();

                // renderer.draw(gfx);
            }
            Event::LoopDestroyed => {
                handler.quit();
            }
            Event::WindowEvent { event, .. } => sketch.handle_event(&mut handler, event),
            _ => {}
        }

        if sketch.has_stopped() {
            *control_flow = ControlFlow::Exit;
        }
    })
}
