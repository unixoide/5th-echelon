local PROTO_MAPPING = {}

local path_ext = "/rmc.txt"
-- windows check
if string.find(Dir.personal_plugins_path(), "\\") then
    path_ext = "\\rmc.txt"
end
for line in io.lines(Dir.personal_plugins_path() .. path_ext) do
    local parts = {}
    for m in line:gmatch("%w+") do
        parts[#parts + 1] = m
    end
    local pid = tonumber(parts[1])
    local mid = tonumber(parts[2])
    if PROTO_MAPPING[pid] == nil then
        PROTO_MAPPING[pid] = { name = parts[3], methods = {} }
    end
    PROTO_MAPPING[pid].methods[mid] = parts[4]
end

local quazal_proto = Proto("prudp", "Pretty Reliable UDP (Quazal)")
local vport_proto = Proto("vport", "PRUDP VPort")

local stream_types = {
    [1] = "DO",
    [2] = "RV",
    [3] = "RVSec",
    [4] = "SBMGMT",
    [5] = "NAT",
    [6] = "SessionDiscovery",
    [7] = "NATEcho",
    [8] = "Routing",
}

vport_proto.fields.port = ProtoField.uint8("vport.port", "Port", base.DEC, nil, 0xf)
vport_proto.fields.type = ProtoField.uint8("vport.type", "StreamType", base.DEC, stream_types, 0xf0)

quazal_proto.fields.src = ProtoField.protocol("prudp.src", "Source")
quazal_proto.fields.dst = ProtoField.protocol("prudp.dst", "Destination")
quazal_proto.fields.flags = ProtoField.uint8("prudp.flags", "Flags", base.HEX, nil, 0xF8)
quazal_proto.fields.flags_ack = ProtoField.bool("prudp.flags.ack", "Ack", 8, nil, bit.lshift(0x1, 3))
quazal_proto.fields.flags_reliable = ProtoField.bool("prudp.flags.reliable", "Reliable", 8, nil, bit.lshift(0x2, 3))
quazal_proto.fields.flags_needack = ProtoField.bool("prudp.flags.need_ack", "Need Ack", 8, nil, bit.lshift(0x4, 3))
quazal_proto.fields.flags_hassize = ProtoField.bool("prudp.flags.has_size", "Has Size", 8, nil, bit.lshift(0x8, 3))
quazal_proto.fields.session_id = ProtoField.uint8("prudp.session_id", "Session ID", base.HEX)
quazal_proto.fields.signature = ProtoField.uint32("prudp.signature", "Signature", base.HEX)
quazal_proto.fields.sequence = ProtoField.uint16("prudp.sequence", "Sequence", base.DEC)
quazal_proto.fields.conn_signature = ProtoField.uint32("prudp.conn_signature", "Connection Signature", base.HEX)
quazal_proto.fields.fragment_id = ProtoField.uint8("prudp.fragment_id", "Fragment ID", base.DEC)
quazal_proto.fields.size = ProtoField.uint16("prudp.payload_size", "Payload size", base.DEC)
quazal_proto.fields.payload = ProtoField.bytes("prudp.payload", "Payload")
quazal_proto.fields.checksum = ProtoField.uint8("prudp.checksum", "Checksum", base.HEX)
quazal_proto.fields.decompressed = ProtoField.bytes("prudp.decompressed", "Decompressed")
-- quazal_proto.experts.decompressed = ProtoExpert.new("prudp.decompressed", "Decompressed", expert.group.UNDECODED,
--     expert.severity.NOTE)

local packet_types = {
    [0] = "Syn",
    [1] = "Connect",
    [2] = "Data",
    [3] = "Disconnect",
    [4] = "Ping",
    [5] = "User",
    [6] = "Route",
    [7] = "Raw",
}
quazal_proto.fields.type = ProtoField.uint8("prudp.type", "Type", base.DEC, packet_types, 0x07)


local rmc_proto = Proto("RMC", "Quazal RMC")
rmc_proto.fields.size = ProtoField.uint32("rmc.size", "Size")
rmc_proto.fields.prot_id_short = ProtoField.uint8("rmc.protocol_id", "Procotol ID", base.DEC, nil, 0x7F)
rmc_proto.fields.prot_id_long = ProtoField.uint16("rmc.protocol_id", "Procotol ID")
rmc_proto.fields.is_request = ProtoField.bool("rmc.is_request", "Is Request", 8, nil, 0x80)
rmc_proto.fields.call_id = ProtoField.uint32("rmc.call_id", "Call")
rmc_proto.fields.method_id = ProtoField.uint32("rmc.method_id", "Method", base.DEC, nil, 0x7FFF)
rmc_proto.fields.is_success = ProtoField.bool("rmc.is_success", "Is Success", 8, nil, 1)
rmc_proto.fields.payload = ProtoField.bytes("rmc.payload", "Payload")
rmc_proto.fields.error_code = ProtoField.uint32("rmc.error_code", "Error Code", base.HEX)

local rmc_dissector_table = DissectorTable.new("rmc.protocol_id", nil, ftypes.UINT16, nil, rmc_proto)

local method_id_field = Field.new("rmc.method_id")
local is_request_field = Field.new("rmc.is_request")
local is_success_field = Field.new("rmc.is_success")

local function rmc_proto_dissector(buffer, pinfo, tree)
    pinfo.cols.protocol = "RMC"
    local subtree = tree:add(rmc_proto, buffer(), "RMC Data")
    subtree:add_le(rmc_proto.fields.size, buffer(0, 4))
    local size = buffer(0, 4):le_uint()
    local off = 4
    local is_request = true
    local proto = 0
    local pt = nil
    if buffer(4, 1):uint() == 0x7f or buffer(4, 1):uint() == 0xff then
        pt = subtree:add_le(rmc_proto.fields.prot_id_long, buffer(5, 2))
        is_request = buffer(4, 1):bitfield(0, 1) ~= 0
        off = 7
        proto = buffer(5, 2):le_uint()
    else
        pt = subtree:add_le(rmc_proto.fields.prot_id_short, buffer(4, 1))
        subtree:add_le(rmc_proto.fields.is_request, buffer(4, 1))
        is_request = buffer(4, 1):bitfield(0, 1) ~= 0
        off = 5
        proto = bit.band(buffer(4, 1):le_uint(), 0x7F)
    end

    if is_request then
        pinfo.cols.info = "Request"
    else
        pinfo.cols.info = "Response"
    end

    if PROTO_MAPPING[proto] then
        pt:append_text(" " .. PROTO_MAPPING[proto].name)
        pinfo.cols.info:append(" " .. PROTO_MAPPING[proto].name)
    else
        pinfo.cols.info:append(" Proto(" .. proto .. ")")
    end

    if is_request then
        subtree:add_le(rmc_proto.fields.call_id, buffer(off, 4))
        pinfo.cols.info:append(" Call(" .. buffer(off, 4):le_uint() .. ")")
        off = off + 4
        local mt = subtree:add_le(rmc_proto.fields.method_id, buffer(off, 4))
        local mid = buffer(off, 4):le_uint()
        off = off + 4
        if PROTO_MAPPING[proto] and PROTO_MAPPING[proto].methods[mid] then
            mt:append_text(" " .. PROTO_MAPPING[proto].methods[mid])
            pinfo.cols.info:append(" " .. PROTO_MAPPING[proto].methods[mid])
        else
            pinfo.cols.info:append(" Method(" .. mid .. ")")
        end
    else
        subtree:add_le(rmc_proto.fields.is_success, buffer(off, 1))
        local is_success = buffer(off, 1):bitfield(7, 1) ~= 0
        off = off + 1
        if is_success then
            subtree:add_le(rmc_proto.fields.call_id, buffer(off, 4))
            pinfo.cols.info:append(" Call(" .. buffer(off, 4):le_uint() .. ")")
            off = off + 4
            local mt = subtree:add_le(rmc_proto.fields.method_id, buffer(off, 4))
            local mid = bit.band(buffer(off, 4):le_uint(), 0x7FFF)
            off = off + 4
            if PROTO_MAPPING[proto] and PROTO_MAPPING[proto].methods[mid] then
                mt:append_text(" " .. PROTO_MAPPING[proto].methods[mid])
                pinfo.cols.info:append(" " .. PROTO_MAPPING[proto].methods[mid])
            else
                pinfo.cols.info:append(" Method(" .. mid .. ")")
            end
        else
            subtree:add_le(rmc_proto.fields.error_code, buffer(off, 4))
            pinfo.cols.info:append(" Error(" .. buffer(off, 4):le_uint() .. ")")
            off = off + 4
            subtree:add_le(rmc_proto.fields.call_id, buffer(off, 4))
            pinfo.cols.info:append(" Call(" .. buffer(off, 4):le_uint() .. ")")
            off = off + 4
        end
    end
    local payload = buffer(off, size - off + 4)
    subtree:add(rmc_proto.fields.payload, payload)
    method_id_field()
    rmc_dissector_table:try(proto, payload:tvb("RMC Payload"), pinfo, tree)
    off = size + 4
    if buffer:len() > off then
        rmc_proto_dissector(buffer(off), pinfo, tree)
    end
end

local stream_index = Field.new("udp.stream")
local ip_src = Field.new("ip.src")
local udp_port = Field.new("udp.srcport")
local fragments = {}

local do_message_types = {
    [0] = "JoinRequest",
    [1] = "JoinResponse",
    [2] = "Update",
    [3] = nil,
    [4] = "Delete",
    [5] = "Action",
    [6] = nil,
    [7] = nil,
    [8] = "CallOutcome",
    [9] = nil,
    [10] = "RMCCall",
    [11] = "RMCResponse",
    [12] = nil,
    [13] = "FetchRequest",
    [14] = nil,
    [15] = "Bundle",
    [16] = nil,
    [17] = "Migration",
    [18] = "CreateDuplica",
    [19] = "CreateAndPromoteDuplica",
    [20] = "GetParticipantsRequest",
    [21] = "GetParticipantsResponse",
    [254] = "Empty",
    [255] = "EOS",
}
local do_proto = Proto("DO", "Quazal DO")
do_proto.fields.size = ProtoField.uint32("do.size", "Size")
do_proto.fields.message_id = ProtoField.uint8("do.message_id", "Message", base.DEC, do_message_types)
local message_id_field = Field.new("do.message_id")

-- Reads `Quazal::String`.
local function read_string(buffer) 
    local size = buffer(0, 2):le_uint()
    local off = 2
    local string = buffer(off, size):string()
    return string, off + size
end

-- JoinRequest definition
do_proto.fields.join_request_process_auth = ProtoField.none("do.join_request.process_auth", "ProcessAuthentication")
do_proto.fields.join_request_process_auth_struct_ver = ProtoField.uint8("do.join_request.process_auth.struct_ver", "Structure version")
do_proto.fields.join_request_process_auth_lib_ver = ProtoField.uint8("do.join_request.process_auth.lib_ver", "Library protocol version")
do_proto.fields.join_request_process_auth_unknown_ver = ProtoField.uint8("do.join_request.process_auth.unknown_ver", "Unknown version")
do_proto.fields.join_request_process_major_ver = ProtoField.uint32("do.join_request.process_auth.major_ver", "Major version", base.HEX)
do_proto.fields.join_request_process_minor_ver = ProtoField.uint32("do.join_request.process_auth.minor_ver", "Minor version", base.HEX)
do_proto.fields.join_request_process_title_checksum = ProtoField.uint32("do.join_request.process_auth.title_checksum", "Title checksum", base.HEX)
do_proto.fields.join_request_process_proto_flags = ProtoField.uint32("do.join_request.process_auth.proto_flags", "Protocol flags", base.HEX)
do_proto.fields.join_request_station_ident = ProtoField.none("do.join_request.station_ident", "StationIdentification")
do_proto.fields.join_request_station_ident_token = ProtoField.string("do.join_request.station_ident.token", "Identification token")
do_proto.fields.join_request_station_ident_process_name = ProtoField.string("do.join_request.station_ident.process_name", "Process name")
do_proto.fields.join_request_station_ident_process_type = ProtoField.uint32("do.join_request.station_ident.process_type", "Process type")
do_proto.fields.join_request_station_ident_product_ver = ProtoField.uint32("do.join_request.station_ident.product_ver", "Product version")
local function do_parse_join_request(buffer, pinfo, tree, subtree)
    -- `Quazal::ProcessAuthentication`
    local pa = subtree:add(do_proto.fields.join_request_process_auth, buffer(0, 19))
    local off = 0
    pa:add_le(do_proto.fields.join_request_process_auth_struct_ver, buffer(off, 1))
    off = off + 1
    pa:add_le(do_proto.fields.join_request_process_auth_lib_ver, buffer(off, 1))
    off = off + 1
    pa:add_le(do_proto.fields.join_request_process_auth_unknown_ver, buffer(off, 1))
    off = off + 1
    pa:add_le(do_proto.fields.join_request_process_major_ver, buffer(off, 4))
    off = off + 4
    pa:add_le(do_proto.fields.join_request_process_minor_ver, buffer(off, 4))
    off = off + 4
    pa:add_le(do_proto.fields.join_request_process_title_checksum, buffer(off, 4))
    off = off + 4
    pa:add_le(do_proto.fields.join_request_process_proto_flags, buffer(off, 4))
    off = off + 4
    -- `Quazal::StationIdentification`
    local si = subtree:add(do_proto.fields.join_request_station_ident, buffer(off))
    local str, str_len = read_string(buffer(off))
    si:add(do_proto.fields.join_request_station_ident_token, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    si:add(do_proto.fields.join_request_station_ident_process_name, buffer(off, str_len), str)
    off = off + str_len
    si:add_le(do_proto.fields.join_request_station_ident_process_type, buffer(off, 4))
    off = off + 4
    si:add_le(do_proto.fields.join_request_station_ident_product_ver, buffer(off, 4))
    off = off + 4
    return off
end

-- JoinResponse definition
do_proto.fields.join_response_success = ProtoField.bool("do.join_response.success", "Success")
do_proto.fields.join_response_client_station_id = ProtoField.uint32("do.join_response.client_station_id", "Client station ID", base.HEX)
do_proto.fields.join_response_master_station_id = ProtoField.uint32("do.join_response.master_station_id", "Master station ID", base.HEX)
do_proto.fields.join_response_bootstrap_urls_count = ProtoField.uint16("do.join_response.bootstrap_urls_count", "Bootstrap URLs")
local function do_parse_join_response(buffer, pinfo, tree, subtree)
    local off = 0
    subtree:add_le(do_proto.fields.join_response_success, buffer(off, 1))
    off = off + 1
    subtree:add_le(do_proto.fields.join_response_client_station_id, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.join_response_master_station_id, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.join_response_bootstrap_urls_count, buffer(off, 2))
    off = off + 2
    -- TODO: read bootstrap URLs
    return off
end

local function do_parse_update(buffer, pinfo, tree, subtree)
    
end

local function do_parse_delete(buffer, pinfo, tree, subtree)
    
end

local function do_parse_action(buffer, pinfo, tree, subtree)
    
end

local function do_parse_call_outcome(buffer, pinfo, tree, subtree)
    
end

local function do_parse_rmc_call(buffer, pinfo, tree, subtree)
    
end

local function do_parse_rmc_response(buffer, pinfo, tree, subtree)
    
end

local function do_parse_fetch_request(buffer, pinfo, tree, subtree)
    
end

-- Bundle definition
do_proto.fields.bundle_msg = ProtoField.none("do.bundle.msg", "DO Message")
local function do_parse_bundle(buffer, pinfo, tree, subtree)
    local off = 0
    while off < buffer:len() do
        local subsubtree = subtree:add(do_proto.fields.bundle_msg, buffer())
        subsubtree:add_le(do_proto.fields.size, buffer(off, 4))
        local size = buffer(off, 4):le_uint()
        off = off + 4
        -- end of bundle
        if size == 0 then
            break
        end
        subsubtree:add_le(do_proto.fields.message_id, buffer(off, 1))
        local msg_id = buffer(off, 1):uint()
        local msg_name = do_message_types[msg_id]
        if msg_name == nil then
            msg_name = "Empty"
        end
        pinfo.cols.info:append(" " .. msg_name)
        local parser = DO_message_parsers[msg_name]
        off = off + 1
        if parser then
            off = off + parser(buffer(off), pinfo, subtree, subsubtree)
        end
    end
    return off
end

local function do_parse_migration(buffer, pinfo, tree, subtree)
    
end

local function do_parse_create_duplica(buffer, pinfo, tree, subtree)
    
end

-- CreateAndPromoteDuplica definition
do_proto.fields.create_promote_duplica_call_id = ProtoField.uint16("do.create_promote_duplica.call_id", "Call ID")
do_proto.fields.create_promote_duplica_client_station_id = ProtoField.uint32("do.create_promote_duplica.client_station_id", "Client station ID", base.HEX)
do_proto.fields.create_promote_duplica_master_station_id = ProtoField.uint32("do.create_promote_duplica.master_station_id", "Master station ID", base.HEX)
do_proto.fields.create_promote_duplica_version = ProtoField.uint8("do.create_promote_duplica.version", "Version")
do_proto.fields.create_promote_duplica_list = ProtoField.uint32("do.create_promote_duplica.list", "Duplicas")
do_proto.fields.create_promote_duplica_duplica = ProtoField.uint32("do.create_promote_duplica.duplica", "Duplica")
do_proto.fields.create_promote_duplica_discovery_msg = ProtoField.none("do.create_promote_duplica.discovery_msg", "DiscoveryMessage")
do_proto.fields.create_promote_duplica_connect_info = ProtoField.none("do.create_promote_duplica.connect_info", "DS_ConnectionInfo")
do_proto.fields.create_promote_duplica_connect_info_exists = ProtoField.bool("do.create_promote_duplica.connect_info.exists", "Exists")
do_proto.fields.create_promote_duplica_connect_info_url_init = ProtoField.bool("do.create_promote_duplica.connect_info.url_init", "URL initialized")
do_proto.fields.create_promote_duplica_connect_info_url1 = ProtoField.string("do.create_promote_duplica.connect_info.url1", "URL1")
do_proto.fields.create_promote_duplica_connect_info_url2 = ProtoField.string("do.create_promote_duplica.connect_info.url2", "URL2")
do_proto.fields.create_promote_duplica_connect_info_url3 = ProtoField.string("do.create_promote_duplica.connect_info.url3", "URL3")
do_proto.fields.create_promote_duplica_connect_info_url4 = ProtoField.string("do.create_promote_duplica.connect_info.url4", "URL4")
do_proto.fields.create_promote_duplica_connect_info_url5 = ProtoField.string("do.create_promote_duplica.connect_info.url5", "URL5")
do_proto.fields.create_promote_duplica_connect_info_in_bandwidth = ProtoField.uint32("do.create_promote_duplica.connect_info.in_bandwidth", "Input bandwidth")
do_proto.fields.create_promote_duplica_connect_info_in_latency = ProtoField.uint32("do.create_promote_duplica.connect_info.in_latency", "Input latency")
do_proto.fields.create_promote_duplica_connect_info_out_bandwidth = ProtoField.uint32("do.create_promote_duplica.connect_info.out_bandwidth", "Output bandwidth")
do_proto.fields.create_promote_duplica_connect_info_out_latency = ProtoField.uint32("do.create_promote_duplica.connect_info.out_latency", "Output latency")
do_proto.fields.create_promote_duplica_station_ident = ProtoField.none("do.create_promote_duplica.station_ident", "StationIdentification")
do_proto.fields.create_promote_duplica_station_ident_exists = ProtoField.bool("do.create_promote_duplica.station_ident.exists", "Exists")
do_proto.fields.create_promote_duplica_station_ident_token = ProtoField.string("do.create_promote_duplica.station_ident.token", "Identification token")
do_proto.fields.create_promote_duplica_station_ident_process_name = ProtoField.string("do.create_promote_duplica.station_ident.process_name", "Process name")
do_proto.fields.create_promote_duplica_station_ident_process_type = ProtoField.uint32("do.create_promote_duplica.station_ident.process_type", "Process type")
do_proto.fields.create_promote_duplica_station_ident_product_ver = ProtoField.uint32("do.create_promote_duplica.station_ident.product_ver", "Product version")
do_proto.fields.create_promote_duplica_station_info = ProtoField.none("do.create_promote_duplica.station_info", "StationInfo")
do_proto.fields.create_promote_duplica_station_info_exists = ProtoField.bool("do.create_promote_duplica.station_info.exists", "Exists")
do_proto.fields.create_promote_duplica_station_info_observer = ProtoField.uint32("do.create_promote_duplica.station_info.observer", "Observer")
do_proto.fields.create_promote_duplica_station_info_machine_uid = ProtoField.uint32("do.create_promote_duplica.station_info.machine_uid", "Machine UID")
do_proto.fields.create_promote_duplica_station_state = ProtoField.none("do.create_promote_duplica.station_state", "StationState")
do_proto.fields.create_promote_duplica_station_state_exists = ProtoField.bool("do.create_promote_duplica.station_state.exists", "Exists")
do_proto.fields.create_promote_duplica_station_state_state = ProtoField.uint16("do.create_promote_duplica.station_state.state", "Station state")
local function do_parse_create_and_promote_duplica(buffer, pinfo, tree, subtree)
    local off = 0
    -- header
    subtree:add_le(do_proto.fields.create_promote_duplica_call_id, buffer(off, 2))
    off = off + 2
    subtree:add_le(do_proto.fields.create_promote_duplica_client_station_id, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.create_promote_duplica_master_station_id, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.create_promote_duplica_version, buffer(off, 1))
    off = off + 1
    local duplicas = subtree:add_le(do_proto.fields.create_promote_duplica_list, buffer(off, 4))
    local duplica_count = buffer(off, 4):le_uint()
    off = off + 4
    for idx=1, duplica_count do
        duplicas:add_le(do_proto.fields.create_promote_duplica_duplica, buffer(off, 4))
        off = off + 4
    end
    -- `DS_ConnectionInfo`
    local connect_info = subtree:add(do_proto.fields.create_promote_duplica_connect_info, buffer(off))
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_exists, buffer(off, 1))
    off = off + 1
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_url_init, buffer(off, 1))
    off = off + 1
    local str, str_len = read_string(buffer(off))
    connect_info:add(do_proto.fields.create_promote_duplica_connect_info_url1, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    connect_info:add(do_proto.fields.create_promote_duplica_connect_info_url2, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    connect_info:add(do_proto.fields.create_promote_duplica_connect_info_url3, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    connect_info:add(do_proto.fields.create_promote_duplica_connect_info_url4, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    connect_info:add(do_proto.fields.create_promote_duplica_connect_info_url5, buffer(off, str_len), str)
    off = off + str_len
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_in_bandwidth, buffer(off, 4))
    off = off + 4
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_in_latency, buffer(off, 4))
    off = off + 4
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_out_bandwidth, buffer(off, 4))
    off = off + 4
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_out_latency, buffer(off, 4))
    off = off + 4
    -- `Quazal::StationIdentification`
    local station_ident = subtree:add(do_proto.fields.create_promote_duplica_station_ident, buffer(off))
    station_ident:add(do_proto.fields.create_promote_duplica_station_ident_exists, buffer(off, 1))
    off = off + 1
    str, str_len = read_string(buffer(off))
    station_ident:add(do_proto.fields.create_promote_duplica_station_ident_token, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    station_ident:add(do_proto.fields.create_promote_duplica_station_ident_process_name, buffer(off, str_len), str)
    off = off + str_len
    station_ident:add_le(do_proto.fields.create_promote_duplica_station_ident_process_type, buffer(off, 4))
    off = off + 4
    station_ident:add_le(do_proto.fields.create_promote_duplica_station_ident_product_ver, buffer(off, 4))
    off = off + 4
    -- `Quazal::StationInfo`
    local station_info = subtree:add(do_proto.fields.create_promote_duplica_station_info, buffer(off))
    station_info:add_le(do_proto.fields.create_promote_duplica_station_info_exists, buffer(off, 1))
    off = off + 1
    station_info:add_le(do_proto.fields.create_promote_duplica_station_info_observer, buffer(off, 4))
    off = off + 4
    station_info:add_le(do_proto.fields.create_promote_duplica_station_info_machine_uid, buffer(off, 4))
    off = off + 4
    -- `Quazal::StationState`
    local station_state = subtree:add(do_proto.fields.create_promote_duplica_station_state, buffer(off))
    station_state:add_le(do_proto.fields.create_promote_duplica_station_state_exists, buffer(off, 1))
    off = off + 1
    station_state:add_le(do_proto.fields.create_promote_duplica_station_state_state, buffer(off, 2))
    off = off + 2
    return off
end

-- GetParticipantsRequest definition
do_proto.fields.get_participants_request_urls = ProtoField.uint32("do.get_participants_request.urls", "Station URLs")
do_proto.fields.get_participants_request_url = ProtoField.string("do.get_participants_request.url", "URL")
local function do_parse_get_participants_request(buffer, pinfo, tree, subtree)
    local url_count = buffer(0, 4):le_uint()
    local urls = subtree:add_le(do_proto.fields.get_participants_request_urls, buffer(0, 4))
    local off = 4
    if url_count > 0 then
        for idx=1,url_count do
            local str, str_len = read_string(buffer(off))
            urls:add(do_proto.fields.get_participants_request_url, buffer(off, str_len), str)
            off = off + str_len
        end
    end
    return off
end

-- GetParticipantsResponse definition
do_proto.fields.get_participants_response_success = ProtoField.bool("do.get_participants_response.success", "Success")
do_proto.fields.get_participants_response_participants = ProtoField.uint32("do.get_participants_response.participants", "Participants")
do_proto.fields.get_participants_response_urls = ProtoField.uint32("do.get_participants_response.urls", "Station URLs")
do_proto.fields.get_participants_response_url = ProtoField.string("do.get_participants_response.url", "URL")
local function do_parse_get_participants_response(buffer, pinfo, tree, subtree)
    subtree:add_le(do_proto.fields.get_participants_response_success, buffer(0, 1))
    local off = 1
    local participants = subtree:add_le(do_proto.fields.get_participants_response_participants, buffer(off, 4))
    local participants_count = buffer(off, 4):le_uint()
    off = off + 4
    for idx=1, participants_count do
        local urls = participants:add_le(do_proto.fields.get_participants_response_urls, buffer(off, 4))
        local urls_count = buffer(off, 4):le_uint()
        off = off + 4
        for i=1, urls_count do
            local str, str_len = read_string(buffer(off))
            urls:add(do_proto.fields.get_participants_response_url, buffer(off, str_len), str)
            off = off + str_len
        end
    end
    return off
end

-- This message carries no payload.
local function do_parse_empty(buffer, pinfo, tree, subtree)
    return 0
end

local function do_parse_eos(buffer, pinfo, tree, subtree)
    
end

DO_message_parsers = {
    ["JoinRequest"] = do_parse_join_request,
    ["JoinResponse"] = do_parse_join_response,
    ["Update"] = do_parse_update,
    ["Delete"] = do_parse_delete,
    ["Action"] = do_parse_action,
    ["CallOutcome"] = do_parse_call_outcome,
    ["RMCCall"] = do_parse_rmc_call,
    ["RMCResponse"] = do_parse_rmc_response,
    ["FetchRequest"] = do_parse_fetch_request,
    ["Bundle"] = do_parse_bundle,
    ["Migration"] = do_parse_migration,
    ["CreateDuplica"] = do_parse_create_duplica,
    ["CreateAndPromoteDuplica"] = do_parse_create_and_promote_duplica,
    ["GetParticipantsRequest"] = do_parse_get_participants_request,
    ["GetParticipantsResponse"] = do_parse_get_participants_response,
    ["Empty"] = do_parse_empty,
    ["EOS"] = do_parse_eos,
}

local function do_proto_dissector(buffer, pinfo, tree)
    pinfo.cols.protocol = "DO"
    local subtree = tree:add(do_proto, buffer(), "DO Message")
    subtree:add_le(do_proto.fields.size, buffer(0, 4))
    local size = buffer(0, 4):le_uint()
    local off = 4
    local message = subtree:add_le(do_proto.fields.message_id, buffer(off, 1))
    local msg_id = buffer(off, 1):uint()
    local msg_name = do_message_types[msg_id]
    if msg_name == nil then
       msg_name = "Empty"
    end
    pinfo.cols.info:append(" " .. msg_name)
    local parser = DO_message_parsers[msg_name]
    off = off + 1
    if parser then
        off = off + parser(buffer(off), pinfo, tree, subtree)
    end
    if size > off then
        do_proto_dissector(buffer(off), pinfo, tree)
    end
end

function table.map_length(t)
    local c = 0
    for k, v in pairs(t) do
        c = c + 1
    end
    return c
end

-- https://github.com/kinnay/NintendoClients/blob/71fe4d65befb5a49874d09a711e1c121a641575e/nintendo/nex/prudp.py#L693-L711
-- SlidingWindow = { next = 2, packets = {} }


-- function SlidingWindow:new()
--     local o = {}
--     setmetatable(o, self)
--     self.__index = self
--     return o
-- end

-- function SlidingWindow:update(data, segment_id, fragment_id)
--     local res = {}
--     print("Next: ", self.next, " ", segment_id)
--     if segment_id >= self.next and self.packets[segment_id] == nil then
--         self.packets[segment_id] = { data = data, fragment_id = fragment_id, segment_id = segment_id }
--         while self.packets[self.next] ~= nil do
--             local packet = self.packets[self.next]
--             self.packets[self.next] = nil
--             self.next = bit.band(self.next + 1, 0xFFFF)
--             table.insert(res, packet)
--         end
--         if #res > 0 and res[#res - 1].fragment_id > 0 then
--             -- missing fragments
--             for i = #res - 1, 0 do
--                 local packet = res[i]
--                 if packet.fragment_id == 0 then
--                     break
--                 end
--                 self.packets[packet.segment_id] = packet
--                 self.next = packet.segment_id + 1
--                 res[i] = nil
--             end
--         end
--     end
--     print("Packets: ", #res)
--     return res
-- end

PacketCollector = { packets = {} }

function PacketCollector:new()
    local o = {}
    setmetatable(o, self)
    self.__index = self
    o.packets = {}
    return o
end

function PacketCollector:update(data, segment_id, fragment_id)
    if self.packets[segment_id] == nil then
        self.packets[segment_id] = { data = data, segment_id = segment_id, fragment_id = fragment_id }
    end

    -- print(segment_id, fragment_id)
    if fragment_id > 0 then
        return {}
    end

    local res = {}
    -- check for previous fragments
    local prev = segment_id - 1
    --    print("X", segment_id, prev, self.packets[prev].fragment_id)
    while self.packets[prev] ~= nil and self.packets[prev].fragment_id > 0 do
        local packet = self.packets[prev]
        res[packet.fragment_id] = packet
        prev = prev - 1
    end
    -- print(segment_id, fragment_id)
    res[#res + 1] = self.packets[segment_id]
    return res
end

local function vport(tree, buf)
    tree:add(vport_proto.fields.port, buf(0, 1))
    tree:add(vport_proto.fields.type, buf(0, 1))
end

function quazal_proto.init()
    fragments = {}
end

local function new_rc4(key)
    -- plain Lua implementation
    local function new_ks(key)
        local st = {}
        for i = 0, 255 do st[i] = i end

        local len = #key
        local j = 0
        for i = 0, 255 do
            j = (j + st[i] + key:byte((i % len) + 1)) % 256
            st[i], st[j] = st[j], st[i]
        end

        return { x = 0, y = 0, st = st }
    end

    local function rc4_crypt(ks, input)
        local x, y, st = ks.x, ks.y, ks.st

        local t = {}
        for i = 1, #input do
            x = (x + 1) % 256
            y = (y + st[x]) % 256;
            st[x], st[y] = st[y], st[x]
            t[i] = string.char(bit.bxor(input:byte(i), st[(st[x] + st[y]) % 256]))
        end

        ks.x, ks.y = x, y
        return table.concat(t)
    end

    local o = new_ks(key)
    return setmetatable(o, { __call = rc4_crypt, __metatable = false })
end

local function quazal_proto_dissector(buffer, pinfo, tree, fragments)
    pinfo.cols.protocol = "PRUDP"

    local subtree = tree:add(quazal_proto, buffer(), "PRUDP Data")
    vport(subtree:add_le(quazal_proto.fields.src, buffer(0, 1)), buffer(0, 1))
    vport(subtree:add_le(quazal_proto.fields.dst, buffer(1, 1)), buffer(1, 1))

    subtree:add(quazal_proto.fields.type, buffer(2, 1))
    local ftree = subtree:add_le(quazal_proto.fields.flags, buffer(2, 1))
    ftree:add(quazal_proto.fields.flags_ack, buffer(2, 1))
    ftree:add(quazal_proto.fields.flags_reliable, buffer(2, 1))
    ftree:add(quazal_proto.fields.flags_needack, buffer(2, 1))
    ftree:add(quazal_proto.fields.flags_hassize, buffer(2, 1))
    subtree:add(quazal_proto.fields.session_id, buffer(3, 1))
    subtree:add_le(quazal_proto.fields.signature, buffer(4, 4))
    subtree:add_le(quazal_proto.fields.sequence, buffer(8, 2))

    local has_size = buffer(2, 1):bitfield(1, 1) ~= 0
    local ack = buffer(2, 1):bitfield(4, 1) ~= 0
    local stype = stream_types[buffer(0, 1):bitfield(0, 4)]
    local ptype = packet_types[bit.band(buffer(2, 1):uint(), 0x07)]
    local sequence_id = buffer(8, 2):le_uint()
    local fragment_id = nil
    pinfo.cols.info = stype .. " " .. ptype
    if ack then
        pinfo.cols.info:append(" ACK")
    end
    local off = 10
    if ptype == "Syn" or ptype == "Connect" then
        subtree:add_le(quazal_proto.fields.conn_signature, buffer(off, 4))
        off = off + 4
    elseif ptype == "Data" then
        subtree:add_le(quazal_proto.fields.fragment_id, buffer(off, 1))
        fragment_id = buffer(off, 1):le_uint()
        off = off + 1
    end

    local size = 0
    if has_size then
        subtree:add_le(quazal_proto.fields.size, buffer(off, 2))
        size = buffer(off, 2):le_uint()
        off = off + 2
    else
        size = buffer(off):len() - 1
    end

    local payload = buffer(off, size)
    if size > 0 then
        subtree:add(quazal_proto.fields.payload, payload)
        off = off + size
    end
    subtree:add_le(quazal_proto.fields.checksum, buffer(off, 1))
    off = off + 1

    if ptype ~= "Syn" and stype == "RVSec" then
        local dec = new_rc4("CD&ML")
        payload = ByteArray.new(dec(payload:raw()), true):tvb("Decrypted")
        if payload:len() > 0 then
            local compressed = payload(0, 1):uint() ~= 0
            payload = payload(1)
            if compressed then
                payload = payload:uncompress("Decompressed")
            end
        end
    end

    -- DO decryption
    -- DO decompression
    if ptype ~= "Syn" and stype == "DO" then
        -- local dec = new_rc4("CD&ML")
        -- payload = ByteArray.new(dec(payload:raw()), true):tvb("Decrypted")
        if payload:len() > 0 then
            local compressed = payload(0, 1):uint() ~= 0
            payload = payload(1)
            if compressed then
                payload = payload:uncompress("Decompressed")
            end
        end
    end

    if payload:len() > 0 and ptype == "Data" and ack == false then
        --
        local packets = fragments:update(payload:bytes(), sequence_id, fragment_id)
        -- print(#packets)
        if #packets > 0 then
            local data = ByteArray.new()
            for i = 1, #packets do
                data:append(packets[i].data)
            end
            if stype == "RVSec" then
                rmc_proto_dissector(data:tvb("Reassembled"), pinfo, tree)
            elseif stype == "DO" then
                do_proto_dissector(data:tvb("Reassembled"), pinfo, tree)
            end
        end
    end

    if buffer:len() > off then
        quazal_proto_dissector(buffer(off), pinfo, tree, fragments)
    end
end

function quazal_proto.dissector(buffer, pinfo, tree)
    local si = tostring(stream_index().value)
    local is = tostring(ip_src().value)
    local up = tostring(udp_port().value)
    local key = si .. "|" .. is .. "|" .. up
    -- print("1", key, fragments[key])
    if fragments[key] == nil then
        -- print("2", key)
        fragments[key] = PacketCollector:new()
        -- print("P", #fragments[key].packets)
    end
    quazal_proto_dissector(buffer, pinfo, tree, fragments[key])
end

local udp_table = DissectorTable.get("udp.port")
udp_table:add(3074, quazal_proto) -- SC:BL
udp_table:add(2347, quazal_proto) -- GR:FS
udp_table:add(9103, quazal_proto) -- SC:C
udp_table:add(7917, quazal_proto) -- AC:B


-- local secure_connect_proto = Proto("SecureConnectionProtocol", "SecureConnectionProtocol")
-- secure_connect_proto.fields.vec_my_urls = ProtoField.stringz("secure_connect_proto.vec_my_urls", "vecMyURLs")

-- function secure_connect_proto.init()
--     DissectorTable.get("rmc.protocol_id"):add(11, secure_connect_proto)
-- end

-- --- @param buffer Tvb
-- --- @param pinfo Pinfo
-- --- @param tree TreeItem
-- function secure_connect_proto.dissector(buffer, pinfo, tree)
--     local method_id = method_id_field().value
--     local is_request = is_request_field().value

--     local subtree = nil
--     if is_request then
--         if method_id == 4 then
--             subtree = tree:add(quazal_proto, buffer(), "SecureConnectionProtocol.RegisterEx")
--             local cnt = buffer(0, 4):le_uint()
--             local off = 4
--             for i = 1, cnt do
--                 local len = buffer(off, 1):uint()
--                 off = off + 4
--                 subtree:add_packet_field(secure_connect_proto.fields.vec_my_urls, buffer(off, len), ENC_STRING +
--                     ENC_ASCII)
--                 off = off + len
--             end
--         end
--     end
-- end
local proto_dir = Dir.personal_plugins_path() .. "/quazal"
if Dir.exists(proto_dir) then
    for fname in Dir.open(proto_dir) do
        print("Loading " .. fname)
        local module = loadfile(proto_dir .. "/" .. fname)
        local status, proto = pcall(function() module(method_id_field, is_request_field, is_success_field) end)
        if status then
            print("Done")
        else
            print("Failed: " .. proto)
        end
    end
end
