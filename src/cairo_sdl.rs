extern crate cairo;
extern crate sdl2;

use cairo::Context;
use cairo::FontFace;
use cairo::FontSlant;
use cairo::FontWeight;
use cairo::Format;
use cairo::ImageSurface;

use sdl2::pixels::PixelFormatEnum;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::surface::Surface;

use std::thread::sleep;
use std::time;

static SCREEN_WIDTH: i32 = 400;
static SCREEN_HEIGHT: i32 = 300;

pub fn run_sdl2() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let window = video_subsys
        .window(
            "Cairo SDL2 Example",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    let mut win_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let vrect = win_canvas.viewport();
    let scale = vrect.w as f64 / SCREEN_WIDTH as f64;

    win_canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    win_canvas.clear();

    let sdl_surface =
        Surface::new(vrect.w as u32, vrect.h as u32, PixelFormatEnum::RGBA32).unwrap();

    let stride: i32 = sdl_surface.pitch() as i32;

    let data = unsafe {
        let pixels = (*sdl_surface.raw()).pixels;
        let size: usize = sdl_surface.height() as usize * sdl_surface.pitch() as usize;
        println!("size: {}", size);
        std::slice::from_raw_parts_mut(pixels as *mut u8, size)
    };

    let cairo_surface =
        ImageSurface::create_for_data(data, Format::ARgb32, vrect.w as i32, vrect.h as i32, stride)
            .unwrap();

    let cr = Context::new(&cairo_surface);
    cr.scale(scale, scale);
    cr.set_line_width(25.0);

    cr.set_source_rgb(1.0, 0.0, 0.0);
    cr.line_to(0., 0.);
    cr.line_to(100., 100.);
    cr.stroke();

    cr.set_source_rgb(0.0, 0.0, 1.0);
    cr.line_to(0., 100.);
    cr.line_to(100., 0.);
    cr.stroke();
    cr.move_to(20.0, 220.0);
    cr.set_font_size(14.0);

    cr.set_source_rgb(0.0, 0.0, 0.0);

    let font = FontFace::toy_create("Helvetica", FontSlant::Normal, FontWeight::Bold);
    cr.set_font_face(&font);
    cr.show_text("…or create a new repository on the command line");
    let angle = 0.0;
    let dst = Some(vrect);

    let texture_creator = win_canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&sdl_surface)
        .unwrap();

    win_canvas.copy_ex(
        &texture,
        None,
        dst,
        angle,
        Some(Point::new(vrect.w, vrect.h)),
        false,
        false,
    )?;

    win_canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        sleep(time::Duration::from_millis(100));
    }

    Ok(())
}
