use alloc::boxed::Box;
use esp_hal::{
    Blocking,
    delay::Delay,
    gpio::{
        Output,
        OutputConfig,
        Level,
        AnyPin
    },
    spi::master::{
        Config,
        Spi,
        Instance
    },
    time::Rate
};
use embedded_hal_bus::spi::ExclusiveDevice;
use mipidsi::{ 
    NoResetPin,
    interface::{
        SpiInterface
    },
    options::{
        ColorInversion,
        Orientation,
        Rotation
    },
    models::ST7789,
};
use bevy::prelude::Resource;

pub type Display = mipidsi::Display<
    SpiInterface<
        'static,
        ExclusiveDevice<
            esp_hal::spi::master::Spi<
                'static, 
                Blocking
            >,
            Output<
                'static
            >,
            embedded_hal_bus::spi::NoDelay
        >,
        Output<
            'static
        >,
    >,
    ST7789,
    NoResetPin,
>;

const DISPLAY_HORIZONTAL_RESOLUTION: u16 = 320;
const DISPLAY_VERTICAL_RESOLUTION: u16 = 170;

#[derive(Resource)]
pub struct DisplayResource {
    pub display: Display,
}

impl DisplayResource {
    pub fn new<T: Instance + 'static>(
        backlight_pin: AnyPin<'static>,
        tft_cs_pin: AnyPin<'static>,
        spi_instance: T,
        tft_sck_pin: AnyPin<'static>,
        tft_mosi_pin: AnyPin<'static>,
        tft_dc_pin: AnyPin<'static>,
        tft_enable_pin: AnyPin<'static>
    ) -> Self {
        Output::new(backlight_pin, Level::High, OutputConfig::default())
            .set_high();

        let spi = Spi::new(
            spi_instance, 
            Config::default()
                .with_frequency(Rate::from_mhz(40))
                .with_mode(esp_hal::spi::Mode::_0)
        )
            .unwrap()
            .with_sck(tft_sck_pin)
            .with_mosi(tft_mosi_pin);
        
        let spi_device = ExclusiveDevice::new_no_delay(spi, Output::new(tft_cs_pin, Level::High, OutputConfig::default()))
            .unwrap();
        let spi_buffer = Box::new([0_u8; 512]);
        let spi_interface = SpiInterface::new(spi_device, Output::new(tft_dc_pin, Level::Low, OutputConfig::default()), Box::leak(spi_buffer));
        let display = mipidsi::Builder::new(ST7789, spi_interface)
            .display_size(DISPLAY_VERTICAL_RESOLUTION, DISPLAY_HORIZONTAL_RESOLUTION)
            .display_offset(35, 0)
            .invert_colors(ColorInversion::Inverted)
            .orientation(Orientation::default().rotate(Rotation::Deg90))
            .init(&mut Delay::new())
            .unwrap();

        Output::new(tft_enable_pin, Level::High, OutputConfig::default())
            .set_high();

        DisplayResource { display }
    }
}