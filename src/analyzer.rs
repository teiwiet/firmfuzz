use std::{fs,path::{Path,PathBuf}};
enum Arch{
    X86,
    X86_64,
    Mips,
    MipsEl,
    Arm32,
    Arm64,
    Unknown(u32)
}

impl Arch{
    fn from_elf_machine(machine : u16)->Self{
        match machine {
            0x03 => Arch::X86,
            0x3E => Arch::X86_64,
            0x08 => Arch::Mips,
            0x0A => Arch::MipsEl,
            0x28 => Arch::Arm32,
            0xB7 => Arch::Arm64,
            other => Arch::Unknown(other as u32)
        }
    }
}

pub struct BinaryReport{
    path : PathBuf,
    danger_func : Vec<String>,
    input_func : Vec<String>,
    arch: Arch,
    score : u16,
}

const DANGER_FUNC : &[&str] = &[
    "popen",
    "system",
    "gets",
    "execve",
    "strcpy",
    "strcat",
    "sprintf",
    "vsprintf",
];

const INPUT_FUNC : &[&str] = &[
    "recv",
    "recvfrom",
    "scanf",
    "read",
    "fread",
    "getevn",
    "fgets"
];
