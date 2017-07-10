extern crate camera_capture;
extern crate piston_window;
extern crate image;
extern crate texture;
extern crate fps_counter;

use piston_window::{PistonWindow, Texture, WindowSettings, TextureSettings, clear};
use image::ConvertBuffer;
use fps_counter::FPSCounter;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tex: Option<Texture<_>> = None;
    let (sender, receiver) = std::sync::mpsc::channel();
    let imgthread = std::thread::spawn(move || {
        let mut fpsc = FPSCounter::new();

        let cam = camera_capture::create(0).unwrap()
                                                    .fps(30.0)
                                                    .unwrap()
                                                    .resolution(1280, 720)
                                                    .unwrap()
                                                    .start()
                                                    .unwrap();
        for frame in cam {
            println!("FPS: {}", fpsc.tick());
            if let Err(_) = sender.send(frame.convert()) {
                break;
            }
        }
    });

    while let Some(e) = window.next() {
        if let Ok(frame) = receiver.try_recv() {
             if let Some(mut t) = tex {
                t.update(&mut window.encoder, &frame).unwrap();
                tex = Some(t);
            } else {
                tex = Texture::from_image(&mut window.factory, &frame, &TextureSettings::new()).ok();
            }
        }
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            if let Some(ref t) = tex {
                piston_window::image(t, c.transform, g);
            }
        });
    }
    drop(receiver);
    imgthread.join().unwrap();
}
