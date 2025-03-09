use quazal::prudp::ClientRegistry;
use quazal::rmc::basic::ToStream;
use quazal::rmc::types::DateTime;
use quazal::rmc::Protocol;
use quazal::Context;
use serde::Deserialize;

use crate::login_required;

#[derive(Debug, ToStream, FromStream, Default, Deserialize)]
struct NewsItem {
    maybe_id: u32,
    unk2: u32,
    unk3: u32,
    unk4: u32,
    unk5: String,
    unk6: DateTime,
    unk7: DateTime,
    expiration_time: DateTime,
    title: String,
    link: String,
    description: String,
}

#[allow(clippy::module_name_repetitions)]
pub struct OverlordNewsProtocol;

impl<T> Protocol<T> for OverlordNewsProtocol {
    fn id(&self) -> u16 {
        5002
    }

    fn name(&self) -> String {
        "OverlordNewsProtocol".into()
    }

    fn num_methods(&self) -> u32 {
        2
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
                let news: Vec<NewsItem> = std::fs::File::open("data/news.json")
                    .ok()
                    .map(serde_json::from_reader)
                    .and_then(Result::ok)
                    .unwrap_or(vec![NewsItem {
                        maybe_id: 19_5389,
                        unk2: 9,
                        unk3: 2,
                        unk4: 2,
                        title: String::from("WELCOME BACK!"),
                        description: String::from("5th Echelon is here!"),
                        link: String::from("https://github.com/unixoide/5th-echelon"),
                        unk5: String::from("Quazal Rendez-Vous"),
                        ..Default::default()
                    }]);
                Ok(news.to_bytes())
            }
            2 => {
                error!(logger, "not implemented yet");
                Err(quazal::rmc::Error::UnknownMethod)
            }
            _ => Err(quazal::rmc::Error::UnknownMethod),
        }
    }

    fn method_name(&self, method_id: u32) -> Option<String> {
        if method_id == 1 {
            Some("get_news".into())
        } else {
            None
        }
    }
}

pub fn new_protocol<T: 'static>() -> Box<dyn Protocol<T>> {
    Box::new(OverlordNewsProtocol)
}

#[cfg(test)]
mod tests {
    use quazal::rmc::basic::FromStream;

