use bevy::ecs::system::Resource;
use crossbeam::channel::{Receiver, Sender};
use ctru::services::uds::{ConnectionType, NetworkScanInfo, Uds};

use crate::{UdsInstance, UdsPacket, UdsSession};

#[derive(Resource, Debug)]
pub struct UdsSocket {
    tx: Sender<UdsPacket>,
    rx: Receiver<UdsPacket>,
    channel: u8,
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
        psk: &str,
        channel: u8,
    ) -> ctru::Result<UdsSocket> {
        uds.create_network(comm_id, None, None, psk.as_bytes(), channel)?;
        Ok(self.create_socket(channel))
    }
    pub fn connect_to_network(
        &mut self,
        uds: &mut Uds,
        net: &NetworkScanInfo,
        psk: &str,
        conn_ty: ConnectionType,
        channel: u8,
    ) -> ctru::Result<UdsSocket> {
        uds.connect_network(net, psk.as_bytes(), conn_ty, channel)?;
        Ok(self.create_socket(channel))
    }
}
