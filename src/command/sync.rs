data_structure! {
    #[derive(Debug)]
    #[doc = "Used to synchronize DMX Data being sent along a network. (Note: If the IPs of the most recent DMX and Sync packet do not match the nodes will ignore the sync packet)"]
    pub struct Sync {
        #[doc = "Determines which version the server has. Will be ARTNET_PROTOCOL_VERSION by default"]
        pub version: [u8; 2],
        #[doc = "Transmit as Zero"]
        pub aux1: u8,
        #[doc = "Transmit as Zero"]
        pub aux2: u8,
    }
}

impl Default for Sync {
    fn default() -> Sync {
        Sync {
            version: super::ARTNET_PROTOCOL_VERSION,
            aux1: 0,
            aux2: 0,
        }
    }
}