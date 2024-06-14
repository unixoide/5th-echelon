#!/usr/bin/env python3
import argparse
from collections import defaultdict
import json
import textwrap
from typing import Literal, TypedDict
import pathlib
import re

ROOT = pathlib.Path(__file__).parent.parent.absolute()
SRC = ROOT / "src"
QUAZAL_ROOT = SRC / "research" / "network" / "quazal"
RMC_ROOT = QUAZAL_ROOT / "rmc"

START_MARKER = "<!-- INSERT {} START -->".format
END_MARKER = "<!-- INSERT {} END -->".format


class DDLUnitDeclaration(TypedDict):
    name1: str
    name2: str
    name3: str
    name4: str
    location: str


class ProtocolDeclaration(TypedDict):
    name1: str
    name2: str
    namespace: str
    u1: int
    methods: list["Element"]
    id: int


class Method(TypedDict):
    name1: str
    name2: str
    u1: int
    u2: int
    elements1: list["Element"]
    elements2: list["Element"]

class Template(TypedDict):
    name: str
    template_name: str
    parameters: list['Type']

class Type(TypedDict):
    Simple: str
    Class: str
    Template: Template

class DataType(TypedDict):
    ty: Type

class Parameter(TypedDict):
    name1: str
    name2: str
    dtype1: DataType
    dtype2: DataType
    ty: Literal["Request"]|Literal["Response"]


class Element(TypedDict):
    DDLUnitDeclaration: DDLUnitDeclaration
    ProtocolDeclaration: ProtocolDeclaration
    Method: Method
    Parameter: Parameter


class DDLUnit(TypedDict):
    elements: list[Element]


def name2fname(name: str) -> str:
    return re.sub(r"(?<=[^A-Z])([A-Z]+)", lambda m: "_" + m[1], name).lower()


def replace_content(fname: str | pathlib.Path, content: str, section: str = "default"):
    if isinstance(fname, str):
        fname = pathlib.Path(fname)

    data = fname.read_text()
    start_marker = START_MARKER(section)
    end_marker = END_MARKER(section)
    start_marker_re = re.escape(start_marker)
    end_marker_re = re.escape(end_marker)

    def repl(m: re.Match[str]) -> str:
        prefix = m[1]
        txt = textwrap.indent(text=content, prefix=prefix)
        return prefix + start_marker + "\n" + txt + "\n" + prefix + end_marker

    new_data = re.sub(
        rf"^( *){start_marker_re}$.*^\s*{end_marker_re}$",
        repl,
        data,
        count=1,
        flags=re.MULTILINE | re.DOTALL,
    )
    fname.write_text(new_data)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("ddl_json", metavar="ddl.json", type=argparse.FileType("r"))
    # parser.add_argument('mapping.json', type=argparse.FileType('r'))

    args = parser.parse_args()

    if not RMC_ROOT.exists():
        RMC_ROOT.mkdir(parents=True, exist_ok=True)

    units: list[DDLUnit] = json.load(args.ddl_json)

    ddl_unit_names: list[str] = []
    protocols: dict[str, list[ProtocolDeclaration]] = defaultdict(list)
    for unit in units:
        for element in unit["elements"]:
            if "DDLUnitDeclaration" in element:
                ddl_unit_names.append(element["DDLUnitDeclaration"]["name1"])
            if p := element.get("ProtocolDeclaration"):
                protocols[p["namespace"]].append(p)

    replace_content(
        RMC_ROOT / "units.md", "\n".join(f"- {name}" for name in ddl_unit_names)
    )

    replace_content(
        SRC / "SUMMARY.md",
        "\n".join(
            f"- [{name}](./research/network/quazal/rmc/{name2fname(name)}.md)"
            for name in ddl_unit_names
        ),
        "units",
    )

    for ns, protos in protocols.items():
        fpath = RMC_ROOT / (name2fname(ns) + ".md")

        lines: list[str] = ["| Protocol ID | Name |", "|-------------|------|"]
        for proto in protos:
            lines.append(
                f'| {proto["id"] or "?"} | [{proto["name1"]}](#{proto["name1"].lower()}) |'
            )

        replace_content(fpath, "\n".join(lines), "protocol_idx")

        lines: list[str] = []
        for proto in protos:
            lines.append(f'## {proto["name1"]}')
            lines.append("<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>")
            lines.append("<tbody>")
            for i, el in enumerate(proto["methods"], 1):
                if m := el.get("Method"):
                    def get_type2(ty: Type) -> str:
                        if s := ty.get("Simple"):
                            return s
                        if c := ty.get("Class"):
                            return c
                        if t := ty.get("Template"):
                            params = ', '.join(
                                get_type2(p)
                                for p in t["parameters"]
                            )
                            return f'{t["template_name"]}<{params}>'
                        return ty

                    def get_type(dt: DataType) -> str:
                        return get_type2(dt["ty"])
                    
                    params = [e["Parameter"] for e in m["elements1"] if "Parameter" in e]
                    req_params = ", ".join(f"{p["name1"]}: {get_type(p["dtype1"])}" for p in params if p["ty"] == "Request")
                    res_params = [f"retval: {get_type(e["ReturnValue"]["dtype1"])}" for e in m["elements1"] if "ReturnValue" in e] + [f"{p["name1"]}: {get_type(p["dtype1"])}" for p in params if p["ty"] == "Response"]
                    res_params = ", ".join(res_params)
                    lines.append(f"<tr><td>{i}</td><td>\n\n```swift\nfunc {m["name1"]}({req_params}) -> ({res_params})\n```\n\n</td></tr>")
            lines.append("</tbody></table>")

        replace_content(fpath, "\n".join(lines), "protocols")


if __name__ == "__main__":
    main()
