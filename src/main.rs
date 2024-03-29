//! MFEKabout - MFEK about screen.
//! Main author is Fredrick Brennan (@ctrlcctrlv); see AUTHORS.
//! (c) 2021. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKglif

// Cargo.toml comments say what crates are used for what.
use env_logger;
use image;
use log::warn;
use sdl2;
use skulpin;

use mfek_ipc;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::video::Window;
use skulpin::rafx::api::{RafxError, RafxExtents2D};
use skulpin::skia_safe as skia;
use skulpin::{LogicalSize, RendererBuilder};

use std::collections::HashSet;
use std::env;
use std::num::Wrapping;
use std::time::Instant;

mod consts;
use consts::*;
mod renderer;

fn main() {
    env_logger::init();
    if env::args().any(|a| a.contains("--version")) {
        println!(env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let (sdl_context, window) = initialize_sdl();

    // Skulpin initialization TODO: proper error handling
    let mut renderer = initialize_skulpin_renderer(&window).unwrap();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not create sdl event pump");

    let mut i = Wrapping(0usize);
    let start = Instant::now();

    mfek_ipc::display_header("about");
    print!("{}", MFEK);
    println!("{}", INFO);
    for module in ["glif", "metadata", "stroke", "init", "about"] {
        if let Ok((mfek_ipc::module::Version::OutOfDate(Some(mut v)), _)) =
            mfek_ipc::module::available(module.into(), "_")
        {
            if v.chars().nth(0) == Some('v') {
                v.remove(0);
            }
            println!(
                "MFEK{} ({}, v{})",
                module,
                OK,
                if v.len() == 0 { "???" } else { v.as_str() }
            );
        } else {
            println!("MFEK{} ({})", module, NG);
        }
    }

    'main_loop: loop {
        // Create a set of pressed Keys.
        let keys_down: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // sdl event handling
        for event in event_pump.poll_iter() {
            // we're gonna handle some of these events before handling commands so that we don't have the logic for this stuff
            // intertwined in command handling
            match &event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown { keycode, .. } => {
                    if (*keycode == Some(Keycode::Q)
                        && (keys_down.contains(&Keycode::LCtrl)
                            || keys_down.contains(&Keycode::RCtrl)))
                        || *keycode == Some(Keycode::Escape)
                    {
                        break 'main_loop;
                    }
                }
                _ => {}
            }

            //match event {}
        }

        let (window_width, window_height) = window.vulkan_drawable_size();
        let extents = RafxExtents2D {
            width: window_width,
            height: window_height,
        };

        let drew = renderer.draw(extents, 1.0, |canvas, _coordinate_system_helper| {
            let elapsed = start.elapsed();
            renderer::render_frame(canvas, Wrapping((elapsed.as_millis() / 36) % 180).0);
        });

        i += Wrapping(1usize);

        if drew.is_err() {
            warn!("Failed to draw frame. This can happen when resizing due to VkError(ERROR_DEVICE_LOST); if happens otherwise, file an issue.");
        }
    }
}

fn initialize_sdl() -> (sdl2::Sdl, Window) {
    // SDL initialization
    let sdl_context = sdl2::init().expect("Failed to initialize sdl2");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to create sdl video subsystem");

    video_subsystem.text_input().start();

    let logical_size = LogicalSize {
        width: WIDTH,
        height: HEIGHT,
    };

    let mut window = video_subsystem
        .window(
            &format!("About MFEK"),
            logical_size.width,
            logical_size.height,
        )
        .position_centered()
        .allow_highdpi()
        .vulkan()
        .resizable()
        .build()
        .expect("Failed to create window");

    let logo = include_bytes!("../resources/logo.png");

    let mut im = image::load_from_memory_with_format(logo, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();

    // SDL2's pixel formats are not byte-by-byte, but rather word-by-word, where the words are each
    // 32 bits long. So RGBA8888 means a 32-bit word where 8 bits are R, G, B and A. However,
    // SDL2's words are not big endian, they are little endian, so we need to reverse them.
    im.chunks_exact_mut(4).for_each(|pixel: &mut _| {
        let oldpixel: [u8; 4] = [pixel[0], pixel[1], pixel[2], pixel[3]];
        pixel[0] = oldpixel[3];
        pixel[1] = oldpixel[2];
        pixel[2] = oldpixel[1];
        pixel[3] = oldpixel[0];
    });

    let surface = Surface::from_data(&mut im, 512, 512, 512 * 4, PixelFormatEnum::RGBA8888)
        .expect("Failed to create SDL2 Surface");

    window.set_icon(surface);

    (sdl_context, window)
}

fn initialize_skulpin_renderer(sdl_window: &Window) -> Result<skulpin::Renderer, RafxError> {
    let (window_width, window_height) = sdl_window.vulkan_drawable_size();

    let extents = RafxExtents2D {
        width: window_width,
        height: window_height,
    };

    let scale_to_fit = skia::matrix::ScaleToFit::Start;
    let visible_range = skia::Rect {
        left: 0.0,
        right: WIDTH as f32 / 3.,
        top: 0.0,
        bottom: HEIGHT as f32 / 3.,
    };

    let renderer = RendererBuilder::new()
        .coordinate_system(skulpin::CoordinateSystem::VisibleRange(
            visible_range,
            scale_to_fit,
        ))
        .build(sdl_window, extents);

    return renderer;
}
