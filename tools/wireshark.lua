--- @module Wireshark
--- Base Wireshark proto definitions.
-- Annotations are EmmyLua format, as used by the Sumneko VSCode extension.
-- https://emmylua.github.io/
--  https://github.com/sumneko/lua-language-server
-- Ideally, we'd auto-create this file from the C++ sources in Wireshark
DESEGMENT_ONE_MORE_SEGMENT = 1

--- @class Proto
--- A new protocol in Wireshark. Protocols have several uses. The main one is to dissect a protocol, but they can also be dummies used to register preferences for other purposes
Proto = {}

--- @param name string The name of the protocol.
--- @param desc string A Long Text description of the protocol (usually lowercase).
--- @return proto The newly created Proto object.
function Proto.new(name, desc) end

--- @class proto

proto = {}

--- The protocol’s dissector, a function you define.
--- @param tree TreeItem
function proto.dissector(tvb, pinfo, tree) end

--- @param name string The name of the protocol.
--- @param desc string A Long Text description of the protocol (usually lowercase).
function proto:__call(name, desc) end

ProtoField = {}
ProtoExpert = {}

--- @class base
base = { NONE = 1, DEC = 2, HEX = 3, OCT = 4, DEC_HEX = 5, HEX_DEC = 6 }

ftypes = { UINT8 = 1, UINT16 = 2, UINT24 = 3, UINT32 = 4, STRING = 5 }

--- @class Tvb
Tvb = {}

--- @class TreeItem
--- @class Pinfo

--- @class dissectortable @returned by DissectorTable.get
dissectortable = {}

--- A refererence to a dissector, used to call a dissector against a packet or a part of it.
Dissector = {}

--- Obtains a dissector reference by name.
--- @param name string name of dissector
--- @return dissector @The Dissector reference if found, otherwise nil
function Dissector.get(name) end

--- Gets a Lua array table of all registered Dissector names.
--- @return string[] The array table of registered dissector names.
function Dissector.list() end

--- @class dissector
dissector = {}

--- Calls a dissector against a given packet (or part of it).
---@param tvb Tvb The buffer to dissect
---@param pinfo Pinfo The packet info
---@param tree TreeItem The tree on which to add the protocol items.
---@return number Number of bytes dissected. Note that some dissectors always return number of bytes in incoming buffer, so be aware.
function dissector:call(tvb, pinfo, tree) end

--- Calls a dissector against a given packet (or part of it).
---@param tvb Tvb The buffer to dissect
---@param pinfo Pinfo The packet info
---@param tree TreeItem The tree on which to add the protocol items.
function dissector:__call(tvb, pinfo, tree) end

---Gets the Dissector’s protocol short name.
---@return string @A string of the protocol’s short name.
function dissector:__tostring() end

--- A table of subdissectors of a particular protocol
DissectorTable = {}

---Creates a new DissectorTable for your dissector’s use.
--- @param tablename string The short name of the table. Use lower-case alphanumeric, dot, and/or underscores (e.g., "ansi_map.tele_id" or "udp.port").
--- @param uiname string optional: The name of the table in the user interface. Defaults to the name given in tablename, but can be any string.
--- @param type number One of ftypes.UINT8, ftypes.UINT16, ftypes.UINT24, ftypes.UINT32, or ftypes.STRING. Defaults to ftypes.UINT32
--- @param base number One of base.NONE, base.DEC, base.HEX, base.OCT, base.DEC_HEX or base.HEX_DEC. Defaults to base.DEC.
--- @param proto Proto The Proto object that uses this dissector table.
--- @return dissectortable @The newly created DissectorTable.
function DissectorTable.new(tablename, uiname, type, base, proto) end

--- Gets a Lua array table of all DissectorTable names - i.e., the string names you can use for the first argument to DissectorTable.get()
--- @return string[] @The array table of registered DissectorTable names
function DissectorTable.list() end

--- Gets a Lua array table of all heuristic list names - i.e., the string names you can use for the first argument in Proto:register_heuristic()
--- @return string[] @The array table of registered heuristic list names
function DissectorTable.heuristic_list() end

---Try all the dissectors in a given heuristic dissector table
---@param listname string The name of the heuristic dissector.
---@param tvb Tvb The buffer to dissect
---@param pinfo Pinfo The packet info
---@param tree TreeItem The tree on which to add the protocol items
function DissectorTable.try_heuristics(listname, tvb, pinfo, tree) end

--- Obtain a reference to an existing dissector table.
--- @param tablename string The short name of the table
--- @return dissectortable @The DissectorTable reference if found, otherwise nil.
function DissectorTable.get(tablename) end

--- Add a Proto with a dissector function or a Dissector object to the dissector table.
--- @param pattern number|string The pattern to match (either an integer, a integer range or a string depending on the table’s type)
--- @param dissector Proto|dissector The dissector to add (either a Proto or a Dissector)
function dissectortable:add(pattern, dissector) end

--- Clear all existing dissectors from a table and add a new dissector or a range of new dissectors.
--- @param pattern number|string The pattern to match (either an integer, a integer range or a string depending on the table’s type)
--- @param dissector Proto|dissector The dissector to add (either a Proto or a Dissector)
function dissectortable:set(pattern, dissector) end

--- Remove a dissector or a range of dissectors from a table.
--- @param pattern number|string The pattern to match (either an integer, a integer range or a string depending on the table’s type)
--- @param dissector Proto|dissector The dissector to remove (either a Proto or a Dissector).
function dissectortable:remove(pattern, dissector) end

---Remove all dissectors from a table.
function dissectortable:remove_all() end

--- Try to call a dissector from a table.
--- @param pattern string|number The pattern to be matched (either an integer or a string depending on the table’s type)
--- @param tvb Tvb The Tvb to dissect
--- @param pinfo Pinfo The packet info
--- @param tree TreeItem The tree on which to add the protocol items
--- @return number Number of bytes dissected. Note that some dissectors always return number of bytes in incoming buffer, so be aware
function dissectortable:try(pattern, tvb, pinfo, tree) end

--- Try to obtain a dissector from a table
--- @param pattern number|string The pattern to match (either an integer, a integer range or a string depending on the table’s type)
--- @return dissector  The Dissector handle if found, otherwise nil
function dissectortable:get_dissector(pattern) end

---Gets some debug information about the DissectorTable.
---@return string A string of debug information about the DissectorTable.
function dissectortable:__tostring() end

expert = {}

--- Add a directory to the head of the package search path
--- @param path string directory to add
function package.prepend_path(path) end

--- @class Field
Field = {}

--- @class field
field = {}

--- @param name string
--- @return field
function Field.new(name) end

--- @return FieldInfo
function field:__call() end

--- @class FieldInfo
--- @field value unknown
FieldInfo = {}

ENC_STRING = 0
ENC_UTF_8 = 0
ENC_ASCII = 0


--- @class ByteArray
ByteArray = {}
