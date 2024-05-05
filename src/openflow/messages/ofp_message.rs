pub enum OfpMsg {
    Hello = 0,
    FeaturesReq = 5,
    PacketIn = 8,
    FlowMod = 14,
    NotFound = -1,
}
