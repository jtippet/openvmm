// Copyright (C) Microsoft Corporation. All rights reserved.

//! Resource definitions for vmbus serial ports.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use mesh::MeshPayload;
use vm_resource::kind::SerialBackendHandle;
use vm_resource::kind::VmbusDeviceHandleKind;
use vm_resource::Resource;
use vm_resource::ResourceId;

/// A handle to a vmbus serial device.
#[derive(MeshPayload)]
pub struct VmbusSerialDeviceHandle {
    /// The port identity within the guest.
    pub port: VmbusSerialPort,
    /// The serial port backend.
    pub backend: Resource<SerialBackendHandle>,
}

impl ResourceId<VmbusDeviceHandleKind> for VmbusSerialDeviceHandle {
    const ID: &'static str = "vmbus_serial";
}

/// The port identity. This corresponds to different specific vmbus instance
/// IDs.
#[derive(MeshPayload)]
pub enum VmbusSerialPort {
    /// A device to reemulate as "COM1".
    Com1,
    /// A device to reemulate as "COM2".
    Com2,
}