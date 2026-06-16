use alloc::vec;
use bevy::reflect::Reflect;
use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::{
    Blocking, gpio::{
        AnyPin,
        Input,
        InputConfig,
        Pull,
        Event,
        Io
    }, handler, interrupt::RunLevel::Interrupt, peripherals::IO_MUX, time::Duration, timer::{
        AnyTimer, PeriodicTimer
    }
};

use rotary_encoder_embedded::{
    RotaryEncoder,
    standard::StandardMode
};

static ROTARY_ENCODER_TIMER: Mutex<RefCell<Option<PeriodicTimer<Blocking>>>> = Mutex::new(RefCell::new(None));
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder<StandardMode, Input, Input>>>> = Mutex::new(RefCell::new(None));
pub static ROTARY_ENCODER_DIRECTION: Mutex<RefCell<vec::Vec<RotaryEncoderDirection>>> = Mutex::new(RefCell::new(vec::Vec::new()));
static ROTARY_ENCODER_BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
pub static ROTARY_ENCODER_BUTTON_PRESS: Mutex<RefCell<vec::Vec<bool>>> = Mutex::new(RefCell::new(vec::Vec::new()));

pub enum RotaryEncoderDirection {
    Clockwise,
    Anticlockwise
}

pub fn setup_rotary_encoder(
    dt_pin: AnyPin<'static>,
    clk_pin: AnyPin<'static>,
    button_pin: AnyPin<'static>,
    timer: AnyTimer<'static>,
    io_mux: IO_MUX
) {
    let rotary_encoder = RotaryEncoder::new(
        Input::new(dt_pin, InputConfig::default().with_pull(Pull::Up)),
        Input::new(clk_pin, InputConfig::default().with_pull(Pull::Up)),
        
    )
        .into_standard_mode();

    let mut periodic_timer = PeriodicTimer::new(timer);

    periodic_timer.set_interrupt_handler(rotary_encoder_timer_interrupt_handler);
    
    let _ = periodic_timer.start(Duration::from_millis(10));

    let mut button_input = Input::new(button_pin, InputConfig::default().with_pull(Pull::Up));
    
    let mut io = Io::new(io_mux);

    io.set_interrupt_handler(button_interrupt_handler);
    
    critical_section::with(|cs| {
        ROTARY_ENCODER.borrow_ref_mut(cs).replace(rotary_encoder);
        
        periodic_timer.listen();

        ROTARY_ENCODER_TIMER.borrow_ref_mut(cs).replace(periodic_timer);

        button_input.listen(Event::FallingEdge);

        ROTARY_ENCODER_BUTTON.borrow_ref_mut(cs).replace(button_input)
    });

    
}

#[handler]
fn rotary_encoder_timer_interrupt_handler() {
    critical_section::with(|cs| {
        match 
            ROTARY_ENCODER
                .borrow_ref_mut(cs)
                .as_mut()
                .unwrap()
                .update()
        {
            rotary_encoder_embedded::Direction::Clockwise => {
                ROTARY_ENCODER_DIRECTION
                    .borrow_ref_mut(cs)
                    .push(RotaryEncoderDirection::Clockwise);
            }

            rotary_encoder_embedded::Direction::Anticlockwise => {
                ROTARY_ENCODER_DIRECTION
                    .borrow_ref_mut(cs)
                    .push(RotaryEncoderDirection::Anticlockwise);
            }
            
            rotary_encoder_embedded::Direction::None => {}
        }

        ROTARY_ENCODER_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt()
    });
}

#[handler]
fn button_interrupt_handler() {
    critical_section::with(|cs| {
        if ROTARY_ENCODER_BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .is_interrupt_set()
        {
        ROTARY_ENCODER_BUTTON_PRESS
            .borrow_ref_mut(cs)
            .push(true);
        }

        ROTARY_ENCODER_BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt()
    });
}