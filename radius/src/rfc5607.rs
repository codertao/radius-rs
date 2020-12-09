// Code generated by machine generator; DO NOT EDIT.

use crate::avp::{AVPError, AVPType, AVP};
use crate::packet::Packet;

pub const FRAMED_MANAGEMENT_TYPE: AVPType = 133;
pub fn delete_framed_management(packet: &mut Packet) {
    packet.delete(FRAMED_MANAGEMENT_TYPE);
}
pub fn add_framed_management(packet: &mut Packet, value: FramedManagement) {
    packet.add(AVP::from_u32(FRAMED_MANAGEMENT_TYPE, value as u32));
}
pub fn lookup_framed_management(packet: &Packet) -> Option<Result<FramedManagement, AVPError>> {
    packet
        .lookup(FRAMED_MANAGEMENT_TYPE)
        .map(|v| Ok(v.encode_u32()? as FramedManagement))
}
pub fn lookup_all_framed_management(packet: &Packet) -> Result<Vec<FramedManagement>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_MANAGEMENT_TYPE) {
        vec.push(avp.encode_u32()? as FramedManagement)
    }
    Ok(vec)
}

pub const MANAGEMENT_TRANSPORT_PROTECTION_TYPE: AVPType = 134;
pub fn delete_management_transport_protection(packet: &mut Packet) {
    packet.delete(MANAGEMENT_TRANSPORT_PROTECTION_TYPE);
}
pub fn add_management_transport_protection(
    packet: &mut Packet,
    value: ManagementTransportProtection,
) {
    packet.add(AVP::from_u32(
        MANAGEMENT_TRANSPORT_PROTECTION_TYPE,
        value as u32,
    ));
}
pub fn lookup_management_transport_protection(
    packet: &Packet,
) -> Option<Result<ManagementTransportProtection, AVPError>> {
    packet
        .lookup(MANAGEMENT_TRANSPORT_PROTECTION_TYPE)
        .map(|v| Ok(v.encode_u32()? as ManagementTransportProtection))
}
pub fn lookup_all_management_transport_protection(
    packet: &Packet,
) -> Result<Vec<ManagementTransportProtection>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MANAGEMENT_TRANSPORT_PROTECTION_TYPE) {
        vec.push(avp.encode_u32()? as ManagementTransportProtection)
    }
    Ok(vec)
}

pub const MANAGEMENT_POLICY_ID_TYPE: AVPType = 135;
pub fn delete_management_policy_id(packet: &mut Packet) {
    packet.delete(MANAGEMENT_POLICY_ID_TYPE);
}
pub fn add_management_policy_id(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(MANAGEMENT_POLICY_ID_TYPE, value));
}
pub fn lookup_management_policy_id(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(MANAGEMENT_POLICY_ID_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_management_policy_id(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MANAGEMENT_POLICY_ID_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const MANAGEMENT_PRIVILEGE_LEVEL_TYPE: AVPType = 136;
pub fn delete_management_privilege_level(packet: &mut Packet) {
    packet.delete(MANAGEMENT_PRIVILEGE_LEVEL_TYPE);
}
pub fn add_management_privilege_level(packet: &mut Packet, value: u32) {
    packet.add(AVP::from_u32(MANAGEMENT_PRIVILEGE_LEVEL_TYPE, value));
}
pub fn lookup_management_privilege_level(packet: &Packet) -> Option<Result<u32, AVPError>> {
    packet
        .lookup(MANAGEMENT_PRIVILEGE_LEVEL_TYPE)
        .map(|v| v.encode_u32())
}
pub fn lookup_all_management_privilege_level(packet: &Packet) -> Result<Vec<u32>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MANAGEMENT_PRIVILEGE_LEVEL_TYPE) {
        vec.push(avp.encode_u32()?)
    }
    Ok(vec)
}

pub type FramedManagement = u32;
pub const FRAMED_MANAGEMENT_SNMP: FramedManagement = 1;
pub const FRAMED_MANAGEMENT_WEB_BASED: FramedManagement = 2;
pub const FRAMED_MANAGEMENT_NETCONF: FramedManagement = 3;
pub const FRAMED_MANAGEMENT_FTP: FramedManagement = 4;
pub const FRAMED_MANAGEMENT_TFTP: FramedManagement = 5;
pub const FRAMED_MANAGEMENT_SFTP: FramedManagement = 6;
pub const FRAMED_MANAGEMENT_RCP: FramedManagement = 7;
pub const FRAMED_MANAGEMENT_SCP: FramedManagement = 8;

pub type ManagementTransportProtection = u32;
pub const MANAGEMENT_TRANSPORT_PROTECTION_NO_PROTECTION: ManagementTransportProtection = 1;
pub const MANAGEMENT_TRANSPORT_PROTECTION_INTEGRITY_PROTECTION: ManagementTransportProtection = 2;
pub const MANAGEMENT_TRANSPORT_PROTECTION_INTEGRITY_CONFIDENTIALITY_PROTECTION:
    ManagementTransportProtection = 3;

pub type ServiceType = u32;
pub const SERVICE_TYPE_FRAMED_MANAGEMENT: ServiceType = 18;
