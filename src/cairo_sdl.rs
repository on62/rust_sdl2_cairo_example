extern crate cairo;
extern crate chrono;

extern crate sdl2;
use cairo::Context;
use cairo::Format;
use cairo::ImageSurface;

use sdl2::pixels::PixelFormatEnum;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ops::Deref;

use sdl2::surface::Surface;

use cairo::{FontFace, FontSlant, FontWeight};
use chrono::prelude::*;
use chrono::{DateTime, Local};
use std::f64::consts::PI;
use std::thread::sleep;
use std::time;

static SCREEN_WIDTH: i32 = 400;
static SCREEN_HEIGHT: i32 = 300;
static RADIUS: f64 = 100.0;

fn draw_cairo(surface: &ImageSurface, scale: f64) {
    let cr = Context::new(surface);
    cr.scale(scale, scale);
    cr.set_source_rgba(0.77, 0.4, 0.2, 1.0);
    cr.rectangle(0., 0., SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
    cr.fill();

    // Clock Face
    cr.set_line_width(2.0);

    let center_x = SCREEN_WIDTH as f64 / 2.0;
    let center_y = SCREEN_HEIGHT as f64 / 2.0 - 40.0;

    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.arc(center_x, center_y, RADIUS, 0.0, 2.0 * PI);
    cr.stroke();

    for i in (0..12).rev() {
        let mut inset = 0.1 * RADIUS;
        if i % 3 == 0 {
            inset = 0.2 * RADIUS;
        }
        let new_x = center_x + (RADIUS - inset) * (i as f64 * PI / 6.0).cos();
        let new_y = center_y + (RADIUS - inset) * (i as f64 * PI / 6.0).sin();
        cr.move_to(new_x, new_y);

        let new_x = center_x + RADIUS * (i as f64 * PI / 6.0).cos();
        let new_y = center_y + RADIUS * (i as f64 * PI / 6.0).sin();

        cr.line_to(new_x, new_y);
        cr.stroke();
    }
    // Hour

    cr.set_source_rgb(2.0, 0.0, 0.0);
    let local: DateTime<Local> = Local::now();
    let hours = local.hour();
    let minutes = local.minute();
    let seconds = local.second();

    cr.set_line_width(5.0);
    cr.move_to(center_x, center_y);
    let mut new_x =
        center_x + RADIUS / 2.0 * (PI / 6.0 * hours as f64 + PI / 360.0 * minutes as f64).sin();
    let mut new_y =
        center_y + RADIUS / 2.0 * -(PI / 6.0 * hours as f64 + PI / 360.0 * minutes as f64).cos();
    cr.line_to(new_x, new_y);
    cr.stroke();

    // Minutes
    cr.set_source_rgb(0.0, 3.0, 0.0);
    cr.set_line_width(4.0);
    cr.move_to(center_x, center_y);
    new_x = center_x + RADIUS * 0.7 * (PI / 30.0 * minutes as f64).sin();
    new_y = center_y + RADIUS * 0.7 * -(PI / 30.0 * minutes as f64).cos();
    cr.line_to(new_x, new_y);
    cr.stroke();

    // Seconds
    cr.set_source_rgb(0.0, 0.0, 4.0);
    cr.set_line_width(3.0);
    cr.move_to(center_x, center_y);
    new_x = center_x + RADIUS * 0.75 * (PI / 30.0 * seconds as f64).sin();
    new_y = center_y + RADIUS * 0.75 * -(PI / 30.0 * seconds as f64).cos();
    cr.line_to(new_x, new_y);
    cr.stroke();

    let font = FontFace::toy_create("Sans", FontSlant::Normal, FontWeight::Bold);
    cr.set_font_face(&font);
    // show text
    cr.set_source_rgb(1.0, 1.0, 1.0);

    let text = local.format("%H:%M:%S").to_string();
    cr.set_font_size(24.0);

    let text_ext = cr.text_extents(&text);
    cr.move_to(center_x - text_ext.width as f64 / 2.0, 260.0);
    cr.show_text(&text);
}

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

    win_canvas.clear();

    let sdl_surface = Surface::new(vrect.w as u32, vrect.h as u32, PixelFormatEnum::RGBA32)
        .map_err(|e| e.to_string())?;

    let stride: i32 = sdl_surface.pitch() as i32;
    let mut cairo_surface = ImageSurface::create(Format::ARgb32, vrect.w as i32, vrect.h as i32)
        .map_err(|e| e.to_string())?;

    let texture_creator = win_canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(PixelFormatEnum::RGBA32, vrect.w as u32, vrect.h as u32)
        .map_err(|e| e.to_string())?;

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
        draw_cairo(&cairo_surface, scale);
        let data = cairo_surface.get_data().map_err(|e| e.to_string())?;
        texture
            .update(vrect, data.deref(), stride as usize)
            .map_err(|e| e.to_string())?;
        win_canvas.copy(&texture, vrect, vrect)?;
        win_canvas.present();
        sleep(time::Duration::from_millis(200));
    }

    Ok(())
}
