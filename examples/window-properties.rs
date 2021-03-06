extern crate sdl2;

use sdl2::pixels::Color;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();

    let mut running = true;
    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }

        {
            // Update the window title.
            let mut window = renderer.window_mut().unwrap();

            let position = window.position();
            let size = window.size();
            let title = format!("Window - pos({}x{}), size({}x{}): {}", position.0, position.1, size.0, size.1, tick);
            window.set_title(&title);

            tick += 1;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
