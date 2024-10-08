local packet_types = {
    [5] = "Session?",
    [8] = "NAT",
    [9] = "PingRequest",
    [10] = "PingReply",
    [11] = "Player?",
}


local storm_proto = Proto("Storm", "Storm P2P")
storm_proto.fields.header1 = ProtoField.uint8("storm.header.field1", "Field 1")
storm_proto.fields.header2 = ProtoField.uint8("storm.header.field2", "Field 2")
storm_proto.fields.header3 = ProtoField.uint8("storm.header.field3", "Field 3", base.DEC)
storm_proto.fields.header4 = ProtoField.uint8("storm.header.field4", "Field 4", base.DEC)
storm_proto.fields.header5 = ProtoField.uint64("storm.header.field5", "Field 5", base.HEX)
storm_proto.fields.type = ProtoField.uint8("storm.header.type", "Type", base.HEX, packet_types)
storm_proto.fields.header6 = ProtoField.uint16("storm.header.field6", "Field 6", base.HEX)


local nat_storm_proto = Proto("StormNAT", "Storm NAT")
nat_storm_proto.fields.size = ProtoField.uint8("storm.nat.size", "Size", base.DEC, nil, 0x0F)

local routable_packet_proto = Proto("StormRoutablePacket", "Storm RoutablePacket")
routable_packet_proto.fields.field1 = ProtoField.uint32("storm.routable.field1", "Field 1", base.HEX)
routable_packet_proto.fields.peer_desc_field1_1 = ProtoField.uint32("storm.routable.peer_desc.field1_1",
    "Peer Desc Field 1.1", base.HEX)
routable_packet_proto.fields.peer_desc_field1_2 = ProtoField.uint16("storm.routable.peer_desc.field1_2",
    "Peer Desc Field 1.2", base.HEX)
routable_packet_proto.fields.peer_desc_field2_1 = ProtoField.bytes("storm.routable.peer_desc.field2_1",
    "Peer Desc Field 2.1")

local peer_packet_proto = Proto("StormPeerPacket", "Storm PeerPacket")
peer_packet_proto.fields.field1 = ProtoField.uint32("storm.peer.field1", "Field 1", base.HEX)

local data_dis = Dissector.get("data")
local packet_type = Field.new("storm.header.type")


local subtypes = {
    [1] = peer_packet_proto,
    [2] = routable_packet_proto,
    [5] = routable_packet_proto,
    [7] = routable_packet_proto,
    [8] = nat_storm_proto,
    [0xb] = routable_packet_proto,
}

local function read_bits(buffer, start, bits)
    local res = ByteArray.new()
    res:set_size(bit.rshift(bits + 7, 3))
    print(start, bits, res:len())
    local tmp = 0
    local idx = 0
    local out_idx = 0
    print(buffer(0, 2))
    if start ~= 0 then
        -- tmp = bit.lshift(buffer(idx, 1):bitfield(start, 8 - start), start)
        tmp = buffer(idx, 1):bitfield(start, 8 - start)
        tmp = bit.lshift(tmp, start)
        bits = bits - (8 - start)
        idx = idx + 1
    end

    if bits <= 0 then
        res:set_index(0, tmp)
        return res
    end

    while bits >= 8 do
        local tmp2 = buffer(idx, 1):bitfield(0, start)
        tmp = bit.bor(tmp, tmp2)
        print(tmp)
        res:set_index(out_idx, tmp)
        tmp = bit.lshift(buffer(idx, 1):bitfield(start, 8 - start), start)
        print(tmp)
        bits = bits - 8
        idx = idx + 1
        out_idx = out_idx + 1
    end
    if bits > 0 then
        local tmp2 = buffer(idx, 1):bitfield(0, bits)
        tmp = bit.bor(tmp, tmp2)
        res:set_index(out_idx, tmp)
    end
    return res
end

local function read_compressed_bytes(buffer, bit_off, num_bytes, param_3)
    local prefix1 = 0
    local prefix2 = 0
    if param_3 == 0 then
        prefix1 = 0xFF
        prefix2 = 0xF0
    end

    local res = ByteArray.new()
    res:set_size(num_bytes)
    for i = 2, num_bytes do
        local idx = bit.rshift(bit_off, 3)
        local rem = bit.band(bit_off, 0x7)
        local x = buffer(idx, 1):bitfield(rem, 1)
        bit_off = bit_off + 1
        if x == 0 then
            local idx = bit.rshift(bit_off, 3)
            local rem = bit.band(bit_off, 0x7)
            res:append(read_bits(buffer(idx), rem, (num_bytes - i) * 8))
            return bit_off + (num_bytes - i) * 8, res
        end
        res:set_index(i - 1, prefix1)
    end

    local idx = bit.rshift(bit_off, 3)
    local rem = bit.band(bit_off, 0x7)
    bit_off = bit_off + 1
    if buffer(idx, 1):bitfield(rem, 1) == 0 then
        local tmp = buffer(idx, 2):bitfield(rem + 1, 8)
        res:set_index(num_bytes - 1, tmp)
    else
        local tmp = buffer(idx, 2):bitfield(rem + 1, 4)
        res:set_index(num_bytes - 1, bit.bor(tmp, prefix2))
    end
    return res
