use crate::desktop::interop::create_dispatcher_queue_controller_for_current_thread;
use crate::desktop::window_target::CompositionDesktopWindowTargetSource;
use crate::minesweeper::Minesweeper;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bindings::Windows::{
    Foundation::Numerics::Vector2,
    Win32::System::WinRT::{RoInitialize, RO_INIT_SINGLETHREADED},
    UI::Composition::Compositor,
};

pub fn run() -> windows::Result<()> {
    unsafe {
        RoInitialize(RO_INIT_SINGLETHREADED)?;
    }
    let _controller = create_dispatcher_queue_controller_for_current_thread()?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Minesweeper");

    let compositor = Compositor::new()?;
    let target = window.create_window_target(&compositor, false)?;

    let root = compositor.CreateContainerVisual()?;
    root.SetRelativeSizeAdjustment(Vector2 { X: 1.0, Y: 1.0 })?;
    target.SetRoot(&root)?;

    let window_size = window.inner_size();
    let window_size = Vector2 {
        X: window_size.width as f32,
        Y: window_size.height as f32,
    };
    let mut game = Minesweeper::new(&root, &window_size)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let size = Vector2 {
                    X: size.width as f32,
                    Y: size.height as f32,
                };
                game.on_parent_size_changed(&size).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let point = Vector2 {
                    X: position.x as f32,
                    Y: position.y as f32,
                };
                game.on_pointer_moved(&point).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed {
                    game.on_pointer_pressed(button == MouseButton::Right, false)
                        .unwrap();
                }
            }
            _ => (),
        }
    });
}