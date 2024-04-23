#[cfg(feature = "ggrs")]
mod ggrs;
mod plugin;
mod session;
pub mod socket;

use std::ops::{Deref, DerefMut};

use bevy::ecs::system::Resource;
use ctru::services::uds::{NodeID, Uds};

pub use plugin::UdsGgrsPlugin;
use socket::UdsChannels;

#[derive(Debug, Clone)]
pub struct UdsPacket {
    pub data: Box<[u8]>,
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
    pub fn current_id(&self) -> ctru::Result<NodeID> {
        Ok(self.connection_status()?.cur_node_id())
    }
    /// Get all peers currently connected to the network
    pub fn connected_peers(&self) -> ctru::Result<impl Iterator<Item = NodeID>> {
        let status = self.0.connection_status()?;
        let bitmask = status.node_bitmask();
        let ps = (0..16)
            .filter(move |i| (bitmask & (1 << i)) != 0) // connected check
            .map(|i| NodeID::Node(i + 1));
        Ok(ps)
    }
}