    use super::*;
    #[test]
    fn parse_sample() {
        let data = b"\x03\x00\x00\x00\x3c\xfb\x02\x00\x09\x00\x00\x00\x02\x00\x00\x00\
            \x02\x00\x00\x00\x13\x00\x51\x75\x61\x7a\x61\x6c\x20\x52\x65\x6e\
            \x64\x65\x7a\x2d\x56\x6f\x75\x73\x00\x00\xf0\x86\x75\x1f\x00\x00\
            \x00\xa7\x7b\x33\x96\x1f\x00\x00\x00\x20\x9a\x32\x9a\x1f\x00\x00\
            \x00\x11\x00\x43\x6f\x6d\x6d\x75\x6e\x69\x74\x79\x2d\x50\x6f\x72\
            \x74\x61\x6c\x00\x01\x00\x00\xe2\x00\x41\x75\x66\x20\x75\x6e\x73\
            \x65\x72\x65\x72\x20\x57\x65\x62\x73\x69\x74\x65\x20\x73\x68\x61\
            \x64\x6f\x77\x6e\x65\x74\x2e\x73\x70\x6c\x69\x6e\x74\x65\x72\x63\
            \x65\x6c\x6c\x2e\x63\x6f\x6d\x20\x66\x69\x6e\x64\x65\x73\x74\x20\
            \x64\x75\x20\x61\x6b\x74\x75\x65\x6c\x6c\x65\x20\x53\x74\x61\x74\
            \x69\x73\x74\x69\x6b\x65\x6e\x2c\x20\x6b\x61\x6e\x6e\x73\x74\x20\
            \x64\x69\x63\x68\x20\x65\x69\x6e\x65\x72\x20\x53\x70\x6c\x69\x6e\
            \x74\x65\x72\x20\x43\x65\x6c\x6c\x20\x61\x6e\x73\x63\x68\x6c\x69\
            \x65\xc3\x9f\x65\x6e\x20\x75\x6e\x64\x20\x76\x69\x65\x6c\x65\x73\
            \x20\x6d\x65\x68\x72\x21\x20\x53\x63\x68\x61\x75\x20\x6e\x61\x63\
            \x68\x2c\x20\x77\x69\x65\x20\x64\x75\x20\x69\x6d\x20\x56\x65\x72\
            \x67\x6c\x65\x69\x63\x68\x20\x7a\x75\x20\x64\x65\x69\x6e\x65\x6e\
            \x20\x46\x72\x65\x75\x6e\x64\x65\x6e\x20\x64\x61\x73\x74\x65\x68\
            \x73\x74\x20\x75\x6e\x64\x20\x66\x6f\x72\x64\x65\x72\x65\x20\x73\
            \x69\x65\x20\x68\x65\x72\x61\x75\x73\x21\x00\x3d\xfb\x02\x00\x09\
            \x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x13\x00\x51\x75\x61\
            \x7a\x61\x6c\x20\x52\x65\x6e\x64\x65\x7a\x2d\x56\x6f\x75\x73\x00\
            \x00\xf0\x86\x75\x1f\x00\x00\x00\xa7\x7b\x33\x96\x1f\x00\x00\x00\
            \x20\x9a\x32\x9a\x1f\x00\x00\x00\x1c\x00\x49\x6e\x66\x69\x6c\x74\
            \x72\x61\x74\x69\x6f\x6e\x20\x61\x6c\x73\x20\x53\x70\x69\x64\x65\
            \x72\x2d\x42\x6f\x74\x00\x01\x00\x00\xb2\x00\x4d\x69\x74\x20\x75\
            \x6e\x73\x65\x72\x65\x6d\x20\x53\x70\x69\x65\x6c\x20\x66\xc3\xbc\
            \x72\x20\x4d\x6f\x62\x69\x6c\x2d\x47\x65\x72\xc3\xa4\x74\x65\x20\
            \x62\x69\x73\x74\x20\x64\x75\x20\x6d\x69\x74\x20\x64\x65\x72\x20\
            \x53\x70\x6c\x69\x6e\x74\x65\x72\x2d\x43\x65\x6c\x6c\x2d\x57\x65\
            \x6c\x74\x20\x76\x65\x72\x62\x75\x6e\x64\x65\x6e\x20\x77\x69\x65\
            \x20\x6e\x6f\x63\x68\x20\x6e\x69\x65\x6d\x61\x6c\x73\x20\x7a\x75\
            \x76\x6f\x72\x21\x20\x49\x6e\x66\x69\x6c\x74\x72\x69\x65\x72\x65\
            \x2c\x20\x62\x65\x73\x6f\x72\x67\x65\x20\x49\x6e\x66\x6f\x72\x6d\
            \x61\x74\x69\x6f\x6e\x65\x6e\x2c\x20\x75\x6e\x64\x20\xc3\xbc\x62\
            \x65\x72\x6c\x65\x62\x65\x20\x61\x6c\x73\x20\x53\x61\x6d\x73\x20\
            \x62\x65\x73\x74\x65\x20\x57\x61\x66\x66\x65\x2e\x00\x3e\xfb\x02\
            \x00\x09\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x13\x00\x51\
            \x75\x61\x7a\x61\x6c\x20\x52\x65\x6e\x64\x65\x7a\x2d\x56\x6f\x75\
            \x73\x00\x00\xf0\x86\x75\x1f\x00\x00\x00\xa7\x7b\x33\x96\x1f\x00\
            \x00\x00\x20\x9a\x32\x9a\x1f\x00\x00\x00\x17\x00\x48\x4f\x43\x48\
            \x4c\x45\x49\x53\x54\x55\x4e\x47\x53\x53\x45\x54\x20\x28\x44\x4c\
            \x43\x29\x00\x2c\x00\x68\x74\x74\x70\x3a\x2f\x2f\x72\x73\x73\x2e\
            \x75\x62\x69\x2e\x63\x6f\x6d\x2f\x69\x6e\x67\x61\x6d\x65\x2f\x73\
            \x63\x36\x2f\x70\x63\x2f\x64\x65\x2d\x44\x45\x2f\x55\x4c\x43\x31\
            \x00\x9a\x00\x42\x72\x61\x75\x63\x68\x73\x74\x20\x64\x75\x20\x57\
            \x61\x66\x66\x65\x6e\x2c\x20\x64\x69\x65\x20\x6d\x65\x68\x72\x20\
            \x6c\x65\x69\x73\x74\x65\x6e\x3f\x20\x48\x6f\x6c\x20\x64\x69\x72\
            \x20\x64\x61\x73\x20\x48\x4f\x43\x48\x4c\x45\x49\x53\x54\x55\x4e\
            \x47\x53\x53\x45\x54\x2c\x20\x64\x61\x73\x73\x20\x64\x69\x65\x20\
            \x69\x6d\x20\x45\x69\x6e\x7a\x65\x6c\x73\x70\x69\x65\x6c\x65\x72\
            \x2c\x20\x4b\x4f\x4f\x50\x20\x75\x6e\x64\x20\x62\x65\x69\x20\x53\
            \x70\x69\x6f\x6e\x65\x20\x67\x67\x2e\x20\x53\xc3\xb6\x6c\x64\x6e\
            \x65\x72\x20\x68\x69\x6c\x66\x74\x2e\x20\x4a\x65\x74\x7a\x74\x20\
            \x65\x72\x68\xc3\xa4\x6c\x74\x6c\x69\x63\x68\x21\x00";

        let parsed: Vec<NewsItem> = FromStream::from_bytes(data).unwrap();
        println!("{parsed:#?}");
    }
}
