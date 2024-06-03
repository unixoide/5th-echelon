#!/usr/bin/env python3
import argparse
import json
import platform
import pathlib
import os
from dataclasses import dataclass, field, InitVar
from pprint import pprint
import re
import textwrap
import traceback
from typing import Any, Optional

if platform.system() == "Windows":
    PLUGIN_DIR = pathlib.Path(os.getenv("APPDATA")) / "Wireshark" / "plugins"
else:
    PLUGIN_DIR = pathlib.Path("~/.local/lib/wireshark/plugins").expanduser()

TARGET_DIR: pathlib.Path = PLUGIN_DIR / "quazal"


def snake_case(self: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", self).lower()


class Type:
    @staticmethod
    def from_ty(ty):
        if "ty" in ty:
            assert tuple(ty) == ("ty", "unknown")
            ty = ty["ty"]

        if name := ty.get("Simple"):
            return SimpleType(name)
        elif name := ty.get("Class"):
            return ClassType(name)
        elif templ := ty.get("Template"):
            return TemplateType(
                templ["template_name"], [Type.from_ty(ty) for ty in templ["parameters"]]
            )
        else:
            raise RuntimeError(f"Unknown type {ty}")

    def to_wireshark_def(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        abbr: str,
        field_name: str,
        name: str,
    ) -> str:
        raise NotImplementedError(str(self))

    def to_wireshark(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: "Method",
        field: "Field",
        name: str,
    ) -> str:
        raise NotImplementedError(str(self))


def string_sz():
    lines = []
    lines.append("local sz = buffer(off, 2):le_uint()")
    lines.append("off = off + 2")
    return "\n".join(lines), "sz"


def variant_sz():
    return "", ""


@dataclass
class SimpleType(Type):
    name: str

    def translate(self):
        return {
            "bool": (1, "ftypes.BOOLEAN", [], []),
            "byte": (1, "ftypes.UINT8", [], ["ENC_LITTLE_ENDIAN"]),
            "uint8": (1, "ftypes.UINT8", [], ["ENC_LITTLE_ENDIAN"]),
            "int8": (1, "ftypes.INT8", [], ["ENC_LITTLE_ENDIAN"]),
            "uint16": (2, "ftypes.UINT16", [], ["ENC_LITTLE_ENDIAN"]),
            "int16": (2, "ftypes.INT16", [], ["ENC_LITTLE_ENDIAN"]),
            "uint32": (4, "ftypes.UINT32", [], ["ENC_LITTLE_ENDIAN"]),
            "int32": (4, "ftypes.INT32", [], ["ENC_LITTLE_ENDIAN"]),
            "uint64": (8, "ftypes.UINT64", [], ["ENC_LITTLE_ENDIAN"]),
            "int64": (8, "ftypes.INT64", [], ["ENC_LITTLE_ENDIAN"]),
            "double": (8, "ftypes.DOUBLE ", [], ["ENC_LITTLE_ENDIAN"]),
            "datetime": (4, "ftypes.ABSOLUTE_TIME", [], ["ENC_LITTLE_ENDIAN"]),
            "variant": (variant_sz, "ftypes.NONE", [], ["ENC_LITTLE_ENDIAN"]),
            "string": (string_sz, "ftypes.STRINGZ", [], ["ENC_STRING+ENC_ASCII"]),
            "stationurl": (string_sz, "ftypes.STRINGZ", [], ["ENC_STRING+ENC_ASCII"]),
            "qresult": (
                4,
                "ftypes.UINT32",
                ['{[0x10001] = "Ok"}', "base.HEX"],
                ["ENC_LITTLE_ENDIAN"],
            ),
            "buffer": (string_sz, "ftypes.STRINGZ", [], ["ENC_STRING+ENC_ASCII"]),
        }[self.name]

    def to_wireshark_def(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        abbr: str,
        field_name: str,
        name: str,
    ) -> str:
        sz, field_type, args, _ = self.translate()
        if args:
            args = ", " + ", ".join(args)
        else:
            args = ""
        return f'{name} = ProtoField.new("{field_name}", "{abbr}", {field_type}{args})'

    def to_wireshark(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: "Method",
        field: "Field",
        name: str,
    ) -> str:
        sz, _, _, args = self.translate()
        if args:
            args = ", " + ", ".join(args)
        else:
            args = ""

        lines = []
        if not isinstance(sz, int):
            code, sz = sz()
            lines.append(code)
        lines.append(
            f"subtree:add_packet_field({proto_var}.fields.{name}, buffer(off, {sz}){args})"
        )
        lines.append(f"off = off + {sz}")

        return "\n".join(lines)


@dataclass
class ClassType(Type):
    name: str

    def _find_cls(self, namespace: "Namespace"):
        clzs = [clz for clz in namespace.classes if clz.name == self.name]
        if not clzs:
            clzs = [
                clz
                for ns in namespace.all_namespaces
                for clz in ns.classes
                if clz.name == self.name
            ]

        assert len(clzs) == 1, (self,)
        return clzs[0]

    def to_wireshark_def(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        abbr: str,
        field_name: str,
        name: str,
    ) -> str:
        clz = self._find_cls(namespace)
        lines = []
        if clz.base:
            lines.append(
                ClassType(clz.base).to_wireshark_def(
                    namespace, protocol, proto_var, abbr, field_name, name
                )
            )
        for field in clz.fields:
            lines.append(
                field.to_wireshark_def(
                    namespace, protocol, proto_var, method=None, clz=clz
                )
            )
        return "\n".join(lines)

    def to_wireshark(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: "Method",
        field: "Field",
        name: str,
    ) -> str:
        clz = self._find_cls(namespace)
        lines = []
        if clz.base:
            lines.append(
                ClassType(clz.base).to_wireshark(
                    namespace, protocol, proto_var, method, field, name
                )
            )
        for field in clz.fields:
            lines.append(
                field.to_wireshark(namespace, protocol, proto_var, method=None, clz=clz)
            )
        return "\n".join(lines)


@dataclass
class TemplateType(Type):
    name: str
    parameters: list[Type]

    def to_wireshark_def(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        abbr: str,
        field_name: str,
        name: str,
    ):
        assert self.name in ("any", "qlist", "std_list", "qvector", "std_map"), self
        if self.name == "any":
            assert self.parameters[0].name == "Data"
            assert self.parameters[1].name == "string"
            return self.parameters[1].to_wireshark_def(
                namespace, protocol, proto_var, abbr, field_name, name
            )
        elif self.name in ("qlist", "std_list", "qvector"):
            return self.parameters[0].to_wireshark_def(
                namespace, protocol, proto_var, abbr, field_name, name
            )
        elif self.name == "std_map":
            return (
                self.parameters[0].to_wireshark_def(
                    namespace,
                    protocol,
                    proto_var,
                    abbr + ".key",
                    field_name + " Key",
                    name + "_key",
                )
                + f"\n"
                + self.parameters[1].to_wireshark_def(
                    namespace,
                    protocol,
                    proto_var,
                    abbr + ".value",
                    field_name + " Value",
                    name + "_value",
                )
            )
        else:
            return None
        return f"{name} = {code}"

    def to_wireshark(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: "Method",
        field: "Field",
        name: str,
    ):
        lines = []
        assert self.name in ("any", "qlist", "std_list", "qvector", "std_map"), self
        if self.name == "any":
            assert self.parameters[0].name == "Data"
            assert self.parameters[1].name == "string"
            lines.append(
                self.parameters[1].to_wireshark(
                    namespace, protocol, proto_var, method, field, name
                )
            )
            # lines.append(
            #     self.parameters[0].to_wireshark(
            #         namespace, protocol, proto_var, method, field
            #     )
            # )
        elif self.name in ("qlist", "std_list", "qvector"):
            lines.append("local cnt = buffer(off, 4):le_uint()")
            lines.append("off = off + 4")
            lines.append("for i=1,cnt do")
            lines.append(
                textwrap.indent(
                    self.parameters[0].to_wireshark(
                        namespace, protocol, proto_var, method, field, name
                    ),
                    " " * 4,
                )
            )
            lines.append("end")
        elif self.name == "std_map":
            lines.append("local cnt = buffer(off, 4):le_uint()")
            lines.append("off = off + 4")
            lines.append("for i=1,cnt do")
            lines.append(
                textwrap.indent(
                    self.parameters[0].to_wireshark(
                        namespace, protocol, proto_var, method, field, name + "_key"
                    ),
                    " " * 4,
                )
            )
            lines.append(
                textwrap.indent(
                    self.parameters[1].to_wireshark(
                        namespace, protocol, proto_var, method, field, name + "_value"
                    ),
                    " " * 4,
                )
            )
            lines.append("end")

        return "\n".join(lines)


@dataclass
class Field:
    name: str
    type: Type = field(init=False)
    ty: InitVar[Any]

    def __post_init__(self, ty):
        self.type = Type.from_ty(ty)

    def to_wireshark_def(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: Optional["Method"] = None,
        clz: Optional["DDLClass"] = None,
    ):
        if method:
            name = f"{snake_case(method.name)}_{snake_case(self.name)}"
            ident = f"{snake_case(protocol.name)}.{snake_case(method.name)}.{snake_case(self.name)}"
        elif clz:
            name = f"{snake_case(clz.name)}_{snake_case(self.name)}"
            ident = f"{snake_case(protocol.name)}.{snake_case(clz.name)}.{snake_case(self.name)}"
        else:
            raise RuntimeError("method or clz required")
        try:
            definition = self.type.to_wireshark_def(
                namespace,
                protocol,
                proto_var,
                ident,
                self.name,
                f"{proto_var}.fields.{name}",
            )
        except (NotImplementedError, KeyError) as ex:
            traceback.print_exc()
            return f"-- {name} no def: {ex}"
        if definition is None:
            return f"-- {name} no def"
        return definition

    def to_wireshark(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: Optional["Method"] = None,
        clz: Optional["DDLClass"] = None,
    ):
        if method:
            name = f"{snake_case(method.name)}_{snake_case(self.name)}"
        elif clz:
            name = f"{snake_case(clz.name)}_{snake_case(self.name)}"
        else:
            raise RuntimeError("method or clz required")
        try:
            lines = [
                self.type.to_wireshark(
                    namespace, protocol, proto_var, method, self, name
                ),
            ]
            return "\n".join(lines)
        except (NotImplementedError, KeyError) as ex:
            traceback.print_exc()
            return f"-- {name} no parser: {ex}"


@dataclass
class DDLClass:
    name: str
    base: Optional[str]
    fields: list[Field] = field(default_factory=list)

    def process_fields(self, fields):
        for field in fields:
            assert tuple(field) == ("Variable",), field
            data = field["Variable"]
            self.fields.append(Field(data["name1"], ty=data["ty"]))

    def to_wireshark_def(
        self, namespace: "Namespace", protocol: "Protocol", proto_var: str
    ):
        return "\n".join(
            field.to_wireshark_def(namespace, protocol, proto_var, self)
            for field in self.fields
        )

    def to_wireshark(
        self,
        namespace: "Namespace",
        protocol: "Protocol",
        proto_var: str,
        method: "Method",
        field: "Field",
        name: str,
    ):
        pass


@dataclass
class Method:
    id: int
    name: str
    parameters: list[Field] = field(default_factory=list)
    returns: list[Field] = field(default_factory=list)

    def process_parameters(self, parameters):
        for parameter in parameters:
            assert len(parameter) == 1
            if "Parameter" in parameter:
                data = parameter["Parameter"]
                assert data["ty"] in ("Request", "Response"), data["ty"]
                is_request = data["ty"] == "Request"
                param = Field(data["name1"], ty=data["dtype1"])
            elif "ReturnValue" in parameter:
                data = parameter["ReturnValue"]
                is_request = False
                param = Field("result", ty=data["dtype1"])
                self.returns.insert(0, param)
                return
            else:
                raise RuntimeError(f"unexpected type {parameter}")

            if is_request:
                self.parameters.append(param)
            else:
                self.returns.append(param)

    def to_wireshark(
        self, namespace: "Namespace", protocol: "Protocol", proto_var: str
    ) -> str:
        req_lines = [
            f'local subtree = tree:add({proto_var}, buffer(), "{protocol.name}.{self.name}")',
            f"local off = 0",
        ]
        resp_lines = req_lines[::]
        req_lines.extend(
            param.to_wireshark(namespace, protocol, proto_var, self)
            for param in self.parameters
        )
        resp_lines.extend(
            param.to_wireshark(namespace, protocol, proto_var, self)
            for param in self.returns
        )
        req = (
            "if is_request then\n"
            + textwrap.indent("\n".join(req_lines), " " * 4)
            + "\nelse\n"
        )
        resp = textwrap.indent("\n".join(resp_lines), " " * 4) + "\nend"
        return textwrap.indent(
            f"if method_id == {self.id} then\n"
            + textwrap.indent(req, " " * 4)
            + textwrap.indent(resp, " " * 4)
            + "\nend",
            " " * 4,
        )


@dataclass
class Protocol:
    id: int | None
    name: str
    methods: list[Method] = field(default_factory=list)

    def process_methods(self, methods):
        for i, method in enumerate(methods, 1):
            assert len(method) == 1
            data = method["Method"]
            try:
                method = Method(i, data["name1"])
                method.process_parameters(data["elements1"])

                self.methods.append(method)
            except AssertionError:
                traceback.print_exc()

    def to_wireshark(self, namespace: "Namespace", classes: list[DDLClass]) -> str:
        proto_var = snake_case(namespace.name) + "_" + snake_case(self.name)
        lines = [
            f"{proto_var} = Proto({self.name!r}, {self.name!r})",
        ]
        for clz in classes:
            lines.append(clz.to_wireshark_def(namespace, self, proto_var))
        for meth in self.methods:
            for param in meth.parameters:
                lines.append(param.to_wireshark_def(namespace, self, proto_var, meth))
            for param in meth.returns:
                lines.append(param.to_wireshark_def(namespace, self, proto_var, meth))
        lines.append(
            f"""
function {proto_var}.init()
    DissectorTable.get("rmc.protocol_id"):add({self.id}, {proto_var})
end"""
        )

        lines.append(
            f"""
--- @param buffer Tvb
--- @param pinfo Pinfo
--- @param tree TreeItem
function {proto_var}.dissector(buffer, pinfo, tree)
    local method_id = method_id_field().value
    local is_request = is_request_field().value
"""
        )

        lines.extend(
            method.to_wireshark(namespace, self, proto_var) for method in self.methods
        )

        lines.append("end")

        return "\n".join(lines)

    def required_classes(self):
        def from_ty(ty):
            match ty:
                case ClassType():
                    return [ty.name]
                case TemplateType():
                    return [name for param in ty.parameters for name in from_ty(param)]
                case _:
                    return []

        classes = []
        for meth in self.methods:
            for param in meth.parameters + meth.returns:
                classes.extend(from_ty(param.type))
        return classes


@dataclass
class Namespace:
    name: str
    protocols: list[Protocol] = field(default_factory=list)
    classes: list[DDLClass] = field(default_factory=list)
    all_namespaces: list["Namespace"] = field(default_factory=list)

    def to_wireshark(self) -> str:
        header = """
local method_id_field, is_request_field, is_success_field = ...
"""
        return header + "\n".join(
            proto.to_wireshark(
                self,
                [clz for clz in self.classes if clz.name in proto.required_classes()],
            )
            for proto in self.protocols
            if proto.id is not None
        )


def process_ddls(ddls) -> list[Namespace]:
    namespaces = []
    for entry in ddls:
        namespaces.append(process_element(entry["elements"]))
    return namespaces


class Vistor:
    def process(self, entries):
        for entry in entries:
            assert len(entry) == 1
            type, data = next(iter(entry.items()))
            if handler := getattr(self, f"visit_{type}", None):
                handler(data)
            else:
                print(f"Unhandled type {type}")
                continue


class ElementVisitor(Vistor):
    def visit_DDLUnitDeclaration(self, data):
        self.namespace = Namespace(data["name1"])

    def visit_ClassDeclaration(self, data):
        assert data["namespace"] == self.namespace.name
        clz = DDLClass(data["name1"], data.get("base"))
        clz.process_fields(data["variables"])
        self.namespace.classes.append(clz)

    def visit_ProtocolDeclaration(self, data):
        assert data["namespace"] == self.namespace.name
        proto = Protocol(data["id"], data["name1"])
        proto.process_methods(data["methods"])
        self.namespace.protocols.append(proto)


def process_element(element) -> Namespace:
    visitor = ElementVisitor()
    visitor.process(element)
    return visitor.namespace


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("DDL_JSON", type=argparse.FileType("r"))

    args = parser.parse_args()

    ddls = json.load(args.DDL_JSON)
    namespaces = process_ddls(ddls)
    pprint(namespaces)

    if not TARGET_DIR.exists():
        TARGET_DIR.mkdir()

    for ns in namespaces:
        ns.all_namespaces = namespaces
        # if ns.name not in ("AuthenticationFoundation",):
        #     continue
        lua = ns.to_wireshark()
        TARGET_DIR.joinpath(f"{ns.name}.lua.noload").write_text(lua)
        # print(lua)


if __name__ == "__main__":
    main()
