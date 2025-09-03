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
-- DO header
do_proto.fields.size = ProtoField.uint32("do.size", "Size")
do_proto.fields.message_id = ProtoField.uint8("do.message_id", "Message", base.DEC, do_message_types)
-- `Quazal::ProcessAuthentication`
do_proto.fields.process_auth_struct_ver = ProtoField.uint8("do.process_auth.struct_ver", "Structure version")
do_proto.fields.process_auth_lib_ver = ProtoField.uint8("do.process_auth.lib_ver", "Library protocol version")
do_proto.fields.process_auth_unknown_ver = ProtoField.uint8("do.process_auth.unknown_ver", "Unknown version")
do_proto.fields.process_auth_major_ver = ProtoField.uint32("do.process_auth.major_ver", "Major version", base.HEX)
do_proto.fields.process_auth_minor_ver = ProtoField.uint32("do.process_auth.minor_ver", "Minor version", base.HEX)
do_proto.fields.process_auth_title_checksum = ProtoField.uint32("do.process_auth.title_checksum", "Title checksum", base.HEX)
do_proto.fields.process_auth_proto_flags = ProtoField.uint32("do.process_auth.proto_flags", "Protocol flags", base.HEX)
-- `Quazal::_DS_ConnectionInfo`
do_proto.fields.connect_info_url_init = ProtoField.bool("do.connect_info.url_init", "URL initialized")
do_proto.fields.connect_info_url1 = ProtoField.string("do.connect_info.url1", "URL1")
do_proto.fields.connect_info_url2 = ProtoField.string("do.connect_info.url2", "URL2")
do_proto.fields.connect_info_url3 = ProtoField.string("do.connect_info.url3", "URL3")
do_proto.fields.connect_info_url4 = ProtoField.string("do.connect_info.url4", "URL4")
do_proto.fields.connect_info_url5 = ProtoField.string("do.connect_info.url5", "URL5")
do_proto.fields.connect_info_in_bandwidth = ProtoField.uint32("do.connect_info.in_bandwidth", "Input bandwidth")
do_proto.fields.connect_info_in_latency = ProtoField.uint32("do.connect_info.in_latency", "Input latency")
do_proto.fields.connect_info_out_bandwidth = ProtoField.uint32("do.connect_info.out_bandwidth", "Output bandwidth")
do_proto.fields.connect_info_out_latency = ProtoField.uint32("do.connect_info.out_latency", "Output latency")
-- `Quazal::_DS_StationIdentification`
do_proto.fields.station_ident_token = ProtoField.string("do.station_ident.token", "Identification token")
do_proto.fields.station_ident_process_name = ProtoField.string("do.station_ident.process_name", "Process name")
do_proto.fields.station_ident_process_type = ProtoField.uint32("do.station_ident.process_type", "Process type")
do_proto.fields.station_ident_product_ver = ProtoField.uint32("do.station_ident.product_ver", "Product version")
-- `Quazal::_DS_StationInfo`
do_proto.fields.station_info_observer = ProtoField.uint32("do.station_info.observer", "Observer")
do_proto.fields.station_info_machine_uid = ProtoField.uint32("do.station_info.machine_uid", "Machine UID")
-- `Quazal::_DS_StationState`
do_proto.fields.station_state_state = ProtoField.string("do.station_state.state", "State")
local station_states = {
    [0] = "Unknown",
    [1] = "JoiningSession",
    [2] = "CreatingSession",
    [3] = "Participating",
    [4] = "PreparingToLeave",
    [5] = "Leaving",
    [6] = "LeavingOnFault"
}

-- Reads `Quazal::String`.
local function read_string(buffer)
    local size = buffer(0, 2):le_uint()
    local off = 2
    local string = buffer(off, size):string()
    return string, off + size
end

