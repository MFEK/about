//! MFEKabout - MFEK about screen.
//! Main author is Fredrick Brennan (@ctrlcctrlv); see AUTHORS.
//! (c) 2021. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKglif
#![feature(
    panic_info_message,
    stmt_expr_attributes,
    cell_leak,
)]

// Cargo.toml comments say what crates are used for what.
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate skulpin;
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Mod,
    video::Window,
    Sdl,
};
pub use skulpin::skia_safe;
use skulpin::{rafx::api::RafxError, rafx::api::RafxExtents2D, LogicalSize, RendererBuilder};
use mfek_ipc;

use std::collections::HashSet;
use std::time::Instant;
use std::num::Wrapping;

pub mod renderer;

static WIDTH: u32 = 600;
static HEIGHT: u32 = 300;

static INFO: &str = r#"Modular Font Editor K (MFEK)
(c) 2020-2021 Fredrick R. Brennan
(c) 2021 Matthew Blanchard
MFEK is modular software. For other authors, see AUTHORS file in each module's GitHub repository.
Your MFEK distribution may contain non-official modules not listed below.

Modules found in your $PATH:"#;

fn main() {
    env_logger::init();

    let (sdl_context, window) = initialize_sdl();

    // Skulpin initialization TODO: proper error handling
    let mut renderer = initialize_skulpin_renderer(&window).unwrap();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not create sdl event pump");

    let mut i = Wrapping(0usize);
    let start = Instant::now();

    eprintln!("{}", INFO);
    for module in ["glif", "metadata", "stroke", "init", "about"] {
        let (ok, _) = mfek_ipc::module_available(module.into());
        if ok.assert() {
            eprintln!("MFEK{} (OK)", module);
        } else {
            eprintln!("MFEK{} (NG)", module);
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
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    keymod: km,
                    ..
                } => {
                    if km.contains(Mod::LCTRLMOD) || km.contains(Mod::RCTRLMOD) {
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
            renderer::render_frame(canvas, Wrapping((elapsed.as_millis() / 45) % 180).0);
        });

        i += Wrapping(1usize);

        if drew.is_err() {
            warn!("Failed to draw frame. This can happen when resizing due to VkError(ERROR_DEVICE_LOST); if happens otherwise, file an issue.");
        }
    }
}

fn initialize_sdl() -> (Sdl, Window) {
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

    let window = video_subsystem
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

    /* TODO: Fix icon. 
    let logo = include_bytes!("../doc/logo.png");
    let im = image::load_from_memory_with_format(logo, image::ImageFormat::Png)
        .unwrap()
        .into_rgb8();
    let mut bytes = im.into_vec();
    let surface = Surface::from_data(
        &mut bytes,
        701,
        701,
        701 * 3,
        sdl2::pixels::PixelFormatEnum::RGB888,
    )
    .unwrap();
    window.set_icon(surface);
    */

    (sdl_context, window)
}

fn initialize_skulpin_renderer(sdl_window: &Window) -> Result<skulpin::Renderer, RafxError> {
    let (window_width, window_height) = sdl_window.vulkan_drawable_size();

    let extents = RafxExtents2D {
        width: window_width,
        height: window_height,
    };

    let scale_to_fit = skulpin::skia_safe::matrix::ScaleToFit::Start;
    let visible_range = skulpin::skia_safe::Rect {
        left: 0.0,
        right: WIDTH as f32,
        top: 0.0,
        bottom: HEIGHT as f32,
    };

    let renderer = RendererBuilder::new()
        .coordinate_system(skulpin::CoordinateSystem::VisibleRange(
            visible_range,
            scale_to_fit,
        ))
        .build(sdl_window, extents);

    return renderer;
}
