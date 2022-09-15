# herkulex-stm32f1xx

Herkulex-stm3f1xx is a rust driver to facilitate the use of Herkulex DRS 0101 and DRS 0201 servomotors with a stm32f1xx
using the USART protocol.

It extends [drs-0x01](https://github.com/gbip/drs_0x01_driver), a rust driver that create messages content for these
servomotors.

You can find the documentation of Herkulex DRS 0101 and
0201 [here](http://www.sgbotic.com/products/datasheets/robotics/herkulexeng.pdf). Rust has been chosen for its
memory-safe capacities.

We had some troubles to manage the interruption of the controller to receive messages from the servomotor. In some
conditions it can be stuck in an interruption.

The emitting side of this library has been tested. However, the receiving side has not been tested entirely.

It should work with any stm32f1xx, however it has been tested only with the stm32f103.

# Features

- Control servomotors using USART protocol
- Control multiple servomotors with the same communication.

## Table of Contents

- [Usage and examples](#usage-and-example)
    - [Init the stm32f1xx and the driver](#init-the-stm32f1xx-and-the-driver)
    - [Sending data](#sending-data)
        - [Set IDs](#set-id)
            - [RAM ID](#ram-id)
            - [EEPROM ID](#eeprom-id)
        - [Set speed](#set-speed)
        - [Set position](#set-position)
    - [Receiving data](#receiving-data)
- [Contributions](#contributions)
- [Acknowledgements](#acknowledgements)
- [License](#license)

# Usage and examples

Do not forget to enable torque if you want to set speed or position to the servo.

## Init the stm32f1xx and the driver

See the [example](examples/init.rs) for more information.

```rust
extern crate herkulex_drs_0x01_stm32f1xx;

#[entry]
fn main() {
    let dp: Peripherals = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc: Rcc = dp.RCC.constrain();

    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut afio = dp.AFIO.constrain();

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

    // Separate into tx and rx channels
    let (mut tx, rx) = serial.split();

    let communication = Communication::new(&mut tx, rx);
    let motors = Motors::new(communication);
    let motor0 = motors.new_motor(0x00);
}
```

## Sending data

To send data you need to configure a tx pin on your stm32f1xx. See [here](#init-the-stm32f1xx-and-the-driver) to init
the stm32f1xx.

### Set ID

It is important to differentiate the ram and eeprom commands. If you write in the ram, the parameter will take its
previous value when you will restart the servo. When you write in the EEPROM, the value will stay here even after a
restart.

#### RAM ID

The value of the ID will be changed to the wanted one **until the restart**.

```rust
        // Create a communication
let communication = Communication::new( & mut tx, rx);

// Create a motors group associated with a communication
let motors = Motors::new(communication);
    
// Create a servomotor linked with the servo with the id 0x00
let motor0 = motors.new_motor(0x00);

let id = 0x00;
// The id will be set in the ram
// When you restart the servo, it will use the eeprom value of the id
motor0.set_id(id);
```

See the [example](examples/set_id_ram.rs) for more information.

#### EEPROM ID

The value of the ID will be changed in the eeprom. It will take effect **after a restart**.

```rust
    // Create a communication
let communication = Communication::new( & mut tx, rx);

// Create a motors group associated with a communication
let motors = Motors::new(communication);

// Create a servomotor linked with the servo with the id 0x00
let motor0 = motors.new_motor(0x00);

let id = 0x00;
// The id will be set in the eeprom
// When you restart the servo, it will be this id
motor0.set_id_eep(id);

motor0.reboot();
```

See the [example](examples/set_id_eeprom.rs) for more information.

### Set speed

To set speed you need to **enable the torque** of the servo.

```rust
    // Create a communication
let communication = Communication::new( & mut tx, rx);

// Create a motors group associated with a communication
let motors = Motors::new(communication);

// Create a servomotor linked with the servo with the id 0x00
let motor0 = motors.new_motor(0x00);

// Always enable torque to let the servo rotate
motor0.enable_torque();

// Set the position to 512 (refer to the manual p56)
// It is half max speed
// The rotation enum comes from the drs-0x01 driver
motor0.set_speed(512, Clockwise);
```

See the [example](examples/set_speed.rs) for more information (with init).

### Set position

To set speed you need to **enable the torque** of the servo.

```rust
    // Create a communication
let communication = Communication::new( & mut tx, rx);

// Create a motors group associated with a communication
let motors = Motors::new(communication);

// Create a servomotor linked with the servo with the id 0x00
let motor0 = motors.new_motor(0x00);

// Always enable torque to let the servo rotate
motor0.enable_torque();

// Set the position to 512 (refer to the manual p56)
// It corresponds to 0.163 degrees
motor0.set_position(512);
```

See [here](examples/set_position.rs) for more information(with init).

## Receiving data

Check the documentation to know whether you will get an array or an integer.

⚠ Be careful with the broadcast id, the program may stay stuck in an interruption forever.

### Get the ID of a servo with unknown id

1. Plug **ONLY** one servo
2. Create a motor with the broadcast id (0xFE)
3. Send a stat request
4. Receive an answer
5. Read the 4th column of the packet

See the example below :

```rust
    // Create a communication
let communication = Communication::new( & mut tx, rx);

// Create a motors group associated with a communication
let motors = Motors::new(communication);

// Create a servomotor linked with the broadcast ID 0xFE
let motor0 = motors.new_motor(0xFE);

// Send a stat request
// A stat request ask the servo how it is doing
//
// You should get an answer : an entire packet
// The id of the motor will be in the 4th column of the array received
//
// You can check the manual about the stat command
let id = motor0.stat()[4];
```

See [here](examples/get_id_unknown_servo.rs) for more information.

# Contributions

Feel free to send pull requests and raise issues.

Please try to create bug reports that are:

- Reproducible. Include steps to reproduce the problem.
- Specific. Include as much detail as possible: which version, what environment, etc.
- Unique. Do not duplicate existing opened issues.
- Scoped to a Single Bug. One bug per report.

# Acknowledgements

herkulex-stm32f1xx has been created by Ronan Bonnet.

[Joel](https://github.com/joelimgu) has contributed to the project and helped me to solve some bugs.

I'd like to thank Paul Florence for his drs-0x01 driver than helped me to develop this driver.

# License

This project is licensed under the **Apache-2.0**.

See [LICENSE](LICENSE) for more information.