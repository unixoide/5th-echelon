#!/usr/bin/env python3
import json
import sys
import re
import os
import pathlib

BASE = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else None


def snake(s: str) -> str:
    def repl(m: re.Match[str]) -> str:
        s = ""
        if m.start() > 0:
            s += "_"
        g = m.group(0)
        if len(g) > 1:
            s += g[:-1]
            s += "_"
            s += g[-1]
        else:
            s += g
        return s.lower()

    return re.sub(r"[A-Z]+", repl, s)


def close():
    if (
        len(sys.argv) > 2
        and os.path.isdir(sys.argv[2])
        and sys.stdout is not sys.__stdout__
    ):
        sys.stdout.close()
        sys.stdout = sys.__stdout__


with open(sys.argv[1]) as f:
    DDL = json.load(f)

if BASE and not BASE.is_dir():
    sys.stdout = BASE.open("w")

for element in DDL["elements"]:
    cls = element["__class__"]
    if cls == "DDLUnitDeclaration":
        if BASE and BASE.is_dir():
            BASE = BASE / snake(element["name1"])
        BASE.mkdir(parents=True, exist_ok=True)
    elif cls == "ProtocolDeclaration":
        name = element["name1"]
        enum_name = name + "Method"
        _id = element["_id"]
        if BASE and BASE.is_dir():
            close()
            sys.stdout = (BASE / (snake(name) + ".rs")).open("w")
        variants = [f"{m['name1']} = {i}," for i, m in enumerate(element["methods"], 1)]
        meth_len = len(variants)
        variants = "\n".join(variants)
        match_arms = "\n".join(
            f"Some({enum_name}::{m['name1']}) => self.handle_{snake(m['name1'])}(request, {m['name1']}Request::from_bytes(&request.parameters)),"
            for i, m in enumerate(element["methods"], 1)
        )
        methods = "\n".join(
            f"fn handle_{snake(m['name1'])}(&self, request: &Request, data: std::io::Result<{m['name1']}Request>) -> Response {{ self.unknown_error(request) }}"
            for m in element["methods"]
        )
        requests = "\n\n".join(
            f"#[derive(Debug, FromStream)]\npub struct {m['name1']}Request {{ }}"
            for m in element["methods"]
        )
        print(
            rf"""
        // AUTO GENERATED FILE
        use std::convert::TryInto;
        use std::net::SocketAddr;

        use crate::prudp::packet::QPacket;
        use crate::rmc::basic::FromStream;
        use crate::rmc::Protocol;
        use crate::rmc::{{Request, Response, ResponseError}};
        use crate::ClientInfo;
        use num_enum::{{IntoPrimitive, TryFromPrimitive}};

        #[repr(u32)]
        #[derive(Debug, TryFromPrimitive, IntoPrimitive)]
        enum {enum_name} {{
          {variants}
        }}

        #[derive(Debug)]
        pub struct {name};
        """
        )

        if _id is None:
            print("// No protocol ID")
            continue

        print(
            rf"""
        impl Protocol for {name} {{
          fn id(&self) -> u16 {{ {_id} }}

          fn name(&self) -> String {{ "{name}".into() }}

          fn num_methods(&self) -> u32 {{ {meth_len} }}

          fn method_name(&self, method_id: u32) -> String {{
            let m: Option<{enum_name}> = method_id.try_into().ok();
            match m {{
              Some(m) => format!("{{:?}}", m),
              None => format!("Unknown Method {{}}", method_id),
            }}
          }}

          fn handle(&self,
             ci: &mut ClientInfo,
             src: &SocketAddr,
             packet: &QPacket,
             request: &Request) -> Response
          where
            Self: {name}Trait,
          {{
             let m: Option<{enum_name}> = request.method_id.try_into().ok();
             match m {{
               {match_arms}
               None => Response {{
                  protocol_id: request.protocol_id,
                  result: Err(ResponseError {{
                      error_code: 0x80010001,
                      call_id: request.call_id,
                  }}),
               }},
             }}
           }}
        }}

        pub trait {name}Trait {{
          fn unknown_error(&self, request: &Request) -> Response {{
            Response {{
                protocol_id: request.protocol_id,
                result: Err(ResponseError {{
                    error_code: 0x80010001,
                    call_id: request.call_id,
                }}),
            }}
          }}

          {methods}
        }}
        """
        )
        print(requests)
    elif cls == "ClassDeclaration":
        if BASE and BASE.is_dir():
            close()
            sys.stdout = (BASE / "types.rs").open("a")
        name = element["name1"]
        print(
            rf"""
        pub struct {name} {{}}
        """
        )
    elif cls == "TemplateInstance":
        pass
    else:
        print(element, file=sys.stderr)
        break

close()
