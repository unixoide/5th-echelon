#![deny(clippy::pedantic)]

use quazal::Config;
use quazal::Context;
use quazal::OnlineConfig;
use quazal::Service;

fn main() {
    let mut config = Config::default();
    {
        let mut ctx = Context::splinter_cell_blacklist();
        ctx.secure_server_addr = Some(ctx.listen);
        ctx.secure_server_addr.as_mut().unwrap().set_port(ctx.listen.port() + 1);
        config.service.insert(String::from("sc_bl_auth"), Service::Authentication(ctx));
    }
    {
        let mut ctx = Context::splinter_cell_blacklist();
        ctx.listen.set_port(ctx.listen.port() + 1);
        config.service.insert(String::from("sc_bl_secure"), Service::Secure(ctx));
    }
    {
        let cfg = OnlineConfig::default();
        config.service.insert(String::from("onlineconfig"), Service::Config(cfg));
    }

    config.services.insert(String::from("sc_bl_auth"));
    config.services.insert(String::from("sc_bl_secure"));
    config.services.insert(String::from("onlineconfig"));
    config.save_to_file("service.toml").unwrap();
}
