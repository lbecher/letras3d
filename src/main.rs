use env_logger::init as env_logger_init;
use pollster::block_on;
use std::rc::Rc;
use wgpu::SurfaceError;
use winit::{
    dpi::LogicalSize,
    error::OsError,
    event::{
        ElementState,
        Event,
        VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{
        ControlFlow,
        EventLoop,
    },
    window::{
        Window,
        WindowBuilder,
    },
};

mod application;
mod constants;
mod object;
mod orthographic_view;
mod perspective_view;
mod symbols;
mod texture;
mod types;

use application::Application;

use crate::symbols::*;

async fn run(event_loop: EventLoop<()>, window: Rc<Window>) {
    let mut app = Application::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        app.handle_event(&event);

        if app.captures_event(&event) {
            return;
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                app.resize(size.width, size.height);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Left) if input.state == ElementState::Pressed => {
                        app.dec_selected();
                    }
                    Some(VirtualKeyCode::Right) if input.state == ElementState::Pressed => {
                        app.inc_selected();
                    }
                    Some(VirtualKeyCode::Delete) if input.state == ElementState::Pressed => {
                        if let Some(selected) = app.selected {
                            app.del_object(selected + 1);
                        }
                    }
                    Some(VirtualKeyCode::Back) if input.state == ElementState::Pressed => {
                        if let Some(selected) = app.selected {
                            app.del_object(selected);
                        }
                    }
                    Some(VirtualKeyCode::I) if input.state == ElementState::Pressed => {
                        app.add_object(&i::SI);
                    }
                    Some(VirtualKeyCode::L) if input.state == ElementState::Pressed => {
                        app.add_object(&l::SL);
                    }
                    Some(VirtualKeyCode::U) if input.state == ElementState::Pressed => {
                        app.add_object(&u::SU);
                    }
                    Some(VirtualKeyCode::Z) if input.state == ElementState::Pressed => {
                        app.add_object(&z::SZ);
                    }
                    _ => {}
                };
            }

            Event::RedrawRequested(_) => {
                if let Err(e) = app.render(window.scale_factor() as f32) {
                    if e == SurfaceError::Outdated {
                        let size = window.inner_size();
                        app.resize(size.width, size.height);
                    } else {
                        panic!("{}", e);
                    }
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw()
            }

            _ => {}
        }
    });
}

fn main() -> Result<(), OsError> {
    env_logger_init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Letras 3D - Luiz Fernando Becher de Araujo")
        .with_inner_size(LogicalSize {
            width: 960,
            height: 640,
        })
        .with_min_inner_size(LogicalSize {
            width: 960,
            height: 640,
        })
        .build(&event_loop)?;

    block_on(run(event_loop, Rc::new(window)));

    Ok(())
}
