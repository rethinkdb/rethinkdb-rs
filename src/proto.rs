pub enum Version {
    /*
    V0_1 = 0x3f61ba36,
    V0_2 = 0x723081e1, // Authorization key during handshake
    V0_3 = 0x5f75e83e, // Authorization key and protocol during handshake
    V0_4 = 0x400c2d20, // Queries execute in parallel
    */
    V1_0 = 0x34c2bdc3, // Users and permissions
}

/*
// The protocol to use after the handshake, specified in V0_3
pub enum Protocol {
    PROTOBUF = 0x271ffc41,
    JSON = 0x7e6970c7,
}
*/
