#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use panic_halt;
use stm32f1xx_hal::pac::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc::Rcc;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::{
    pac,
    serial::{Config, Serial},
};


extern crate herkulex_drs_0x01_stm32f1xx;

use herkulex_drs_0x01_stm32f1xx::communication::Communication;
use herkulex_drs_0x01_stm32f1xx::motors::Motors;


#[entry]
fn main() {
    //
    // STM32F1XX CONFIGURATION
    //

    // Get handles to the hardware objects. These functions can only be called
    // once, so that the borrow checker can ensure you don't reconfigure
    // something by accident.
    let dp: Peripherals = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    // GPIO pins on the STM32F1 must be driven by the APB2 peripheral clock.
    // This must be enabled first. The HAL provides some abstractions for

    // us: First get a handle to the RCC peripheral:
    let rcc: Rcc = dp.RCC.constrain();
    // Now we have access to the RCC's registers. The GPIOC can be enabled in
    // RCC_APB2ENR (Prog. Ref. Manual 8.3.7), therefore we must pass this
    // register to the `split` function.
    // This gives us an exclusive handle to the GPIOC peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin C13
    // is usually connected to the Bluepills onboard LED.

    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut afio = dp.AFIO.constrain();

    // let sys_clock = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let clocks_serial = rcc.cfgr.freeze(&mut flash.acr);

    // USART1 on Pins A9 and A10
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;

    let serial = Serial::usart1(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()), // baud rate defined in herkulex doc : 115200
        clocks_serial.clone(),
    );

    //
    // DRIVER USAGE EXAMPLE
    // Set a servo position
    //

    // Separate into tx and rx channels
    let (mut tx, rx) = serial.split();


    // Create a communication
    let communication = Communication::new(&mut tx, rx);

    // Create a motors group associated with a communication
    let motors = Motors::new(communication);

    // Create a servomotor linked with the servo with the id 0x00
    let motor0 = motors.new_motor(0x00);

    let id = 0x00;
    // The id will be set in the eeprom
    // When you restart the servo, it will be this id
    motor0.set_id_eep(id);

    motor0.reboot();
}