-- Reads `Quazal::ProcessAuthentication`
local function read_process_auth(buffer, tree)
    local off = 0
    tree:add_le(do_proto.fields.process_auth_struct_ver, buffer(off, 1))
    off = off + 1
    tree:add_le(do_proto.fields.process_auth_lib_ver, buffer(off, 1))
    off = off + 1
    tree:add_le(do_proto.fields.process_auth_unknown_ver, buffer(off, 1))
    off = off + 1
    tree:add_le(do_proto.fields.process_auth_major_ver, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.process_auth_minor_ver, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.process_auth_title_checksum, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.process_auth_proto_flags, buffer(off, 4))
    off = off + 4
    return off
end

-- Reads `Quazal::_DS_ConnectionInfo`.
local function read_connection_info(buffer, tree)
    local off = 0
    tree:add_le(do_proto.fields.connect_info_url_init, buffer(off, 1))
    off = off + 1
    local str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.connect_info_url1, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.connect_info_url2, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.connect_info_url3, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.connect_info_url4, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.connect_info_url5, buffer(off, str_len), str)
    off = off + str_len
    tree:add_le(do_proto.fields.connect_info_in_bandwidth, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.connect_info_in_latency, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.connect_info_out_bandwidth, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.connect_info_out_latency, buffer(off, 4))
    off = off + 4
    return off
end

-- Reads `Quazal::_DS_StationIdentification`.
local function read_station_identification(buffer, tree)
    local off = 0
    local str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.station_ident_token, buffer(off, str_len), str)
    off = off + str_len
    str, str_len = read_string(buffer(off))
    tree:add(do_proto.fields.station_ident_process_name, buffer(off, str_len), str)
    off = off + str_len
    tree:add_le(do_proto.fields.station_ident_process_type, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.station_ident_product_ver, buffer(off, 4))
    off = off + 4
    return off
end

-- Reads `Quazal::_DS_StationInfo`.
local function read_station_info(buffer, tree)
    local off = 0
    tree:add_le(do_proto.fields.station_info_observer, buffer(off, 4))
    off = off + 4
    tree:add_le(do_proto.fields.station_info_machine_uid, buffer(off, 4))
    off = off + 4
    return off
end

-- Reads `Quazal::_DS_StationState`.
local function read_station_state(buffer, tree)
    local off = 0
    local state = buffer(off, 2):le_uint()
    local state_name = station_states[state]
    tree:add_le(do_proto.fields.station_state_state, buffer(off, 2), string.format("%s (%d)", state_name, state))
    off = off + 2
    return off
end

