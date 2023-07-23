local PROTO_MAPPING = {}
for line in io.lines(Dir.personal_plugins_path() .. "/rmc.txt") do
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

quazal_proto = Proto("prudp", "Pretty Reliable UDP (Quazal)")
vport_proto = Proto("vport", "PRUDP VPort")

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

local stream_index = Field.new("udp.stream")
local ip_src = Field.new("ip.src")
local udp_port = Field.new("udp.srcport")
local fragments = {}

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

    print(segment_id, fragment_id)
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
    print(segment_id, fragment_id)
    res[#res + 1] = self.packets[segment_id]
    return res
end

function vport(tree, buf)
    tree:add(vport_proto.fields.port, buf(0, 1))
    tree:add(vport_proto.fields.type, buf(0, 1))
end

function quazal_proto.init()
    fragments = {}
end

function quazal_proto.dissector(buffer, pinfo, tree)
    local si = tostring(stream_index().value)
    local is = tostring(ip_src().value)
    local up = tostring(udp_port().value)
    local key = si .. "|" .. is .. "|" .. up
    print("1", key, fragments[key])
    if fragments[key] == nil then
        -- print("2", key)
        fragments[key] = PacketCollector:new()
        print("P", #fragments[key].packets)
    end
    quazal_proto_dissector(buffer, pinfo, tree, fragments[key])
end

function quazal_proto_dissector(buffer, pinfo, tree, fragments)
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

    if payload:len() > 0 and stype == "RVSec" and ptype == "Data" and ack == false then
        --
        local packets = fragments:update(payload:bytes(), sequence_id, fragment_id)
        -- print(#packets)
        if #packets > 0 then
            local data = ByteArray.new()
            for i = 1, #packets do
                data:append(packets[i].data)
            end
            rmc_proto_dissector(data:tvb("Reassembled"), pinfo, tree)
        end
    end

    if buffer:len() > off then
        quazal_proto_dissector(buffer(off), pinfo, tree, fragments)
    end
end

udp_table = DissectorTable.get("udp.port")
udp_table:add(3074, quazal_proto)


rmc_proto = Proto("RMC", "Quazal RMC")
rmc_proto.fields.size = ProtoField.uint32("rmc.size", "Size")
rmc_proto.fields.prot_id_short = ProtoField.uint8("rmc.protocol_id", "Procotol ID", base.DEC, nil, 0x7F)
rmc_proto.fields.prot_id_long = ProtoField.uint16("rmc.protocol_id", "Procotol ID")
rmc_proto.fields.call_id = ProtoField.uint32("rmc.call_id", "Call")
rmc_proto.fields.method_id = ProtoField.uint32("rmc.method_id", "Method", base.DEC, nil, 0x7FFF)
rmc_proto.fields.is_success = ProtoField.bool("rmc.is_success", "Is Success", 8, nil, 1)
rmc_proto.fields.payload = ProtoField.bytes("rmc.payload", "Payload")
rmc_proto.fields.error_code = ProtoField.uint32("rmc.error_code", "Error Code", base.HEX)

function rmc_proto_dissector(buffer, pinfo, tree)
    pinfo.cols.protocol = "RMC"
    local subtree = tree:add(rmc_proto, buffer(), "RMC Data")
    subtree:add_le(rmc_proto.fields.size, buffer(0, 4))
    local size = buffer(0, 4):le_uint()
    local off = 4
    local is_request = true
    local proto = 0
    local pt = nil
    if buffer(4, 1):uint() == 0xff then
        pt = subtree:add_le(rmc_proto.fields.prot_id_long, buffer(5, 2))
        off = 7
        proto = buffer(5, 2):le_uint()
    else
        pt = subtree:add_le(rmc_proto.fields.prot_id_short, buffer(4, 1))
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
        local is_success = buffer(off, 1):uint()
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
    subtree:add(rmc_proto.fields.payload, buffer(off, size - off + 4))
    off = size + 4
    if buffer:len() > off then
        rmc_proto_dissector(buffer(off), pinfo, tree)
    end
end

function new_rc4(key)
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
