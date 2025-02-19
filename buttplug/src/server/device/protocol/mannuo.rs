// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2022 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use crate::{
  core::{errors::ButtplugDeviceError, message::Endpoint},
  server::device::{
    hardware::{HardwareCommand, HardwareWriteCmd},
    protocol::{generic_protocol_setup, ProtocolHandler},
  },
};

generic_protocol_setup!(ManNuo, "mannuo");

#[derive(Default)]
pub struct ManNuo {}

impl ProtocolHandler for ManNuo {
  fn handle_scalar_vibrate_cmd(
    &self,
    _index: u32,
    scalar: u32,
  ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
    let mut data = vec![0xAA, 0x55, 0x06, 0x01, 0x01, 0x01, scalar as u8, 0xFA];
    // Simple XOR of everything up to the 9th byte for CRC.
    let mut crc: u8 = 0;
    for b in data.clone() {
      crc ^= b;
    }
    data.push(crc);
    Ok(vec![HardwareWriteCmd::new(Endpoint::Tx, data, true).into()])
  }
}

/*
#[cfg(all(test, feature = "server"))]
mod test {
  use crate::{
    core::messages::{Endpoint, VibrateCmd, VibrateSubcommand},
    server::device::{
      hardware::{HardwareCommand, HardwareWriteCmd},
    hardware::communication::test::{
      check_test_recv_empty,
      check_test_recv_value,
      new_bluetoothle_test_device,
    }},
    util::async_manager,
  };

  #[test]
  pub fn test_mannuo_protocol() {
    async_manager::block_on(async move {
      let (device, test_device) = new_bluetoothle_test_device("Sex toys")
        .await
        .expect("Test, assuming infallible");
      let command_receiver = test_device
        .endpoint_receiver(&Endpoint::Tx)
        .expect("Test, assuming infallible");
      device
        .parse_message(VibrateCmd::new(0, vec![VibrateSubcommand::new(0, 0.5)]).into())
        .await
        .expect("Test, assuming infallible");
      check_test_recv_value(
        &command_receiver,
        HardwareCommand::Write(HardwareWriteCmd::new(
          Endpoint::Tx,
          vec![170, 85, 6, 1, 1, 1, 2, 250, 0],
          true,
        )),
      );
      // Since we only created one subcommand, we should only receive one command.
      device
        .parse_message(VibrateCmd::new(0, vec![VibrateSubcommand::new(0, 0.5)]).into())
        .await
        .expect("Test, assuming infallible");
      assert!(check_test_recv_empty(&command_receiver));
    });
  }
}
 */
