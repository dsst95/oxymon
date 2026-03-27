#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  // TODO: Configure the microcontroller's peripherals here.
  // TODO: Initialize slint and render the splash screen using a state machine
  embassy_stm32::init(Default::default());

  loop {}
}
