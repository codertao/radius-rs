// Code generated by machine generator; DO NOT EDIT.

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

pub const EGRESS_VLANID_TYPE: AVPType = 56;
pub fn delete_egress_vlanid(packet: &mut Packet) {
    packet.delete(EGRESS_VLANID_TYPE);
}
pub fn add_egress_vlanid(packet: &mut Packet, value: u32) {
    packet.add(AVP::from_u32(EGRESS_VLANID_TYPE, value));
}
pub fn lookup_egress_vlanid(packet: &Packet) -> Option<Result<u32, AVPError>> {
    packet.lookup(EGRESS_VLANID_TYPE).map(|v| v.encode_u32())
}
pub fn lookup_all_egress_vlanid(packet: &Packet) -> Result<Vec<u32>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(EGRESS_VLANID_TYPE) {
        vec.push(avp.encode_u32()?)
    }
    Ok(vec)
}

pub const INGRESS_FILTERS_TYPE: AVPType = 57;
pub fn delete_ingress_filters(packet: &mut Packet) {
    packet.delete(INGRESS_FILTERS_TYPE);
}
pub fn add_ingress_filters(packet: &mut Packet, value: IngressFilters) {
    packet.add(AVP::from_u32(INGRESS_FILTERS_TYPE, value as u32));
}
pub fn lookup_ingress_filters(packet: &Packet) -> Option<Result<IngressFilters, AVPError>> {
    packet
        .lookup(INGRESS_FILTERS_TYPE)
        .map(|v| Ok(v.encode_u32()? as IngressFilters))
}
pub fn lookup_all_ingress_filters(packet: &Packet) -> Result<Vec<IngressFilters>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(INGRESS_FILTERS_TYPE) {
        vec.push(avp.encode_u32()? as IngressFilters)
    }
    Ok(vec)
}

pub const EGRESS_VLAN_NAME_TYPE: AVPType = 58;
pub fn delete_egress_vlan_name(packet: &mut Packet) {
    packet.delete(EGRESS_VLAN_NAME_TYPE);
}
pub fn add_egress_vlan_name(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(EGRESS_VLAN_NAME_TYPE, value));
}
pub fn lookup_egress_vlan_name(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(EGRESS_VLAN_NAME_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_egress_vlan_name(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(EGRESS_VLAN_NAME_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const USER_PRIORITY_TABLE_TYPE: AVPType = 59;
pub fn delete_user_priority_table(packet: &mut Packet) {
    packet.delete(USER_PRIORITY_TABLE_TYPE);
}
pub fn add_user_priority_table(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(USER_PRIORITY_TABLE_TYPE, value));
}
pub fn lookup_user_priority_table(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(USER_PRIORITY_TABLE_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_user_priority_table(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(USER_PRIORITY_TABLE_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}

pub type IngressFilters = u32;
pub const INGRESS_FILTERS_ENABLED: IngressFilters = 1;
pub const INGRESS_FILTERS_DISABLED: IngressFilters = 2;
