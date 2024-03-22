mod plugin;
mod session;
pub mod socket;

use bevy::ecs::system::Resource;
use crossbeam::channel::{Receiver, Sender};
use ctru::services::uds::{NodeID, Uds};

pub use plugin::UdsGgrsPlugin;
use socket::UdsChannels;

#[derive(Debug, Clone)]
pub struct UdsPacket {
    pub data: Vec<u8>,
    pub id: NodeID,
    pub channel: u8,
}
pub struct UdsInstance(pub Uds);

#[derive(Resource, Default, Debug)]
pub struct UdsSession {
    channels: UdsChannels,
}

pub type UdsID = NodeID;
