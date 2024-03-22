use ctru::services::uds::Uds;

pub struct UdsGgrsPlugin {
    pub comm_id: [u8; 4],
    pub comm_psk: String,
    pub uds: Option<Uds>,
}
