#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use drs_0x01::Rotation::Clockwise;

#[allow(unused_imports)]
use panic_halt;
use stm32f1xx_hal::pac::Peripherals;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc::Rcc;
use stm32f1xx_hal::rcc::RccExt;

extern crate drs_0x01;

use stm32f1xx_hal::{
    pac,
    serial::{Config, Serial},
};

use herkulex_drs_0x01_stm32f1xx::communication::Communication;

// extern crate herkulex_drs_0x01_stm32f1xx;

#[entry]
fn main() -> ! {
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

    // separate into tx and rx channels
    let (mut tx, rx) = serial.split();
    let mut delay = cp.SYST.delay(&clocks_serial);

    // Create a communication
    let communication = Communication::new(&mut tx, rx);

    // Create the motor group associated with the communication
    let motors = herkulex_drs_0x01_stm32f1xx::motors::Motors::new(communication);

    // Create a servomotor linked to the servo with the id 0x00
    let motor0 = motors.new_motor(0x00);

    // Create a servomotor linked to the servo with the id 0x02
    let motor2 = motors.new_motor(0x02);

    // Initialize servomotors
    motor0.reboot();
    motor2.reboot();
    delay.delay_ms(100_u16);
    motor0.enable_torque();
    motor2.enable_torque();
    delay.delay_ms(100_u16);

    loop {
        // Get the temperature of the two servos and print it in the debug console
        let t0 = motor0.get_temperature();
        hprintln!("temp servo0 : {:?}", t0);
        let t2 = motor2.get_temperature();
        hprintln!("temp servo2 : {:?}", t2);


        // Turn on / off one servo after the other for 1s each iteration
        motor0.set_speed(512, Clockwise);
        motor2.set_speed(0, Clockwise);
        delay.delay_ms(1000_u16);

        motor0.set_speed(0, Clockwise);
        motor2.set_speed(512, Clockwise);
        delay.delay_ms(1000_u16);
    }
}
