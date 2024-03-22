use bevy::app::{Plugin, PostUpdate};
use ctru::services::uds::Uds;

use crate::{
    session::{uds_packets_recv, uds_packets_send},
    UdsInstance, UdsSession,
};

pub struct UdsGgrsPlugin {
    create_uds: bool,
}

impl Plugin for UdsGgrsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if self.create_uds {
            app.insert_non_send_resource(UdsInstance(Uds::new(None).expect("failed to init uds")));
        }
        app.init_resource::<UdsSession>();
        app.add_systems(PostUpdate, (uds_packets_recv, uds_packets_send));
    }
}
