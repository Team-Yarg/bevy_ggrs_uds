//! Local networking example.
//!
//! This example showcases local networking using the UDS module.

use std::ffi::CStr;

use bevy::app::{App, Update};
use bevy::core::NonSendMarker;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, NonSend, NonSendMut, Res, ResMut};
use bevy::input::Input;
use bevy_3ds::input::button::Button3ds;
use bevy_3ds::input::event::Button3dsChangedEvent;
use bevy_3ds::DefaultPlugins;
use bevy_ggrs_uds::{UdsInstance, UdsSession};
use ctru::prelude::*;
use ctru::services::uds::*;

fn handle_status_event(uds: &Uds, prev_node_mask: u16) -> ctru::Result<u16> {
    println!("Connection status event signalled");
    let status = uds.get_connection_status()?;
    println!("Status: {status:#02X?}");
    let left = prev_node_mask & (status.node_bitmask() ^ prev_node_mask);
    let joined = status.node_bitmask() & (status.node_bitmask() ^ prev_node_mask);
    for i in 0..16 {
        if left & (1 << i) != 0 {
            println!("Node {} disconnected", i + 1);
        }
    }
    for i in 0..16 {
        if joined & (1 << i) != 0 {
            println!(
                "Node {} connected: {:?}",
                i + 1,
                uds.get_node_info(NodeID::Node(i + 1))
            );
        }
    }
    Ok(status.node_bitmask())
}

const COMM_ID: &[u8; 4] = b"EXMP";
const COMM_PSK: &CStr = match CStr::from_bytes_with_nul(b"example password\0") {
    Ok(v) => v,
    Err(_) => unreachable!(),
};
const COMM_CHANNEL: u8 = 1;

fn handle_input(
    mut cmds: Commands,
    mut uds: NonSendMut<UdsInstance>,
    mut session: ResMut<UdsSession>,
    input: Res<Input<Button3ds>>,
) {
    for btn in input.get_pressed().filter(|b| input.just_pressed(**b)) {
        match btn.button_type {
            bevy_3ds::input::button::Button3dsType::B => {
                let nets = uds
                    .scan(COMM_ID, None, None)
                    .expect("failed to do uds scan");
                let sock = if nets.is_empty() {
                    session
                        .create_network(&mut uds, COMM_ID, COMM_PSK, COMM_CHANNEL)
                        .expect("failed to create network")
                } else {
                    session
                        .connect_to_network(
                            &mut uds,
                            &nets[0],
                            COMM_PSK,
                            ConnectionType::Client,
                            COMM_CHANNEL,
                        )
                        .expect("failed to connect to uds network")
                };
            }
            bevy_3ds::input::button::Button3dsType::Select => std::process::exit(0),
            _ => {}
        }
    }
}

fn main() {
    {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            std::fs::write("panic.log", info.to_string());
            prev(info);
        }));
    }
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(bevy_ggrs_uds::UdsGgrsPlugin::default())
        .add_systems(Update, handle_input);
}
