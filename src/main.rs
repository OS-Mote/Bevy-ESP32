#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

extern crate alloc;

mod app_state;
mod components;
mod systems;
mod resources;
mod rotary_encoder;
mod plugins;
mod events;
mod observers;
mod sprite_buffer;
mod lfs_flash;
mod storage;

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
use esp_hal::{
    main,
    peripherals,
    timer::timg::TimerGroup
};
use app_state::AppState;
use rotary_encoder::setup_rotary_encoder;
use resources::{
    display::DisplayResource,
    framebuffer::FrameBufferResource
};
use systems::{
    get_input::get_input,
    render::render,
};
use plugins::main_menu::main_menu_plugin;
use systems::{
    setup_app::setup_app,
    clear_frame_buffer::clear_frame_buffer,
    update_display::update_display
};
use esp_storage::FlashStorage;
use esp_bootloader_esp_idf::partitions;
use static_cell::StaticCell;
use littlefs2::fs::{
    Allocation,
    Filesystem
};
use storage::LfsFlash;
use partitions::{
    PartitionType,
    DataPartitionSubType
};

use crate::{plugins::settings_menu::settings_menu_plugin, resources::navigation_stack::NavigationStack};


static FLASH: StaticCell<FlashStorage> = StaticCell::new();
static LFS_PT: StaticCell<partitions::PartitionEntry> = StaticCell::new();
static LFS_STORAGE: StaticCell<storage::LfsFlash<partitions::FlashRegion<FlashStorage>>> = StaticCell::new();
static LFS_ALLOC: StaticCell<Allocation<storage::LfsFlash<partitions::FlashRegion<FlashStorage>>>> = StaticCell::new();
static mut PT_MEM: [u8; partitions::PARTITION_TABLE_MAX_LEN] = [0u8; partitions::PARTITION_TABLE_MAX_LEN];

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    // let flash = FLASH.init(FlashStorage::new(peripherals.FLASH));

    // let partition_table = unsafe {
    //     partitions::read_partition_table(flash, &mut PT_MEM).unwrap()
    // };

    // let littlefs = LFS_PT.init(partition_table
    //     .find_partition(PartitionType::Data(
    //         DataPartitionSubType::LittleFs,
    //     ))
    //     .unwrap()
    //     .unwrap());

    // let littlefs_partition = littlefs.as_embedded_storage(flash);

    // let storage = LFS_STORAGE.init(LfsFlash(littlefs_partition));
    // let alloc = LFS_ALLOC.init(Filesystem::allocate());
    // let filesystem = Filesystem::mount_or_else(alloc, storage, |_, s, _| {
    //     Filesystem::format(s)
    // }).unwrap();

    setup_rotary_encoder(
        peripherals.GPIO4.into(),
        peripherals.GPIO5.into(),
        peripherals.GPIO0.into(),
        TimerGroup::new(peripherals.TIMG0).timer0.into(),
        peripherals.IO_MUX
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
            StatesPlugin,
            main_menu_plugin,
            settings_menu_plugin
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
        .insert_resource(NavigationStack::new())
        .add_systems(Startup, setup_app)
        .add_systems(Update, clear_frame_buffer)
        .add_systems(Update, get_input.after(clear_frame_buffer))
        .add_systems(Update, update_display.after(get_input))
        .init_state::<AppState>()
        .run();

    panic!("The event loop should not return");
}