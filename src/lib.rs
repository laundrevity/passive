mod app;
mod collision;
mod constants;
mod game;
mod game_object;
mod sprite;
mod texture;
mod utils;

use app::App;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use utils::get_time;

#[cfg(target_arch = "wasm32")]
use utils::device_pixel_ratio;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("passive")
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web 🙃
        use winit::dpi::PhysicalSize;

        let dpi = device_pixel_ratio();
        let physical_width = (WINDOW_WIDTH as f64 * dpi) as u32;
        let physical_height = (WINDOW_HEIGHT as f64 * dpi) as u32;

        window.set_inner_size(PhysicalSize::new(physical_width, physical_height));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("passive")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut app = App::new(window).await;
    let mut last_frame = get_time();
    let mut dt = 0f32;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == app.window().id() => {
            if !app.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(physical_size) => {
                        app.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we gotta deref twice
                        app.resize(**new_inner_size);
                    }

                    _ => {}
                }
            }
        }

        Event::RedrawRequested(window_id) if window_id == app.window().id() => {
            let current_frame = get_time();
            dt = current_frame - last_frame;
            last_frame = current_frame;

            app.update(dt);

            match app.render() {
                Ok(_) => {}
                // Reconfigure the surface if it's lost or outdated
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    // state.resize(state.size)
                    *control_flow = ControlFlow::Exit;
                }
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // We're ignoring timeouts
                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
            }
        }

        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually request it
            app.window().request_redraw();
        }

        _ => {}
    });
}