end

Stream = { buffer = nil, current_bit_pos = 0 }

function Stream:new(buffer)
    s = {}
    setmetatable(s, self)
    self.__index = self
    self.buffer = buffer
    self.current_bit_pos = 0
    return s
end

function Stream:read_bits(n)
    local start_idx = bit.rshift(self.current_bit_pos, 3)
    local start_bit = self.current_bit_pos % 8
    local num_bytes = bit.rshift(start_bit + n + 7, 3)
    local bytes = self.buffer(start_idx, num_bytes)
    self.current_bit_pos = self.current_bit_pos + n
    return bytes, bytes:bitfield(start_bit, n)
end

function routable_packet_proto.dissector(buffer, pinfo, tree)
    local subtree = tree:add(routable_packet_proto, buffer(), "Storm RoutablePacket")
    subtree:add_le(routable_packet_proto.fields.field1, buffer(0, 5), bit.bswap(buffer(0, 5):bitfield(4, 32)))
    subtree:add_le(routable_packet_proto.fields.peer_desc_field1_1,
        buffer(4, 5), bit.bswap(buffer(4, 5):bitfield(4, 32)))
    subtree:add_le(routable_packet_proto.fields.peer_desc_field1_2,
        buffer(8, 3), buffer(8, 3):bitfield(4, 16))

    local res = read_compressed_bytes(buffer(10):tvb(), 4, 1)

    subtree:add(routable_packet_proto.fields.peer_desc_field2_1,
        buffer(10, 2), res:raw())

    local bit_off, res = read_compressed_bytes(buffer(11):tvb(), 4, 4)
    local l = bit.rshift(bit_off, 3)

    subtree:add(routable_packet_proto.fields.peer_desc_field2_1,
        buffer(11, l + 1), res:raw())

    local ptype = packet_type().value
    local proto = subtypes[ptype]
    if proto ~= nil and proto ~= routable_packet_proto then
        proto.dissector(buffer(4):tvb(), pinfo, tree)
    else
        data_dis:call(buffer(4):tvb(), pinfo, tree)
    end
end

function peer_packet_proto.dissector(buffer, pinfo, tree)
    local subtree = tree:add(peer_packet_proto, buffer(), "Storm PeerPacket")
    subtree:add_le(routable_packet_proto.fields.field1, buffer(0, 5), buffer(0, 5):bitfield(4, 32))

    data_dis:call(buffer(4):tvb(), pinfo, tree)
end

function nat_storm_proto.dissector(buffer, pinfo, tree)
    pinfo.cols.protocol = "Storm NAT"
    local subtree = tree:add(nat_storm_proto, buffer(), "Storm NAT")
    subtree:add(nat_storm_proto.fields.size, buffer(0, 1))

    data_dis:call(buffer(1):tvb(), pinfo, tree)
end

function storm_proto.dissector(buffer, pinfo, tree)
    pinfo.cols.protocol = "Storm P2P"

    local stream = Stream:new(buffer)

    local subtree = tree:add(storm_proto, buffer(), "Storm Data")
    -- subtree:add(storm_proto.fields.header1, buffer(0, 1))
    -- subtree:add(storm_proto.fields.header2, buffer(1, 1))
    -- subtree:add(storm_proto.fields.header3, buffer(2, 1))
    -- subtree:add(storm_proto.fields.header4, buffer(2, 1))
    -- subtree:add(storm_proto.fields.header5, buffer(3, 8))
    -- subtree:add(storm_proto.fields.type, buffer(11, 1))
    subtree:add(storm_proto.fields.header1, stream:read_bits(8))
    subtree:add(storm_proto.fields.header2, stream:read_bits(8))
    subtree:add(storm_proto.fields.header3, stream:read_bits(4))
    subtree:add(storm_proto.fields.header4, stream:read_bits(4))
    subtree:add(storm_proto.fields.header5, stream:read_bits(64))
    subtree:add(storm_proto.fields.type, stream:read_bits(4))
    local packet_type = buffer(11, 1):bitfield(0, 4)
    subtree:add(storm_proto.fields.header6, stream:read_bits(16))

    if packet_types[packet_type] ~= nil then
        pinfo.cols.protocol = "Storm " .. packet_types[packet_type]
    else
        pinfo.cols.protocol = "Storm (" .. packet_type .. ')'
    end

    local proto = subtypes[packet_type]
    if proto ~= nil then
        if packet_type == 8 then
            proto.dissector(buffer(13):tvb(), pinfo, tree)
        else
            routable_packet_proto.dissector(buffer(13):tvb(), pinfo, tree)
        end
    else
        data_dis:call(buffer(13):tvb(), pinfo, tree)
    end
end

local udp_table = DissectorTable.get("udp.port")
udp_table:add(13000, storm_proto)
