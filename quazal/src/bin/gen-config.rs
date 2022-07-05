use quazal::{Config, Context, OnlineConfig, Service};

fn main() {
    let mut config = Config::default();
    {
        let mut ctx = Context::splinter_cell_blacklist();
        ctx.secure_server_addr = Some(ctx.listen);
        ctx.secure_server_addr
            .as_mut()
            .unwrap()
            .set_port(ctx.listen.port() + 1);
        config
            .service
            .insert("sc_bl_auth", Service::Authentication(ctx));
    }
    {
        let mut ctx = Context::splinter_cell_blacklist();
        ctx.listen.set_port(ctx.listen.port() + 1);
        config.service.insert("sc_bl_secure", Service::Secure(ctx));
    }
    {
        let cfg = OnlineConfig::default();
        config.service.insert("onlineconfig", Service::Config(cfg));
    }

    config.services.insert("sc_bl_auth");
    config.services.insert("sc_bl_secure");
    config.services.insert("onlineconfig");
    config.save_to_file("service.toml").unwrap();
}
