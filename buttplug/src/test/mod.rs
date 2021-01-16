mod test_device;
#[cfg(feature = "server")]
mod test_device_comm_manager;

use std::sync::{Arc, Mutex};
use crate::{
  device::DeviceImplCommand,
  util::stream::{recv_now, iffy_is_empty_check}
};
use tokio::sync::mpsc::Receiver;
pub use test_device::{
  TestDevice,
  TestDeviceEndpointChannel,
  TestDeviceImplCreator,
  TestDeviceInternal,
};
#[cfg(feature = "server")]
pub use test_device_comm_manager::{
  new_bluetoothle_test_device,
  TestDeviceCommunicationManager,
  TestDeviceCommunicationManagerHelper,
};

#[allow(dead_code)]
pub fn check_test_recv_value(receiver: &Arc<Mutex<Receiver<DeviceImplCommand>>>, command: DeviceImplCommand) {
  assert_eq!(recv_now(&mut receiver.lock().unwrap()).unwrap().unwrap(), command);
}

#[allow(dead_code)]
pub fn check_test_recv_empty(receiver: &Arc<Mutex<Receiver<DeviceImplCommand>>>) -> bool {
  iffy_is_empty_check(&mut receiver.lock().unwrap())
}