-- JoinRequest definition
do_proto.fields.join_request_process_auth = ProtoField.none("do.join_request.process_auth", "ProcessAuthentication")
do_proto.fields.join_request_station_ident = ProtoField.none("do.join_request.station_ident", "StationIdentification")
local function do_parse_join_request(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    -- process authentication
    local process_auth = subtree:add(do_proto.fields.join_request_process_auth, buffer(off, 19))
    off = off + read_process_auth(buffer(off), process_auth)
    -- station identification
    local station_ident = subtree:add(do_proto.fields.join_request_station_ident, buffer(off))
    off = off + read_station_identification(buffer(off), station_ident)
    return off
end

-- JoinResponse definition
do_proto.fields.join_response_success = ProtoField.bool("do.join_response.success", "Success")
do_proto.fields.join_response_client_station_id = ProtoField.uint32("do.join_response.client_station_id", "Client station ID", base.HEX)
do_proto.fields.join_response_master_station_id = ProtoField.uint32("do.join_response.master_station_id", "Master station ID", base.HEX)
do_proto.fields.join_response_bootstrap_urls_count = ProtoField.uint16("do.join_response.bootstrap_urls_count", "Bootstrap URLs")
local function do_parse_join_response(buffer, pinfo, tree, subtree, in_bundle)
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

-- Update definition
local station_datasets = {
    [1] = "ConnectionInfo",
    [2] = "StationIdentification",
    [3] = "StationInfo",
    [4] = "StationState"
}
do_proto.fields.update_do_handle = ProtoField.uint32("do.update.do_handle", "DO handle", base.HEX)
do_proto.fields.update_dataset_id = ProtoField.uint8("do.update.dataset_id", "Dataset ID")
do_proto.fields.update_connection_info = ProtoField.none("do.update.connection_info", "ConnectionInfo")
do_proto.fields.update_station_ident = ProtoField.none("do.update.station_ident", "StationIdentification")
do_proto.fields.update_station_info = ProtoField.none("do.update.station_info", "StationInfo")
do_proto.fields.update_station_state = ProtoField.none("do.update.station_state", "StationState")
local function do_parse_update(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    local do_handle = buffer(off, 4):le_uint()
    subtree:add_le(do_proto.fields.update_do_handle, buffer(off, 4))
    off = off + 4
    pinfo.cols.info:append(string.format(" 0x%X", do_handle))
    local dataset_id = buffer(off, 1):le_uint()
    subtree:add_le(do_proto.fields.update_dataset_id, buffer(off, 1))
    off = off + 1
    -- `Quazal::_DO_Station::s_uiClassID` (value from Ghost Recon Online)
    local station_DOC_ID = 23
    -- TODO: handle updates for other OSDK DO classes
    -- handle station updates
    if do_handle >> 22 == station_DOC_ID then
        local dataset_name = station_datasets[dataset_id]
        if dataset_name == nil then
            dataset_name = "Invalid dataset index"
        end
        pinfo.cols.info:append(string.format(" %s", dataset_name))
        if dataset_name == "ConnectionInfo" then
            local connect_info = subtree:add(do_proto.fields.update_connection_info, buffer(off))
            off = off + read_connection_info(buffer(off), connect_info)
        elseif dataset_name == "StationIdentification" then
            local station_ident = subtree:add(do_proto.fields.update_station_ident, buffer(off))
            off = off + read_station_identification(buffer(off), station_ident)
        elseif dataset_name == "StationInfo" then
            local station_info = subtree:add(do_proto.fields.update_station_info, buffer(off))
            off = off + read_station_info(buffer(off), station_info)
        elseif dataset_name == "StationState" then
            local station_state = subtree:add(do_proto.fields.update_station_state, buffer(off))
            off = off + read_station_state(buffer(off), station_state)
        end
        return off
    end
    -- negative bytes read by this function in case we cant read the payload (for bundle processing)
    if in_bundle then
        return -off
    end
    return buffer:len()
end

-- Delete definition
do_proto.fields.delete_do_handle = ProtoField.uint32("do.delete.do_handle", "DO handle", base.HEX)
local function do_parse_delete(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    local do_handle = buffer(off, 4):le_uint()
    pinfo.cols.info:append(string.format(" 0x%X", do_handle))
    subtree:add_le(do_proto.fields.delete_do_handle, buffer(off, 4))
    off = off + 4
    return off
end

-- Action definition
do_proto.fields.action_do_handle = ProtoField.uint32("do.action.do_handle", "DO handle", base.HEX)
do_proto.fields.action_method_id = ProtoField.uint16("do.action.method_id", "Method ID")
local function do_parse_action(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    local do_handle = buffer(off, 4):le_uint()
    pinfo.cols.info:append(string.format(" 0x%X", do_handle))
    subtree:add_le(do_proto.fields.action_do_handle, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.action_method_id, buffer(off, 2))
    off = off + 2
    -- TODO: implement further payload reading
    return off
end

-- CallOutcome definition
local call_outcomes = {
    [0x10001] = "Success",
    [0x60001] = "Success",
    [0x60002] = "CallPostponed",
    [0x80010001] = "UnknownOutcome",
    [0x80010006] = "ErrorAccessDenied",
    [0x8001000A] = "ErrorInvalidParameters",
    [0x80060001] = "ErrorStationNotReached",
    [0x80060002] = "ErrorTargetStationDisconnect",
    [0x80060003] = "ErrorLocalStationLeaving",
    [0x80060004] = "ErrorObjectNotFound",
    [0x80060005] = "ErrorInvalidRole",
    [0x80060006] = "ErrorCallTimeout",
    [0x80060007] = "ErrorRMCDispatchFailed",
    [0x80060008] = "ErrorMigrationInProgress",
    [0x80060009] = "ErrorNoAuthority",
}

do_proto.fields.call_outcome_call_id = ProtoField.uint16("do.call_outcome.call_id", "Call ID")
do_proto.fields.call_outcome_outcome = ProtoField.string("do.call_outcome.call_id", "Outcome")
local function do_parse_call_outcome(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    subtree:add_le(do_proto.fields.call_outcome_call_id, buffer(off, 2))
    off = off + 2
    local outcomeCode = buffer(off, 4):le_uint()
    local outcome = call_outcomes[outcomeCode]
    if outcome == nil then
        outcome = "UnknownOutcome"
    end
    outcome = string.format("%s (0x%X)", outcome, outcomeCode)
    subtree:add(do_proto.fields.call_outcome_outcome, buffer(off, 4), outcome)
    off = off + 4
    return off
end

-- RMCCall definition
do_proto.fields.rmc_call_call_id = ProtoField.uint16("do.rmc_call.call_id", "Call ID")
do_proto.fields.rmc_call_flags = ProtoField.uint32("do.rmc_call.flags", "Flags", base.HEX)
do_proto.fields.rmc_call_source_id = ProtoField.uint32("do.rmc_call.source_id", "Source ID", base.HEX)
do_proto.fields.rmc_call_target_object = ProtoField.uint32("do.rmc_call.target_object", "Target DOC object", base.HEX)
do_proto.fields.rmc_call_method_id = ProtoField.uint16("do.rmc_call.method_id", "Method ID")
local function do_parse_rmc_call(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    subtree:add_le(do_proto.fields.rmc_call_call_id, buffer(off, 2))
    off = off + 2
    subtree:add_le(do_proto.fields.rmc_call_flags, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.rmc_call_source_id, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.rmc_call_target_object, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.rmc_call_method_id, buffer(off, 2))
    off = off + 2
    -- TODO: implement reading for optional fields
    if in_bundle then
        return -off
    end
    return buffer:len()
end

-- RMCResponse definition
do_proto.fields.rmc_response_call_id = ProtoField.uint16("do.rmc_response.call_id", "Call ID")
do_proto.fields.rmc_response_outcome = ProtoField.string("do.rmc_response.outcome", "Outcome")
local function do_parse_rmc_response(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    subtree:add_le(do_proto.fields.rmc_response_call_id, buffer(off, 2))
    off = off + 2
    local outcomeCode = buffer(off, 4):le_uint()
    local outcome = call_outcomes[outcomeCode]
    if outcome == nil then
        outcome = "UnknownOutcome"
    end
    outcome = string.format("%s (0x%X)", outcome, outcomeCode)
    subtree:add(do_proto.fields.rmc_response_outcome, buffer(off, 4), outcome)
    off = off + 4
    -- TODO: Quazal::_DS_Range? (0x00000101 - 0x00000201 for RequestIDRangeFromMaster calls)
    if in_bundle then
        return -off
    end
    return buffer:len()
end

-- FetchRequest definition
do_proto.fields.fetch_request_call_id = ProtoField.uint16("do.fetch_request.call_id", "Call ID")
do_proto.fields.fetch_request_fetched_do = ProtoField.uint32("do.fetch_request.fetched_do", "Fetched DO", base.HEX)
do_proto.fields.fetch_request_master = ProtoField.uint32("do.fetch_request.master", "Master station", base.HEX)
local function do_parse_fetch_request(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    subtree:add_le(do_proto.fields.fetch_request_call_id, buffer(off, 2))
    off = off + 2
    subtree:add_le(do_proto.fields.fetch_request_fetched_do, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.fetch_request_master, buffer(off, 4))
    off = off + 4
    return off
end

-- Bundle definition
do_proto.fields.bundle_msg = ProtoField.none("do.bundle.msg", "DO Message")
local function do_parse_bundle(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    while off < buffer:len() do
        local size = buffer(off, 4):le_uint()
        -- end of bundle
        if size == 0 then
            break
        end
        local subsubtree = subtree:add(do_proto.fields.bundle_msg, buffer())
        subsubtree:add_le(do_proto.fields.size, buffer(off, 4))
        off = off + 4
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
            local parser_off = parser(buffer(off), pinfo, subtree, subsubtree, true)
            if parser_off < 0 then
                off = off - 1 + size
            else
                off = off + parser_off
            end
        end
    end
    return off
end

-- Migration definition
do_proto.fields.migration_call_id = ProtoField.uint16("do.migration.call_id", "Call ID")
do_proto.fields.migration_source_station = ProtoField.uint32("do.migration.source_station", "Source station", base.HEX)
do_proto.fields.migration_recipient_station = ProtoField.uint32("do.migration.recipient_station", "Recipient station", base.HEX)
do_proto.fields.migration_target_station = ProtoField.uint32("do.migration.target_station", "Target station", base.HEX)
do_proto.fields.migration_unknown = ProtoField.uint8("do.migration.unknown", "Unknown byte")
do_proto.fields.migration_duplicas = ProtoField.uint32("do.migration.duplicas", "Duplicas")
do_proto.fields.migration_duplica = ProtoField.uint32("do.migration.duplica", "Duplica")
local function do_parse_migration(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    subtree:add_le(do_proto.fields.migration_call_id, buffer(off, 2))
    off = off + 2
    subtree:add_le(do_proto.fields.migration_source_station, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.migration_recipient_station, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.migration_target_station, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.migration_unknown, buffer(off, 1))
    off = off + 1
    local duplicas = subtree:add_le(do_proto.fields.migration_duplicas, buffer(off, 4))
    local duplica_count = buffer(off, 4):le_uint()
    off = off + 4
    for idx=1, duplica_count do
        duplicas:add_le(do_proto.fields.migration_duplica, buffer(off, 4))
        off = off + 4
    end
    return off
end

-- CreateDuplica definition
do_proto.fields.create_duplica_do_handle = ProtoField.uint32("do.create_duplica.do_handle", "DO handle")
do_proto.fields.create_duplica_master = ProtoField.uint32("do.create_duplica.master", "Master station")
do_proto.fields.create_duplica_version = ProtoField.uint32("do.create_duplica.version", "Version")
local function do_parse_create_duplica(buffer, pinfo, tree, subtree, in_bundle)
    local off = 0
    local do_handle = buffer(off, 4):le_uint()
    subtree:add_le(do_proto.fields.create_duplica_do_handle, buffer(off, 4))
    pinfo.cols.info:append(string.format(" 0x%X", do_handle))
    off = off + 4
    subtree:add_le(do_proto.fields.create_duplica_master, buffer(off, 4))
    off = off + 4
    subtree:add_le(do_proto.fields.create_duplica_version, buffer(off, 1))
    off = off + 1
    -- TODO: implement DO payload reading
    if in_bundle then
        return -off
    end
    return buffer:len()
end

-- CreateAndPromoteDuplica definition
do_proto.fields.create_promote_duplica_call_id = ProtoField.uint16("do.create_promote_duplica.call_id", "Call ID")
do_proto.fields.create_promote_duplica_client_station_id = ProtoField.uint32("do.create_promote_duplica.client_station_id", "Client station ID", base.HEX)
do_proto.fields.create_promote_duplica_master_station_id = ProtoField.uint32("do.create_promote_duplica.master_station_id", "Master station ID", base.HEX)
do_proto.fields.create_promote_duplica_version = ProtoField.uint8("do.create_promote_duplica.version", "Version")
do_proto.fields.create_promote_duplica_list = ProtoField.uint32("do.create_promote_duplica.list", "Duplicas")
do_proto.fields.create_promote_duplica_duplica = ProtoField.uint32("do.create_promote_duplica.duplica", "Duplica")
do_proto.fields.create_promote_duplica_discovery_msg = ProtoField.none("do.create_promote_duplica.discovery_msg", "DiscoveryMessage")
do_proto.fields.create_promote_duplica_connect_info = ProtoField.none("do.create_promote_duplica.connect_info", "ConnectionInfo")
do_proto.fields.create_promote_duplica_connect_info_exists = ProtoField.bool("do.create_promote_duplica.connect_info.exists", "Exists")
do_proto.fields.create_promote_duplica_station_ident = ProtoField.none("do.create_promote_duplica.station_ident", "StationIdentification")
do_proto.fields.create_promote_duplica_station_ident_exists = ProtoField.bool("do.create_promote_duplica.station_ident.exists", "Exists")
do_proto.fields.create_promote_duplica_station_info = ProtoField.none("do.create_promote_duplica.station_info", "StationInfo")
do_proto.fields.create_promote_duplica_station_info_exists = ProtoField.bool("do.create_promote_duplica.station_info.exists", "Exists")
do_proto.fields.create_promote_duplica_station_state = ProtoField.none("do.create_promote_duplica.station_state", "StationState")
do_proto.fields.create_promote_duplica_station_state_exists = ProtoField.bool("do.create_promote_duplica.station_state.exists", "Exists")
do_proto.fields.create_promote_duplica_station_state_state = ProtoField.uint16("do.create_promote_duplica.station_state.state", "State")
local function do_parse_create_and_promote_duplica(buffer, pinfo, tree, subtree, in_bundle)
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
    -- connection info
    local connect_info = subtree:add(do_proto.fields.create_promote_duplica_connect_info, buffer(off))
    local conn_info_exists = buffer(off, 1):le_uint()
    connect_info:add_le(do_proto.fields.create_promote_duplica_connect_info_exists, conn_info_exists)
    off = off + 1
    if conn_info_exists == 1 then
        off = off + read_connection_info(buffer(off), connect_info)
    end
    -- station identification
    local station_ident = subtree:add(do_proto.fields.create_promote_duplica_station_ident, buffer(off))
    local station_ident_exists = buffer(off, 1):le_uint()
    station_ident:add(do_proto.fields.create_promote_duplica_station_ident_exists, station_ident_exists)
    off = off + 1
    if station_ident_exists == 1 then
        off = off + read_station_identification(buffer(off), station_ident)
    end
    -- station info
    local station_info = subtree:add(do_proto.fields.create_promote_duplica_station_info, buffer(off))
    local station_info_exists = buffer(off, 1):le_uint()
    station_info:add_le(do_proto.fields.create_promote_duplica_station_info_exists, station_info_exists)
    off = off + 1
    if station_info_exists == 1 then
        off = off + read_station_info(buffer(off), station_info)
    end
    -- station state
    local station_state = subtree:add(do_proto.fields.create_promote_duplica_station_state, buffer(off))
    local station_state_exists = buffer(off, 1):le_uint()
    station_state:add_le(do_proto.fields.create_promote_duplica_station_state_exists, station_state_exists)
    off = off + 1
    if station_state_exists == 1 then
        off = off + read_station_state(buffer(off), station_state)
    end
    return off
end

-- GetParticipantsRequest definition
do_proto.fields.get_participants_request_urls = ProtoField.uint32("do.get_participants_request.urls", "Station URLs")
do_proto.fields.get_participants_request_url = ProtoField.string("do.get_participants_request.url", "URL")
local function do_parse_get_participants_request(buffer, pinfo, tree, subtree, in_bundle)
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
local function do_parse_get_participants_response(buffer, pinfo, tree, subtree, in_bundle)
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
local function do_parse_empty(buffer, pinfo, tree, subtree, in_bundle)
    return 0
end

local function do_parse_eos(buffer, pinfo, tree, subtree, in_bundle)
    
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
        off = off + parser(buffer(off), pinfo, tree, subtree, false)
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
