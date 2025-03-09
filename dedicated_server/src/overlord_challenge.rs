use std::collections::HashMap;

use quazal::prudp::ClientRegistry;
use quazal::rmc::basic::FromStream;
use quazal::rmc::basic::ToStream;
use quazal::rmc::types::DateTime;
use quazal::rmc::types::Variant;
use quazal::rmc::Protocol;
use quazal::Context;
use serde::Deserialize;

use crate::login_required;

#[derive(Debug, ToStream, FromStream)]
struct GetChallengesRequest {
    class: String,
}

#[derive(Debug, ToStream, FromStream, Deserialize)]
struct Challenge {
    unk1: u32,
    unk2: String,
    some_xml: String,
    unk4: u32,
    unk5: u32,
    unk6: u32,
    unk7: u32,
    unk8: bool,
    unk9: DateTime,
    unk10: DateTime,
    unk11: String,
    unk12: String,
    unk13: String,
    unk14: HashMap<String, Variant>,
    unk15: HashMap<String, Variant>,
    unk16: HashMap<String, Variant>,
    unk17: u32,
    unk18: DateTime,
    unk19: HashMap<String, Variant>,
}

#[derive(Debug, ToStream, FromStream)]
struct GetChallengesResponse {
    challenges: Vec<Challenge>,
}

#[allow(clippy::module_name_repetitions)]
pub struct OverlordChallengeProtocol;

impl<T> Protocol<T> for OverlordChallengeProtocol {
    fn id(&self) -> u16 {
        5007
    }

    fn name(&self) -> String {
        "OverlordChallengeProtocol".into()
    }

    fn num_methods(&self) -> u32 {
        6
    }

    fn handle(
        &self,
        logger: &slog::Logger,
        _ctx: &Context,
        ci: &mut quazal::ClientInfo<T>,
        request: &quazal::rmc::Request,
        _client_registry: &ClientRegistry<T>,
        _socket: &std::net::UdpSocket,
    ) -> std::result::Result<Vec<u8>, quazal::rmc::Error> {
        login_required(&*ci)?;
        match request.method_id {
            1 => {
                let _request: GetChallengesRequest = FromStream::from_bytes(&request.parameters)?;

                let challenges = std::fs::File::open("data/challenges.json")
                    .map(serde_json::from_reader)
                    .ok()
                    .and_then(Result::ok)
                    .unwrap_or(vec![Challenge {
                        unk1: 16_0200,
                        unk2: String::from("{}"),
                        some_xml: String::from("<Challenge Name=\"LocID_SNN_ReminderGoneDark_20\" Desc=\"LocID_SNDES_ReminderGoneDark_20\" Guid=\"160200\" ShortDesc=\"LocID_SNSD_ReminderGoneDark_20\" Category=\"OnlineChallengeGoneDarkHeader\"><GoneDark id=\"160200\" PosX=\"262\" PosY=\"397\" Resource=\"GD_Grim_004\" title=\"LocID_C_INT_20_Title\" loc=\"LocID_C_INT_20_Loc_0\" desc=\"LocID_C_INT_20_Desc_1\" /><Definition><GameEvent><Event><GoneDarkUI><ID Op=\"Equal\" Value=\"160200\" /></GoneDarkUI></Event></GameEvent></Definition><StepReward Count=\"123\"><UnlockChallenge><ID val=\"160201\" /></UnlockChallenge></StepReward></Challenge>"),
                        unk4: 0,
                        unk5: 0,
                        unk6: 0,
                        unk7: 1,
                        unk8: false,
                        unk9: DateTime(0),
                        unk10: DateTime(0xFFFF_FFFF_FFFF_FFFF),
                        unk11: String::from("{}"),
                        unk12: String::from("{}"),
                        unk13: String::from("{}"),
                        unk14: HashMap::default(),
                        unk15: HashMap::from([
                            (String::from("s"), Variant::I64(1)),
                            (String::from("p"), Variant::I64(123)),
                        ]),
                        unk16: HashMap::default(),
                        unk17: 2,
                        unk18: DateTime(0xFFFF_FFFF_FFFF_FFFF),
                        unk19: HashMap::default(),
                    }]);
                Ok(GetChallengesResponse { challenges }.to_bytes())
            }
            2..=6 => {
                error!(logger, "not implemented yet");
                Err(quazal::rmc::Error::UnknownMethod)
            }
            _ => Err(quazal::rmc::Error::UnknownMethod),
        }
    }

