use bevy::ecs::system::{NonSend, Res};
use ctru::services::uds::SendFlags;

use crate::{socket::UdsInstChannels, UdsInstance, UdsPacket, UdsSession};

pub fn uds_packets_send(uds: NonSend<UdsInstance>, session: Res<UdsSession>) {
    let Some(channels) = &session.channels.channels else {
        return;
    };
    while let Ok(pkt) = channels.rx.try_recv() {
        uds.0
            .send_packet(&pkt.data, pkt.id, pkt.channel, SendFlags::Default)
            .expect("failed to send uds packet");
    }
}

pub fn uds_packets_recv(uds: NonSend<UdsInstance>, session: Res<UdsSession>) {
    let Some(UdsInstChannels { tx, channel, .. }) = &session.channels.channels else {
        return;
    };
    let channel = *channel;
    while let Ok(Some((data, id))) = uds.0.pull_packet() {
        tx.send(UdsPacket { data, id, channel })
            .expect("failed to send channels tx");
    }
}
