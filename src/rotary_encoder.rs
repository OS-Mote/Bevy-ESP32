use alloc::vec;
use core::cell::RefCell;
use critical_section::Mutex;
use esp_hal::{
    Blocking,
    timer::{
        PeriodicTimer,
        AnyTimer
    },
    gpio::{
        AnyPin,
        Input,
        InputConfig,
        Pull
    },
    time::{
        Duration
    },
    handler
};

use rotary_encoder_embedded::{
    RotaryEncoder,
    standard::StandardMode
};

static ROTARY_ENCODER_TIMER: Mutex<RefCell<Option<PeriodicTimer<Blocking>>>> = Mutex::new(RefCell::new(None));
static ROTARY_ENCODER: Mutex<RefCell<Option<RotaryEncoder<StandardMode, Input, Input>>>> = Mutex::new(RefCell::new(None));
pub static ROTARY_ENCODER_DIRECTION: Mutex<RefCell<vec::Vec<RotaryEncoderDirection>>> = Mutex::new(RefCell::new(vec::Vec::new()));

pub enum RotaryEncoderDirection {
    Clockwise,
    Anticlockwise
}

pub fn setup_rotary_encoder(
    dt_input: AnyPin<'static>,
    clk_input: AnyPin<'static>,
    timer: AnyTimer<'static>
) {
    let rotary_encoder = RotaryEncoder::new(
        Input::new(dt_input, InputConfig::default().with_pull(Pull::Up)),
        Input::new(clk_input, InputConfig::default().with_pull(Pull::Up)),
        
    )
        .into_standard_mode();

    let mut periodic_timer = PeriodicTimer::new(timer);

    periodic_timer.set_interrupt_handler(rotary_encoder_timer_interrupt);
    
    let _ = periodic_timer.start(Duration::from_millis(10));

    critical_section::with(|cs| {
        ROTARY_ENCODER.borrow_ref_mut(cs).replace(rotary_encoder);
        
        periodic_timer.listen();

        ROTARY_ENCODER_TIMER.borrow_ref_mut(cs).replace(periodic_timer)
    });
}

#[handler]
fn rotary_encoder_timer_interrupt() {
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