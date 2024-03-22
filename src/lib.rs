mod plugin;

use ctru::services::uds::NodeID;

pub use plugin::UdsGgrsPlugin;

pub struct UdsPacket {
    pub data: Vec<u8>,
    pub id: NodeID,
}
pub struct UdsInstance(pub Uds);

pub type UdsID = NodeID;

#[derive(Debug, Resource)]
pub struct UdsChannels {
    tx: Sender<UdsPacket>,
    rx: Receiver<UdsPacket>,
}
