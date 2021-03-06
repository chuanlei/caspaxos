use serde::{Deserialize, Serialize};

mod network;
mod paxos;
mod simulator;
mod udp_net;
mod versioned_storage;

pub use {
    network::Net,
    paxos::{Client, Server},
    simulator::simulate,
};

/// A possibly present value with an associated version number.
#[derive(
    Default,
    Debug,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct VersionedValue {
    pub ballot: u64,
    pub value: Option<Vec<u8>>,
}

impl std::ops::Deref for VersionedValue {
    type Target = Option<Vec<u8>>;

    fn deref(&self) -> &Option<Vec<u8>> {
        &self.value
    }
}

impl std::ops::DerefMut for VersionedValue {
    fn deref_mut(&mut self) -> &mut Option<Vec<u8>> {
        &mut self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Message {
    Request(Request),
    Response(Response),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Request {
    Prepare { ballot: u64, key: Vec<u8> },
    Accept { key: Vec<u8>, value: VersionedValue },
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Response {
    Promise {
        success: bool,
        current_value: VersionedValue,
    },
    Accepted {
        success: Result<(), VersionedValue>,
    },
    Pong,
}

impl Response {
    fn to_promise(self) -> (bool, VersionedValue) {
        if let Response::Promise {
            success,
            current_value,
        } = self
        {
            (success, current_value)
        } else {
            panic!("called to_promise on {:?}", self);
        }
    }

    fn to_accepted(self) -> Result<(), VersionedValue> {
        if let Response::Accepted { success } = self {
            success
        } else {
            panic!("called to_promise on {:?}", self);
        }
    }

    fn is_pong(self) -> bool {
        if let Response::Pong = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Envelope {
    uuid: uuid::Uuid,
    message: Message,
}
