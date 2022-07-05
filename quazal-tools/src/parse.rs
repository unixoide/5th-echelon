use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::error::context;
use nom::error::ParseError;
use nom::multi::length_count;
use nom::multi::length_data;
use nom::number::streaming::be_u32;
use nom::number::streaming::be_u8;
use nom::sequence::preceded;
use nom::sequence::tuple;
use quazal_tools_macros::DDLParser;
use serde::Serialize;
use std::io;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;

#[cfg(verbose_errors)]
type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;
#[cfg(not(verbose_errors))]
type IResult<I, O> = nom::IResult<I, O, nom::error::Error<I>>;

pub static DEBUG: AtomicBool = AtomicBool::new(false);

#[allow(dead_code)]
#[derive(Debug)]
pub struct DDLHeader {
    magic: u32,
    unknown: u8,
    major: u32,
    minor: u32,
    micro: u32,
    build: u32,
}

pub fn parse_ddl(data: &[u8], offset: usize) -> io::Result<(&[u8], Namespace)> {
    let mut rdr = Cursor::new(data);
    let _header = DDLHeader {
        magic: rdr.read_u32::<BigEndian>()?,
        unknown: rdr.read_u8()?,
        major: rdr.read_u32::<BigEndian>()?,
        minor: rdr.read_u32::<BigEndian>()?,
        micro: rdr.read_u32::<BigEndian>()?,
        build: rdr.read_u32::<BigEndian>()?,
    };

    let data = &data[21..];

    #[cfg(verbose_errors)]
    const NBYTES: usize = 50;

    match namespace(data) {
        Ok(r) => Ok(r),
        Err(e) => match e {
            nom::Err::Incomplete(_) => panic!("{}", e),
            nom::Err::Error(e) => {
                let off = offset + 21;
                #[cfg(verbose_errors)]
                {
                    let details = e
                        .errors
                        .into_iter()
                        .map(|(input, kind)| {
                            format!(
                                "Error: {:?} with {:02x?} ({}) at {:#x}",
                                kind,
                                &input[..nbytes],
                                String::from_utf8_lossy(&input[..nbytes]),
                                off - input.len() + data.len(),
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    panic!("Parsing error: {}", details);
                }
                #[cfg(not(verbose_errors))]
                {
                    panic!("Parsing error: {:?} somewhere after {:#x}", e.code, off);
                }
            }
            nom::Err::Failure(e) => {
                let off = offset + 21;
                #[cfg(verbose_errors)]
                {
                    let details = e
                        .errors
                        .into_iter()
                        .map(|(input, kind)| {
                            format!(
                                "Error: {:?} with {:02x?} ({}) at {:#x}",
                                kind,
                                &input[..nbytes],
                                String::from_utf8_lossy(&input[..nbytes]),
                                off - input.len() + data.len(),
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    panic!("Parsing failure: {}", details);
                }
                #[cfg(not(verbose_errors))]
                {
                    panic!("Parsing failure: {:?} somewhere after {:#x}", e.code, off);
                }
            }
        },
    }
}

#[derive(Debug, DDLParser, Serialize)]
pub struct DDLUnitDeclaration {
    pub name1: String,
    pub name2: String,
    pub name3: String,
    #[count(u32)]
    pub elements: Vec<Element>,
    pub name4: String,
    pub location: String,
}

#[derive(Debug, DDLParser, Serialize)]
#[repr(u8)]
pub enum Element {
    DOClassDeclaration(DOClassDeclaration) = 0x03,
    DatasetDeclaration(DatasetDeclaration) = 0x04,
    Variable(Variable) = 0x06,
    Method(Method) = 0x08,
    Action(Action) = 0x09,
    PropertyDeclaration(PropertyDeclaration) = 0x0b,
    ProtocolDeclaration(ProtocolDeclaration) = 0x0c,
    Parameter(Parameter) = 0x0d,
    ReturnValue(ReturnValue) = 0x0e,
    ClassDeclaration(ClassDeclaration) = 0x0f,
    TemplateDeclaration(TemplateDeclaration) = 0x10,
    SimpleDeclaration(SimpleDeclaration) = 0x11,
    TemplateInstance(TemplateInstance) = 0x12,
    DDLUnitDeclaration(DDLUnitDeclaration) = 0x13,
    DupSpaceDeclaration(DupSpaceDeclaration) = 0x14,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct ClassDeclaration {
    pub name1: String,
    pub name2: String,
    pub namespace: String,
    #[count(u32)]
    pub properties: Vec<Element>,
    pub maybe_base: String,
    #[count(u32)]
    pub variables: Vec<Element>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct Namespace {
    #[count(u32)]
    pub elements: Vec<Element>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct PropertyDeclaration {
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub u1: u32,
    pub u2: u32,
    pub u3: u32,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct Variable {
    pub name1: String,
    pub name2: String,
    pub ty: VariableType,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct VariableType {
    pub ty: SubType,
    pub unknown: u32,
}

#[derive(Debug, DDLParser, Serialize)]
#[repr(u8)]
pub enum SubType {
    DOClass(String) = 0x03,
    Dataset(String) = 0x04,
    Class(String) = 0x0f,
    Simple(String) = 0x11,
    Template(TemplateType) = 0x12,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct TemplateType {
    pub name: String,
    pub template_name: String,
    #[count(u8)]
    pub parameters: Vec<SubType>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct TemplateInstance {
    pub name1: String,
    pub name2: String,
    pub u1: u32,
    pub u2: u32,
    pub templ_name: String,
    #[count(u32)]
    pub parameters: Vec<String>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct ProtocolDeclaration {
    pub name1: String,
    pub name2: String,
    pub namespace: String,
    pub u1: u32,
    #[count(u32)]
    pub methods: Vec<Element>,
    #[skip]
    pub id: Option<u16>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct Method {
    pub name1: String,
    pub name2: String,
    pub u1: u32,
    pub u2: u32,
    #[count(u32)]
    pub elements1: Vec<Element>,
    #[count(u32)]
    pub elements2: Vec<Element>,
}

#[derive(Debug, DDLParser, Serialize)]
#[repr(u8)]
pub enum ParameterType {
    Request = 1,
    Response = 2,
    Unknown = 3,
    Unknown2 = 0,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct Parameter {
    pub name1: String,
    pub name2: String,
    pub dtype1: VariableType,
    pub dtype2: VariableType,
    pub ty: ParameterType,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct SimpleDeclaration {
    pub name1: String,
    pub name2: String,
    pub namespace: String,
    pub u1: u32,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct TemplateDeclaration {
    pub name1: String,
    pub name2: String,
    pub namespace: String,
    pub u1: u32,
    pub u2: u32, // num params?
}

#[derive(Debug, DDLParser, Serialize)]
pub struct ReturnValue {
    pub name1: String,
    pub name2: String,
    pub dtype1: VariableType,
    pub dtype2: VariableType,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct DatasetDeclaration {
    pub name1: String,
    pub name2: String,
    pub s1: String,
    #[count(u32)]
    pub properties: Vec<Element>,
    #[count(u32)]
    pub variables: Vec<Element>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct DOClassDeclaration {
    pub name1: String,
    pub name2: String,
    pub s1: String,
    #[count(u32)]
    pub properties: Vec<Element>,
    pub s2: String,
    pub u1: u32,
    #[count(u32)]
    pub elements: Vec<Element>,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct Action {
    pub name1: String,
    pub name2: String,
    pub u1: u32,
    pub u2: u32,
    #[count(u32)]
    pub elements: Vec<Element>,
    pub u3: u32,
}

#[derive(Debug, DDLParser, Serialize)]
pub struct DupSpaceDeclaration {
    pub name1: String,
    pub name2: String,
    pub namespace: String,
    #[count(u32)]
    pub elements: Vec<Element>,
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, data) = length_data(be_u32)(input)?;
    Ok((
        input,
        dump_value(String::from_utf8(data.to_vec()).expect("utf8 string")),
    ))
}

#[allow(dead_code)]
fn boxed<I, O, E, F>(f: F) -> impl Fn(I) -> nom::IResult<I, Box<O>, E>
where
    I: Clone + PartialEq,
    // F: Parser<I, O, E>,
    F: Fn(I) -> nom::IResult<I, O, E>,
    E: ParseError<I>,
{
    // move |i: I| f.parse(i).map(|(i, o)| (i, Box::new(o)))
    move |i: I| f(i).map(|(i, o)| (i, Box::new(o)))
}

fn dbg_dmp<'a, F, O>(
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a [u8]) -> nom::IResult<&'a [u8], O, nom::error::Error<&'a [u8]>>
where
    F: FnMut(&'a [u8]) -> nom::IResult<&'a [u8], O, nom::error::Error<&'a [u8]>>,
{
    use nom::HexDisplay;
    move |i: &'a [u8]| match f(i) {
        Err(e) => {
            let hd = &i[..512].to_hex(16);
            match &e {
                nom::Err::Incomplete(e) => {
                    println!("{}: Incomplete({:?}) at:\n{}", context, e, hd)
                }
                nom::Err::Error(e) => println!("{}: Error({:?}) at:\n{}", context, e.code, hd),
                nom::Err::Failure(e) => println!("{}: Failure({:?}) at:\n{}", context, e.code, hd),
            };
            Err(e)
        }
        a => a,
    }
}

fn dump_value<T: std::fmt::Debug>(value: T) -> T {
    if DEBUG.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("{:#?}", value);
    }
    value
}
