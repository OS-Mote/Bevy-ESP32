#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

extern crate alloc;

mod app_state;
mod components;
mod systems;
mod resources;
mod rotary_encoder;
mod messages;
mod plugins;

use bevy::{
    prelude::*,
    app::App,
    state::app::StatesPlugin
};
use bevy_platform::{
    time::Instant
};
use esp_backtrace as _;
use esp_alloc as _;
use esp_println as _;
use esp_hal::{
    timer::{
        timg::TimerGroup,
    },
    main
};
use rotary_encoder::setup_rotary_encoder;
use resources::{
    display::DisplayResource,
    framebuffer::FrameBufferResource
};
use app_state::AppState;
use systems::{
    setup::setup,
    get_input::get_input,
    render::render,
    process_input::process_input
};
use messages::rotary_encoder_moved::RotaryEncoderMovedMessage;
use plugins::main_menu::main_menu_plugin;

use crate::systems::{clear_frame_buffer::clear_frame_buffer, draw_puppies::draw_puppies, update_display::update_display};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    setup_rotary_encoder(
        peripherals.GPIO4.into(),
        peripherals.GPIO5.into(),
        TimerGroup::new(peripherals.TIMG0).timer0.into()
    );

    fn elapsed_time() -> core::time::Duration {
        core::time::Duration::from_millis(
            esp_hal::time::Instant::now().duration_since_epoch().as_millis()
        )
    }

    unsafe { Instant::set_elapsed(elapsed_time) };

    let mut app = App::new();

    app
        .add_plugins((
            MinimalPlugins,
            StatesPlugin
        ))
        .insert_resource(
            DisplayResource::new(
                peripherals.GPIO21.into(),
                peripherals.GPIO41.into(),
                peripherals.SPI2,
                peripherals.GPIO11.into(),
                peripherals.GPIO9.into(),
                peripherals.GPIO16.into(),
                peripherals.GPIO42.into()
            )
        )
        .insert_resource(FrameBufferResource::new())
        .add_message::<RotaryEncoderMovedMessage>()
        .add_systems(Startup,
            setup
        )
        .add_systems(Update,
            ( 
                (
                    get_input,
                    process_input,
                    clear_frame_buffer,
                    draw_puppies,
                    update_display
                )
                    .chain()
            )
        )
        .init_state::<AppState>()
        .add_plugins(main_menu_plugin)
        .run();

    panic!("The event loop should not return");
}