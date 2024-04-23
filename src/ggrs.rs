use bevy::log::error;
use bevy_ggrs::ggrs::{NonBlockingSocket, PlayerHandle, PlayerType};
use ctru::services::uds::NodeID;

use crate::{socket::UdsSocket, UdsInstance};

impl NonBlockingSocket<NodeID> for UdsSocket {
    fn send_to(&mut self, msg: &bevy_ggrs::ggrs::Message, addr: &NodeID) {
        let data = match bincode::serialize(msg) {
            Ok(v) => v,
            Err(e) => {
                error!("failed to serialize message for uds addr {addr:?} {msg:?}: {e}");
                return;
            }
        };
        self.send(*addr, data.into());
    }

    fn receive_all_messages(&mut self) -> Vec<(NodeID, bevy_ggrs::ggrs::Message)> {
        self.recv()
            .into_iter()
            .filter_map(|(id, data)| Some((id, bincode::deserialize(&data).ok()?)))
            .collect()
    }
}

impl UdsInstance {
    /// Get all peers currently connected to the network
    pub fn peers(&self) -> ctru::Result<Vec<(PlayerType<NodeID>, PlayerHandle)>> {
        let status = self.connection_status()?;
        let ps = self
            .connected_peers()?
            .enumerate()
            .map(|(i, id)| {
                (
                    if id == status.cur_node_id() {
                        PlayerType::Local
                    } else {
                        PlayerType::Remote(id)
                    },
                    i,
                )
            })
            .collect();
        Ok(ps)
    }
}
