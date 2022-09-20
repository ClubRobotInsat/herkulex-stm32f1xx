//! # herkulex-stm32f1xx
//! `herkulex-stm32f1xx` is a crate that allows the easy use of the Herkulex DRS (0101 and 0201) servomotors with an STM32f1xx.
//! It uses the create [drs_0x01](https://crates.io/crates/drs-0x01)
//!
//! # Examples
//!
//! Do not forget to enable torque when trying to turn on a servomotor!
//!
//! To set a servo to a position (see examples/set_position.rs ) :
//!
//! ```
//!     // Create a communication
//!     let communication = Communication::new(&mut tx, rx);
//!
//!     // Create a motors group associated with a communication
//!     let motors = Motors::new(communication);
//!
//!     // Create a servomotor linked with the servo with the id 0x00
//!     let motor0 = motors.new_motor(0x00);
//!
//!     // Always enable torque to let the servo rotate
//!     motor0.enable_torque();
//!
//!     // Set the position to 512 (refer to the manual p56)
//!     // It corresponds to 0.163 degrees
//!     motor0.set_position(512);
//! ```
//!
//! To set a servo speed (see examples/set_speed.rs) :
//!
//! ```
//!     // Create a communication
//!     let communication = Communication::new(&mut tx, rx);
//!
//!     // Create a motors group associated with a communication
//!     let motors = Motors::new(communication);
//!
//!     // Create a servomotor linked with the servo with the id 0x00
//!     let motor0 = motors.new_motor(0x00);
//!
//!     // Always enable torque to let the servo rotate
//!     motor0.enable_torque();
//!
//!     // Set the position to 512 (refer to the manual p56)
//!     // It is half max speed
//!     // The rotation enum comes from the drs-0x01 driver
//!     motor0.set_speed(512, Clockwise);
//! ```
//!


#![no_std]
#![warn(missing_docs)]

extern crate drs_0x01;

/// A module that implements a communication interface when creating new servo.
pub mod motors;

/// A module that implements a USART communication for the servos communications.
pub mod communication;

pub mod motor;
