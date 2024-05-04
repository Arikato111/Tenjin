/*
* it don't need to assign vlue.
* but I think I should assign for reading.
*/
pub enum FlowActionType {
    Output = 0,
    SetVlanId = 1,
    SetVlanPCP = 2,
    StripVlan = 3,
    SetSrcMac = 4,
    SetDstMac = 5,
    SetIPv4Src = 6,
    SetIPv4Des = 7,
    SetTos = 8,
    SetTpSrc = 9,
    SetTpDst = 10,
    Enqueue = 11,
}