    fn method_name(&self, method_id: u32) -> Option<String> {
        if method_id == 1 {
            Some("get_challenges".into())
        } else {
            None
        }
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(OverlordChallengeProtocol)
}

#[cfg(test)]
mod tests {
    use quazal::rmc::basic::FromStream;

    use super::*;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn parse_sample() {
        let data = b"\x03\x00\x00\x00\xc8\x71\x02\x00\x03\x00\x7b\x7d\x00\x31\x02\x3c\
      \x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x20\x4e\x61\x6d\x65\x3d\x22\
      \x4c\x6f\x63\x49\x44\x5f\x53\x4e\x4e\x5f\x52\x65\x6d\x69\x6e\x64\
      \x65\x72\x47\x6f\x6e\x65\x44\x61\x72\x6b\x5f\x32\x30\x22\x20\x44\
      \x65\x73\x63\x3d\x22\x4c\x6f\x63\x49\x44\x5f\x53\x4e\x44\x45\x53\
      \x5f\x52\x65\x6d\x69\x6e\x64\x65\x72\x47\x6f\x6e\x65\x44\x61\x72\
      \x6b\x5f\x32\x30\x22\x20\x47\x75\x69\x64\x3d\x22\x31\x36\x30\x32\
      \x30\x30\x22\x20\x53\x68\x6f\x72\x74\x44\x65\x73\x63\x3d\x22\x4c\
      \x6f\x63\x49\x44\x5f\x53\x4e\x53\x44\x5f\x52\x65\x6d\x69\x6e\x64\
      \x65\x72\x47\x6f\x6e\x65\x44\x61\x72\x6b\x5f\x32\x30\x22\x20\x43\
      \x61\x74\x65\x67\x6f\x72\x79\x3d\x22\x4f\x6e\x6c\x69\x6e\x65\x43\
      \x68\x61\x6c\x6c\x65\x6e\x67\x65\x47\x6f\x6e\x65\x44\x61\x72\x6b\
      \x48\x65\x61\x64\x65\x72\x22\x3e\x3c\x47\x6f\x6e\x65\x44\x61\x72\
      \x6b\x20\x69\x64\x3d\x22\x31\x36\x30\x32\x30\x30\x22\x20\x50\x6f\
      \x73\x58\x3d\x22\x32\x36\x32\x22\x20\x50\x6f\x73\x59\x3d\x22\x33\
      \x39\x37\x22\x20\x52\x65\x73\x6f\x75\x72\x63\x65\x3d\x22\x47\x44\
      \x5f\x47\x72\x69\x6d\x5f\x30\x30\x34\x22\x20\x74\x69\x74\x6c\x65\
      \x3d\x22\x4c\x6f\x63\x49\x44\x5f\x43\x5f\x49\x4e\x54\x5f\x32\x30\
      \x5f\x54\x69\x74\x6c\x65\x22\x20\x6c\x6f\x63\x3d\x22\x4c\x6f\x63\
      \x49\x44\x5f\x43\x5f\x49\x4e\x54\x5f\x32\x30\x5f\x4c\x6f\x63\x5f\
      \x30\x22\x20\x64\x65\x73\x63\x3d\x22\x4c\x6f\x63\x49\x44\x5f\x43\
      \x5f\x49\x4e\x54\x5f\x32\x30\x5f\x44\x65\x73\x63\x5f\x31\x22\x20\
      \x2f\x3e\x3c\x44\x65\x66\x69\x6e\x69\x74\x69\x6f\x6e\x3e\x3c\x47\
      \x61\x6d\x65\x45\x76\x65\x6e\x74\x3e\x3c\x45\x76\x65\x6e\x74\x3e\
      \x3c\x47\x6f\x6e\x65\x44\x61\x72\x6b\x55\x49\x3e\x3c\x49\x44\x20\
      \x4f\x70\x3d\x22\x45\x71\x75\x61\x6c\x22\x20\x56\x61\x6c\x75\x65\
      \x3d\x22\x31\x36\x30\x32\x30\x30\x22\x20\x2f\x3e\x3c\x2f\x47\x6f\
      \x6e\x65\x44\x61\x72\x6b\x55\x49\x3e\x3c\x2f\x45\x76\x65\x6e\x74\
      \x3e\x3c\x2f\x47\x61\x6d\x65\x45\x76\x65\x6e\x74\x3e\x3c\x2f\x44\
      \x65\x66\x69\x6e\x69\x74\x69\x6f\x6e\x3e\x3c\x53\x74\x65\x70\x52\
      \x65\x77\x61\x72\x64\x20\x43\x6f\x75\x6e\x74\x3d\x22\x31\x22\x3e\
      \x3c\x55\x6e\x6c\x6f\x63\x6b\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\
      \x3e\x3c\x49\x44\x20\x76\x61\x6c\x3d\x22\x31\x36\x30\x32\x30\x31\
      \x22\x20\x2f\x3e\x3c\x2f\x55\x6e\x6c\x6f\x63\x6b\x43\x68\x61\x6c\
      \x6c\x65\x6e\x67\x65\x3e\x3c\x2f\x53\x74\x65\x70\x52\x65\x77\x61\
      \x72\x64\x3e\x3c\x2f\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x3e\x00\
      \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
      \x00\x00\x90\x32\x96\x1f\x00\x00\x00\x00\x90\x34\x96\x1f\x00\x00\
      \x00\x03\x00\x7b\x7d\x00\x03\x00\x7b\x7d\x00\x03\x00\x7b\x7d\x00\
      \x00\x00\x00\x00\x02\x00\x00\x00\x02\x00\x70\x00\x01\x01\x00\x00\
      \x00\x00\x00\x00\x00\x02\x00\x73\x00\x01\x01\x00\x00\x00\x00\x00\
      \x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\xfb\x7e\x3f\x3f\x9c\x00\
      \x00\x00\x00\x00\x00\x00\x4e\x53\x00\x00\x03\x00\x7b\x7d\x00\x44\
      \x02\x3c\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x20\x4e\x61\x6d\x65\
      \x3d\x22\x4c\x6f\x63\x49\x64\x5f\x53\x4e\x4e\x5f\x4f\x6e\x6c\x69\
      \x6e\x65\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x44\x61\x69\x6c\x79\
      \x53\x69\x6c\x65\x6e\x63\x65\x22\x20\x44\x65\x73\x63\x3d\x22\x4c\
      \x6f\x63\x49\x64\x5f\x53\x4e\x44\x45\x53\x5f\x4f\x6e\x6c\x69\x6e\
      \x65\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x44\x61\x69\x6c\x79\x53\
      \x69\x6c\x65\x6e\x63\x65\x22\x20\x52\x65\x73\x65\x74\x3d\x22\x4e\
      \x65\x76\x65\x72\x22\x20\x47\x75\x69\x64\x3d\x22\x32\x31\x33\x32\
      \x36\x22\x20\x53\x68\x6f\x72\x74\x44\x65\x73\x63\x3d\x22\x4c\x6f\
      \x63\x49\x64\x5f\x53\x4e\x53\x44\x5f\x4f\x6e\x6c\x69\x6e\x65\x43\
      \x68\x61\x6c\x6c\x65\x6e\x67\x65\x44\x61\x69\x6c\x79\x32\x22\x20\
      \x43\x61\x74\x65\x67\x6f\x72\x79\x3d\x22\x4f\x6e\x6c\x69\x6e\x65\
      \x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x44\x61\x69\x6c\x79\x22\x20\
      \x49\x63\x6f\x6e\x3d\x22\x67\x68\x6f\x73\x74\x22\x20\x4d\x61\x70\
      \x3d\x22\x53\x5f\x52\x41\x4e\x44\x22\x3e\x3c\x44\x65\x66\x69\x6e\
      \x69\x74\x69\x6f\x6e\x3e\x3c\x47\x61\x6d\x65\x45\x76\x65\x6e\x74\
      \x3e\x3c\x43\
      \x6c\x61\x73\x73\x43\x6f\x6d\x70\x6f\x6e\x65\x6e\x74\x20\x4e\x61\
      \x6d\x65\x3d\x22\x45\x63\x68\x65\x6c\x6f\x6e\x2e\x45\x43\x75\x73\
      \x74\x6f\x6d\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x5f\x4d\x61\x70\
      \x53\x63\x6f\x72\x69\x6e\x67\x45\x6e\x61\x62\x6c\x65\x64\x22\x20\
      \x4f\x70\x3d\x22\x45\x71\x75\x61\x6c\x22\x20\x47\x61\x74\x65\x3d\
      \x22\x74\x72\x75\x65\x22\x20\x2f\x3e\x3c\x45\x76\x65\x6e\x74\x3e\
      \x3c\x4d\x69\x73\x73\x69\x6f\x6e\x43\x6f\x6d\x70\x6c\x65\x74\x65\
      \x64\x3e\x3c\x53\x75\x63\x63\x65\x73\x73\x20\x4f\x70\x3d\x22\x45\
      \x71\x75\x61\x6c\x22\x20\x56\x61\x6c\x75\x65\x3d\x22\x74\x72\x75\
      \x65\x22\x20\x2f\x3e\x3c\x57\x61\x73\x44\x65\x74\x65\x63\x74\x65\
      \x64\x20\x4f\x70\x3d\x22\x45\x71\x75\x61\x6c\x22\x20\x56\x61\x6c\
      \x75\x65\x3d\x22\x66\x61\x6c\x73\x65\x22\x20\x2f\x3e\x3c\x2f\x4d\
      \x69\x73\x73\x69\x6f\x6e\x43\x6f\x6d\x70\x6c\x65\x74\x65\x64\x3e\
      \x3c\x2f\x45\x76\x65\x6e\x74\x3e\x3c\x2f\x47\x61\x6d\x65\x45\x76\
      \x65\x6e\x74\x3e\x3c\x2f\x44\x65\x66\x69\x6e\x69\x74\x69\x6f\x6e\
      \x3e\x3c\x53\x74\x65\x70\x52\x65\x77\x61\x72\x64\x20\x43\x6f\x75\
      \x6e\x74\x3d\x22\x31\x22\x3e\x3c\x45\x63\x6f\x6e\x6f\x6d\x79\x20\
      \x56\x61\x6c\x75\x65\x54\x61\x67\x3d\x22\x44\x61\x69\x6c\x79\x4c\
      \x61\x72\x67\x65\x22\x20\x2f\x3e\x3c\x2f\x53\x74\x65\x70\x52\x65\
      \x77\x61\x72\x64\x3e\x3c\x2f\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\
      \x3e\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\
      \x00\x00\x00\x00\x90\x32\x96\x1f\x00\x00\x00\x00\x90\x34\x96\x1f\
      \x00\x00\x00\x03\x00\x7b\x7d\x00\x03\x00\x7b\x7d\x00\x03\x00\x7b\
      \x7d\x00\x00\x00\x00\x00\x02\x00\x00\x00\x02\x00\x70\x00\x01\x01\
      \x00\x00\x00\x00\x00\x00\x00\x02\x00\x73\x00\x01\x01\x00\x00\x00\
      \x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\xfb\x7e\x3f\x3f\
      \x9c\x00\x00\x00\x00\x00\x00\x00\xa6\x75\x00\x00\x03\x00\x7b\x7d\
      \x00\xdd\x01\x3c\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x20\x4e\x61\
      \x6d\x65\x3d\x22\x4c\x6f\x63\x49\x64\x5f\x53\x4e\x4e\x5f\x4f\x6e\
      \x6c\x69\x6e\x65\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x57\x65\x65\
      \x6b\x6c\x79\x46\x44\x53\x74\x6f\x70\x43\x61\x70\x74\x75\x72\x65\
      \x22\x20\x44\x65\x73\x63\x3d\x22\x4c\x6f\x63\x49\x64\x5f\x53\x4e\
      \x44\x45\x53\x5f\x4f\x6e\x6c\x69\x6e\x65\x43\x68\x61\x6c\x6c\x65\
      \x6e\x67\x65\x57\x65\x65\x6b\x6c\x79\x46\x44\x53\x74\x6f\x70\x43\
      \x61\x70\x74\x75\x72\x65\x22\x20\x52\x65\x73\x65\x74\x3d\x22\x4e\
      \x65\x76\x65\x72\x22\x20\x47\x75\x69\x64\x3d\x22\x33\x30\x31\x31\
      \x38\x22\x20\x53\x68\x6f\x72\x74\x44\x65\x73\x63\x3d\x22\x4c\x6f\
      \x63\x49\x64\x5f\x53\x4e\x53\x44\x5f\x4f\x6e\x6c\x69\x6e\x65\x43\
      \x68\x61\x6c\x6c\x65\x6e\x67\x65\x57\x65\x65\x6b\x6c\x79\x22\x20\
      \x49\x63\x6f\x6e\x3d\x22\x61\x73\x73\x61\x75\x6c\x74\x22\x20\x43\
      \x61\x74\x65\x67\x6f\x72\x79\x3d\x22\x4f\x6e\x6c\x69\x6e\x65\x43\
      \x68\x61\x6c\x6c\x65\x6e\x67\x65\x57\x65\x65\x6b\x6c\x79\x22\x20\
      \x4d\x61\x70\x3d\x22\x41\x30\x31\x22\x3e\x3c\x44\x65\x66\x69\x6e\
      \x69\x74\x69\x6f\x6e\x3e\x3c\x47\x61\x6d\x65\x45\x76\x65\x6e\x74\
      \x3e\x3c\x45\x76\x65\x6e\x74\x3e\x3c\x4f\x62\x6a\x65\x63\x74\x69\
      \x76\x65\x52\x65\x77\x61\x72\x64\x3e\x3c\x49\x73\x4e\x65\x75\x74\
      \x72\x61\x6c\x69\x7a\x65\x54\x65\x72\x72\x69\x74\x6f\x72\x79\x20\
      \x4f\x70\x3d\x22\x45\x71\x75\x61\x6c\x22\x20\x56\x61\x6c\x75\x65\
      \x3d\x22\x74\x72\x75\x65\x22\x20\x2f\x3e\x3c\x2f\x4f\x62\x6a\x65\
      \x63\x74\x69\x76\x65\x52\x65\x77\x61\x72\x64\x3e\x3c\x2f\x45\x76\
      \x65\x6e\x74\x3e\x3c\x2f\x47\x61\x6d\x65\x45\x76\x65\x6e\x74\x3e\
      \x3c\x2f\x44\x65\x66\x69\x6e\x69\x74\x69\x6f\x6e\x3e\x3c\x53\x74\
      \x65\x70\x52\x65\x77\x61\x72\x64\x20\x43\x6f\x75\x6e\x74\x3d\x22\
      \x35\x30\x22\x3e\x3c\x45\x63\x6f\x6e\x6f\x6d\x79\x20\x56\x61\x6c\
      \x75\x65\x54\x61\x67\x3d\x22\x57\x65\x65\x6b\x6c\x79\x4c\x61\x72\
      \x67\x65\x22\x20\x2f\x3e\x3c\x2f\x53\x74\x65\x70\x52\x65\x77\x61\
      \x72\x64\x3e\x3c\x2f\x43\x68\x61\x6c\x6c\x65\x6e\x67\x65\x3e\x00\
      \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
      \x00\x00\x90\x2a\x96\x1f\x00\x00\x00\x00\x90\x38\x96\x1f\x00\x00\
      \x00\x03\x00\x7b\x7d\x00\x03\x00\x7b\x7d\x00\x03\x00\x7b\x7d\x00\
      \x00\x00\x00\
      \x00\x02\x00\x00\x00\x02\x00\x70\x00\x01\x32\x00\x00\x00\x00\x00\
      \x00\x00\x02\x00\x73\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\
      \x00\x00\x00\x02\x00\x00\x00\xfb\x7e\x3f\x3f\x9c\x00\x00\x00\x00\
      \x00\x00\x00";

        let parsed: GetChallengesResponse = FromStream::from_bytes(data).unwrap();
        println!("{parsed:#?}");
    }
}
