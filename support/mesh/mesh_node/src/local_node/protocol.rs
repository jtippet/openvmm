// Copyright (C) Microsoft Corporation. All rights reserved.

//! Protocol definitions for sending events to remote nodes.

use zerocopy::AsBytes;
use zerocopy::FromBytes;
use zerocopy::FromZeroes;

#[repr(C)]
#[derive(Copy, Clone, AsBytes, FromBytes, FromZeroes)]
pub struct Uuid([u8; 16]);

impl Uuid {
    pub const ZERO: Self = Self([0; 16]);

    pub fn is_zero(&self) -> bool {
        self.0 == [0; 16]
    }
}

impl From<crate::common::Uuid> for Uuid {
    fn from(value: crate::common::Uuid) -> Self {
        Self(value.0)
    }
}

impl From<Uuid> for crate::common::Uuid {
    fn from(value: Uuid) -> Self {
        Self(value.0)
    }
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
pub struct Event {
    pub port_id: Uuid,
    pub event_type: EventType,
    pub reserved: [u8; 7],
    pub seq: u64,
    pub resource_count: u32,
    pub message_size: u32,
}

open_enum::open_enum! {
    #[derive(AsBytes, FromBytes, FromZeroes)]
    pub enum EventType: u8 {
        MESSAGE = 1,
        CLOSE_PORT = 2,
        CHANGE_PEER = 3,
        ACKNOWLEDGE_CHANGE_PEER = 4,
        ACKNOWLEDGE_PORT = 5,
        FAIL_PORT = 6,
    }
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
pub struct ChangePeerData {
    pub node: Uuid,
    pub port: Uuid,
    pub seq_delta: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
pub struct FailPortData {
    pub node: Uuid,
}

#[repr(C)]
#[derive(Copy, Clone, AsBytes, FromBytes, FromZeroes)]
pub struct ResourceData {
    /// if zero, this is a file descriptor/handle
    pub id: Uuid,
    pub next_local_seq: u64,
    pub reserved: u64,
    pub old_node: Uuid,
    pub old_port: Uuid,
    /// if peer_port is zero, this is the node that caused the port to fail
    pub peer_node: Uuid,
    /// if zero, the port is failed
    pub peer_port: Uuid,
}