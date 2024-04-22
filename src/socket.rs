use std::ffi::CStr;

use bevy::{ecs::system::Resource, log::error};
use bevy_ggrs::ggrs::NonBlockingSocket;
use crossbeam::channel::{Receiver, Sender};
use ctru::services::uds::{ConnectionType, NetworkScanInfo, NodeID, Uds};

use crate::{UdsPacket, UdsSession};

#[derive(Resource, Debug)]
pub struct UdsSocket {
    tx: Sender<UdsPacket>,
    rx: Receiver<UdsPacket>,
    channel: u8,
}

impl UdsSocket {
    pub fn send(&mut self, to: NodeID, data: Box<[u8]>) {
        if let Err(e) = self.tx.try_send(UdsPacket {
            data,
            id: to,
            channel: self.channel,
        }) {
            error!("failed to send uds socket message {e}");
        }
    }

    pub fn recv(&mut self) -> Vec<(NodeID, Box<[u8]>)> {
        self.rx
            .try_iter()
            .map(|UdsPacket { data, id, channel }| {
                assert_eq!(
                    channel, self.channel,
                    "channel mismatch, this is likely a bug in bevy_ggrs_uds"
                );
                (id, data)
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UdsInstChannels {
    pub tx: Sender<UdsPacket>,
    pub rx: Receiver<UdsPacket>,
    pub channel: u8,
}

#[derive(Debug, Resource, Default)]
pub(crate) struct UdsChannels {
    pub channels: Option<UdsInstChannels>,
}

type CommID = [u8; 4];

impl UdsSession {
    fn create_socket(&mut self, channel: u8) -> UdsSocket {
        let (uds_tx, sock_rx) = crossbeam::channel::unbounded();
        let (sock_tx, uds_rx) = crossbeam::channel::unbounded();

        let inst_chans = UdsInstChannels {
            tx: uds_tx,
            rx: uds_rx,
            channel,
        };
        let sock = UdsSocket {
            tx: sock_tx,
            rx: sock_rx,
            channel,
        };
        self.channels.channels.replace(inst_chans);
        sock
    }

    pub fn create_network(
        &mut self,
        uds: &mut Uds,
        comm_id: &CommID,
        psk: &CStr,
        channel: u8,
    ) -> ctru::Result<UdsSocket> {
        uds.create_network(comm_id, None, None, psk.to_bytes_with_nul(), channel)?;
        Ok(self.create_socket(channel))
    }
    pub fn connect_to_network(
        &mut self,
        uds: &mut Uds,
        net: &NetworkScanInfo,
        psk: &CStr,
        conn_ty: ConnectionType,
        channel: u8,
    ) -> ctru::Result<UdsSocket> {
        uds.connect_network(net, psk.to_bytes_with_nul(), conn_ty, channel)?;
        Ok(self.create_socket(channel))
    }
}

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
