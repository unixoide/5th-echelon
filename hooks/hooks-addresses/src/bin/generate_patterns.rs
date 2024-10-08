use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use argh::FromArgs;
use hooks_addresses::patterns::Pattern;
use hooks_addresses::patterns::PatternNibble;
use hooks_addresses::patterns::PatternOctet;
use hooks_config::Hook;
use iced_x86::Decoder;
use iced_x86::DecoderOptions;
use iced_x86::Formatter;
use iced_x86::Instruction;
use iced_x86::IntelFormatter;
use iced_x86::OpKind;

#[derive(Debug, FromArgs)]
/// Generate search patterns from addresses
struct Args {
    /// binary to generate patterns for
    #[argh(option)]
    binary: PathBuf,
}

fn extract_pattern(
    decoder: &mut Decoder,
    instruction: &mut Instruction,
    output: &mut String,
    formatter: &mut IntelFormatter,
    addr: usize,
    text_data: &[u8],
    text_section: &goblin::pe::section_table::SectionTable,
    image_base: usize,
) -> Option<Pattern> {
    let start_pos = addr - text_section.virtual_address as usize - image_base as usize;
    let start_ip = addr as u64;
    decoder.set_ip(start_ip);
    decoder.set_position(start_pos).unwrap();

    let mut pattern = Pattern::default();

    for i in 0..50 {
        decoder.decode_out(instruction);
        output.clear();
        formatter.format(&instruction, output);
        let pos = (instruction.ip() - start_ip) as usize + start_pos;
        let instruction_bytes = &text_data[pos..pos + instruction.len()];
        let code = instruction_bytes
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        println!("{:#x} {} {}", instruction.ip(), code, output);

        let const_off = decoder.get_constant_offsets(&instruction);

        let displ_range = if const_off.has_displacement() {
            Some(
                const_off.displacement_offset()
                    ..const_off.displacement_offset() + const_off.displacement_size(),
            )
        } else {
            None
        };

        let imm_range = if const_off.has_immediate() {
            Some(
                const_off.immediate_offset()
                    ..const_off.immediate_offset() + const_off.immediate_size(),
            )
        } else {
            None
        };

        let imm2_range = if const_off.has_immediate2() {
            Some(
                const_off.immediate_offset2()
                    ..const_off.immediate_offset2() + const_off.immediate_size2(),
            )
        } else {
            None
        };

        pattern.extend(instruction_bytes.iter().enumerate().map(|(i, b)| {
            if displ_range
                .as_ref()
                .map(|r| r.contains(&i))
                .unwrap_or(false)
                || imm_range.as_ref().map(|r| r.contains(&i)).unwrap_or(false)
                || imm2_range.as_ref().map(|r| r.contains(&i)).unwrap_or(false)
            {
                PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Wildcard])
            } else {
                PatternOctet::new(*b)
            }
        }));
        if i > 3 {
            // println!("attempting pattern: {pattern}");
            if pattern.is_unique(text_data.as_ref()) {
                println!("found pattern: {pattern}");
                return Some(pattern);
            }
        }
    }
    println!("No pattern found. Last attempt: {pattern}");
    None
}

fn main() {
    let args: Args = argh::from_env();
    println!("Trying to generate patterns for {:?}", args.binary);

    let addresses =
        hooks_addresses::get_from_path(&args.binary).expect("no known addresses for given binary");

    let binary_content = fs::read(args.binary).unwrap(); // we already hashed the binary before

    let pe = goblin::pe::PE::parse(&binary_content).expect("valid PE");
    let image_base = pe.image_base;
    let text_section = pe
        .sections
        .into_iter()
        .find(|s| dbg!(s.name.as_ref()) == b".text\0\0\0")
        .expect("text section");

    let text_data = text_section.data(&binary_content).unwrap().unwrap();
    let relocations = text_section.relocations(&binary_content).unwrap();

    println!("Known addresses for binary: {addresses:#x?}");

    let mut formatter = IntelFormatter::new();
    formatter.options_mut().set_first_operand_char_index(8);

    let mut output = String::new();
    let mut instruction = Instruction::default();

    let mut decoder = Decoder::with_ip(
        if pe.is_64 { 64 } else { 32 },
        text_data.as_ref(),
        text_section.virtual_address as u64 + image_base as u64,
        DecoderOptions::NONE,
    );

    let mut hook_patterns = Vec::new();

    for &hook in Hook::VARIANTS.iter() {
        let Some(addrs) = addresses.hook_addr(hook) else {
            println!("No known addresses for hook {hook:?}");
            continue;
        };
        if addrs.is_empty() {
            println!("No known addresses for hook {hook:?}");
            continue;
        }
        println!(
            "Generating patterns for {} addresses for hook {:?}",
            addrs.len(),
            hook
        );

        let patterns: Vec<Pattern> = addrs
            .into_iter()
            .flat_map(|addr| {
                extract_pattern(
                    &mut decoder,
                    &mut instruction,
                    &mut output,
                    &mut formatter,
                    addr,
                    text_data.as_ref(),
                    &text_section,
                    image_base,
                )
            })
            .collect();

        hook_patterns.push((hook, patterns));
    }

    println!(
        "static HOOK_PATTERNS: LazyLock<Vec<(Hook, Vec<Pattern>)>> = LazyLock::new(|| {{\nvec!["
    );

    for (hook, patterns) in hook_patterns.into_iter() {
        println!(
            "    ({}, vec![{}]),",
            hook,
            patterns
                .into_iter()
                .map(|p| format!("Pattern::from_str({:#?}).unwrap()", p.to_string()))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
    println!("    ]\n}});")
}
