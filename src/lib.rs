mod plugin;
mod session;
pub mod socket;

use std::ops::{Deref, DerefMut};

use bevy::ecs::system::Resource;
use bevy_ggrs::ggrs::{PlayerHandle, PlayerType};
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

impl Deref for UdsInstance {
    type Target = Uds;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for UdsInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource, Default, Debug)]
pub struct UdsSession {
    channels: UdsChannels,
}

impl UdsInstance {
    /// Get all peers currently connected to the network
    pub fn peers(&self) -> ctru::Result<Vec<(PlayerType<NodeID>, PlayerHandle)>> {
        let status = self.0.get_connection_status()?;
        let ps = (0..16)
            .filter(|i| (status.node_bitmask() & (1 << i)) != 0) // connected check
            .map(|i| {
                let id = NodeID::Node(i + 1);
                (
                    if id == status.cur_node_id() {
                        PlayerType::Local
                    } else {
                        PlayerType::Remote(id)
                    },
                    i as usize,
                )
            })
            .collect();
        Ok(ps)
    }
}
