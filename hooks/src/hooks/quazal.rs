use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::OnceLock;

use hooks_config::Config;
use hooks_config::Hook;
use retour::static_detour;
use tracing::info;
use tracing::instrument;

use crate::addresses::Addresses;

struct RmcMessage {
    protocol_id: u32,
    method_id: u32,
}

static RMC_MESSAGES: OnceLock<Mutex<HashMap<usize, RmcMessage>>> = OnceLock::new();

const RMC_MAP_STR: &str = include_str!("../../../tools/rmc.txt");
type MethodMap = HashMap<u32, &'static str>;
type ProtocolMap = HashMap<u32, (&'static str, MethodMap)>;
static RMC_MAPS: OnceLock<ProtocolMap> = OnceLock::new();

fn parse_rmc_str() -> ProtocolMap {
    RMC_MAP_STR
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split(' ');
            let protocol_id: u32 = iter.next().unwrap().parse().unwrap();
            let method_id: u32 = iter.next().unwrap().parse().unwrap();
            let protocol_name = iter.next().unwrap();
            let method_name = iter.next().unwrap();

            (protocol_id, protocol_name, method_id, method_name)
        })
        .fold(
            HashMap::new(),
            |mut acc, (protocol_id, protocol_name, method_id, method_name)| {
                let (_, meth_map) = acc.entry(protocol_id).or_insert((protocol_name, HashMap::new()));
                meth_map.insert(method_id, method_name);
                acc
            },
        )
}

fn resolve_protocol_method(protocol_id: u32, method_id: u32) -> String {
    let map = RMC_MAPS.get_or_init(parse_rmc_str);
    let Some((proto_name, meth_map)) = map.get(&protocol_id) else {
        return format!("{protocol_id}.{method_id}");
    };
    let Some(meth_name) = meth_map.get(&method_id) else {
        return format!("{proto_name}.{method_id}");
    };
    format!("{proto_name}.{meth_name}")
}

impl RmcMessage {
    fn get_for_pointer(ptr: *const c_void) -> std::sync::MappedMutexGuard<'static, RmcMessage> {
        let map_mutx = RMC_MESSAGES.get_or_init(|| Mutex::new(HashMap::new()));
        std::sync::MutexGuard::map(map_mutx.lock().unwrap(), |map: &mut HashMap<usize, RmcMessage>| {
            map.entry(ptr as usize).or_insert(RmcMessage {
                protocol_id: 0,
                method_id: 0,
            })
        })
    }

    fn remove_from_pointer(ptr: *const c_void) {
        let map_mutx = RMC_MESSAGES.get_or_init(|| Mutex::new(HashMap::new()));
        map_mutx.lock().unwrap().remove(&(ptr as usize));
    }
}

static_detour! {
    static InitMessageHook:  extern "cdecl" fn(*mut c_void, u32, u32);
    static AddMethodIDHook:  extern "cdecl" fn(*mut c_void, u32);
    static SendRMCMessageHook:  extern "thiscall" fn(*mut c_void, *mut c_void, *mut c_void) -> usize;
}

fn init_message_hook(message: *mut c_void, protocol_id: u32, x: u32) {
    {
        let mut rmc_msg = RmcMessage::get_for_pointer(message.cast_const());
        rmc_msg.protocol_id = protocol_id;
    }
    InitMessageHook.call(message, protocol_id, x);
}

fn add_method_id_hook(message: *mut c_void, method_id: u32) {
    {
        let mut rmc_msg = RmcMessage::get_for_pointer(message.cast_const());
        rmc_msg.method_id = method_id;
    }
    AddMethodIDHook.call(message, method_id);
}

#[instrument]
fn send_rmc_message_hook(protocol: *mut c_void, call_ctx: *mut c_void, message: *mut c_void) -> usize {
    {
        let rmc_msg = RmcMessage::get_for_pointer(message.cast_const());
        info!(
            "Sending rmc message: {}",
            resolve_protocol_method(rmc_msg.protocol_id, rmc_msg.method_id)
        );
        drop(rmc_msg);
        RmcMessage::remove_from_pointer(message.cast_const());
    }
    SendRMCMessageHook.call(protocol, call_ctx, message)
}

pub unsafe fn init_hooks(config: &Config, addr: &Addresses) {
    super::configurable_hook!(config, Hook::RMCMessages, InitMessageHook; addr.func_rmc_init_message => init_message_hook);
    super::configurable_hook!(config, Hook::RMCMessages, AddMethodIDHook; addr.func_rmc_add_method_id => add_method_id_hook);
    super::configurable_hook!(config, Hook::RMCMessages, SendRMCMessageHook; addr.func_rmc_send_message => send_rmc_message_hook);
}

pub unsafe fn deinit_hooks(config: &Config) {
    super::disable_configurable_hook!(config, Hook::RMCMessages, InitMessageHook);
    super::disable_configurable_hook!(config, Hook::RMCMessages, AddMethodIDHook);
    super::disable_configurable_hook!(config, Hook::RMCMessages, SendRMCMessageHook);
}
