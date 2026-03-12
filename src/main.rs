#![no_std]
#![no_main]

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
  embassy_stm32::init(Default::default());
  defmt::info!("Hello World!");

  loop {}
}
