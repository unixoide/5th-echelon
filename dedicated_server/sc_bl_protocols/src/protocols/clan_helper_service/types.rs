// AUTOGENERATED with quazal-tools

#![allow(clippy::enum_variant_names, clippy::upper_case_acronyms)]
#[derive(Debug, ToStream, FromStream)]
pub struct ClanInfo {
    pub clid: u32,
    pub tag: String,
    pub title: String,
    pub motto: String,
}