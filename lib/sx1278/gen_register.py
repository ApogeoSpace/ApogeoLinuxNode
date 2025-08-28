#!/bin/env python

import sys
import os
import re

def snake_case(string):
    string = re.sub(r'(?<=[a-z0-9])(?=[A-Z0-9])|[^a-zA-Z0-9]', ' ', string).strip().replace(' ', '_')
    return ''.join(string.lower())

NL = "\n"

class Field:
    def __init__(self, name, size, flag, desc="", skip=False):
        self.name = name
        self.size = int(size)
        self.values = []
        self.skip = skip
        self.flag = flag
        self.desc = desc
        
    def fname(self):
        if self.skip:
            return "__"
        return snake_case(self.name)
    
    def ftype(self):
        if len(self.values) == 0:
            # simple integer field
            return f"B{self.size}" if self.size > 1 else "bool"
        else:
            return self.name

    def pre(self):
        if self.skip:
            return "    #[skip]"
        
        rwflags = ""
        # if 'R' not in self.flag:
        #     rwflags += "#[skip(getters)]"
        # if 'W' not in self.flag:
        #     rwflags += "#[skip(setters)]"
        
        if len(self.values) == 0:
            return f"{rwflags} pub"
        else:
            return f"    /// {self.desc}{NL}    {rwflags} #[bits = {self.size}] pub"
        
    def serialize_single(self, f):
        if len(self.values) == 0:
            return
        
        rep = "u8" if self.size <= 8 else ("u16" if self.size <= 16 else ("u32" if self.size <= 32 else "u64"))

        f.write(f"""
/// {self.desc}
#[derive(Clone, Copy)]
#[repr({rep})]
pub enum {self.name}Value {{
{NL.join([f'    /// {d}{NL}    {n} = {v},' for n, v, d in self.values])}
}}

impl TryFrom<{rep}> for {self.name}Value {{
    type Error = SX1278Error;
    
    fn try_from(value: {rep}) -> Result<Self, Self::Error> {{
        return match value {{
{NL.join([f'            {v} => Ok({self.name}Value::{n}),' for n, v, _ in self.values])}
            _ => Err(SX1278Error::ParseError)
        }}
    }}
}}
""")
        
    def serialize(self, f):
        if len(self.values) == 0:
            return

        f.write(f"""
/// {self.desc}
#[derive(Specifier)]
#[bits = {self.size}]
#[derive(Clone, Copy)]
pub enum {self.name} {{
{NL.join([f'    /// {d}{NL}    {n} = {v},' for n, v, d in self.values])}
}}
""")

class Register:
    def __init__(self, name, address, length, mode, flag, desc):
        self.name = name
        self.address = int(address, 16)
        self.length = int(length)
        self.mode = mode
        self.flag = flag
        self.fields = []
        self.desc = desc
        self.underlying_types = ["-", "u8", "u16", "u32", "u32", "u64", "u64", "u64", "u64", "u128", "u128", "u128", "u128", "u128", "u128", "u128", "u128"]
        self.underlying_types_buflen = [0, 1, 2, 4, 4, 8, 8, 8,8, 8, 16, 16, 16, 16, 16, 16, 16]
        
    def serialize_bitfield(self, f):
        f.write("""
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::{SX1278Mode, LORA, FSK};
use modular_bitfield::prelude::*;
""")
        for field in self.fields:
            field.serialize(f)
    
        f.write(f"""
/// {self.desc}
#[bitfield(bits = {self.length * 8})]
#[derive(Clone, Copy)]
pub struct {self.name} {{
{NL.join([f'{x.pre()} {x.fname()}: {x.ftype()},' for x in self.fields])}
}}
""")
        
        
        if 'RW' == self.flag:
            f.write(f"""
impl{'<MODE: SX1278Mode>' if self.mode == 'ALL' else ''} Register<{'MODE' if self.mode == 'ALL' else self.mode}> for {self.name}{{}}
""")
        
        if 'R' in self.flag:
            f.write(f"""
impl{'<MODE: SX1278Mode>' if self.mode == 'ALL' else ''} ReadableRegister<{'MODE' if self.mode == 'ALL' else self.mode}> for {self.name}{{
    const ADDRESS: u8 = {self.address};
    const SIZE: usize = {self.length};
}}
impl<'x> TryFrom<&'x[u8]> for {self.name} {{
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {{
        if v.len() < {self.length} {{
            return Err(SX1278Error::ParseError)
        }}
        let mut b: [u8; {self.length}] = v.try_into().unwrap();
        {'b.reverse();' if self.length > 1 else ''}
        Ok(Self::from_bytes(b))
    }}
}}
""")
        
        if 'W' in self.flag:
            f.write(f"""
impl{'<MODE: SX1278Mode>' if self.mode == 'ALL' else ''} WritableRegister<{'MODE' if self.mode == 'ALL' else self.mode}> for {self.name}{{
    const ADDRESS: u8 = {self.address};
}}
impl TryWriteInto<[u8], usize> for {self.name} {{
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{{
        if dest.len() < {self.length} {{
            return Err(SX1278Error::SerializationError)
        }}
        let mut repr = self.into_bytes();
        {'repr.reverse();' if self.length > 1 else ''}
        dest[..{self.length}].copy_from_slice(&repr);
        return Ok({self.length})
    }}
}}
""")
            
    def serialize(self, f):
        real_fields = list(filter(lambda x: not x.skip, self.fields))
        if len(real_fields) > 1:
            self.serialize_bitfield(f)
            return
                
        struct_shift = 0 if len(self.fields) == 0 or not self.fields[0].skip else self.fields[0].size
        struct_field_size = 8*self.length if len(real_fields) == 0 else real_fields[0].size
        struct_mask = ((1 << struct_field_size) - 1) << struct_shift
        from_type = self.underlying_types[self.length]
                
        f.write("""
#![allow(dead_code)]

use crate::registers::{Register, ReadableRegister, WritableRegister, TryWriteInto};
use crate::error::SX1278Error;
use crate::{SX1278Mode, LORA, FSK};
""")
        
        if len(real_fields) > 0 and len(real_fields[0].values) > 0:
            for field in real_fields:
                field.serialize_single(f)
                from_type = field.name + "Value"
        
        f.write(f"""
/// {self.desc}
#[derive(Clone, Copy)]
pub struct {self.name} ({self.underlying_types[self.length]});
""")
        
        
        if 'RW' == self.flag:
            f.write(f"""
impl{'<MODE: SX1278Mode>' if self.mode == 'ALL' else ''} Register<{'MODE' if self.mode == 'ALL' else self.mode}> for {self.name}{{}}
""")
        
        if 'R' in self.flag:
            f.write(f"""
impl{'<MODE: SX1278Mode>' if self.mode == 'ALL' else ''} ReadableRegister<{'MODE' if self.mode == 'ALL' else self.mode}> for {self.name}{{
    const ADDRESS: u8 = {self.address};
    const SIZE: usize = {self.length};
}}
impl<'x> TryFrom<&'x[u8]> for {self.name} {{
    type Error = SX1278Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {{
        if v.len() < {self.length} {{
            return Err(SX1278Error::ParseError)
        }}
        let mut buf = [0u8; {self.underlying_types_buflen[self.length]}];
        buf[{self.underlying_types[self.length] - self.length}..].copy_from_slice(v);
        Ok(Self({self.underlying_types[self.length]}::from_be_bytes(buf)))
    }}
}}
""")
        
        if len(real_fields) > 0 and len(real_fields[0].values) > 0:
            f.write(f"""
impl TryInto<{from_type}> for {self.name} {{
    type Error = SX1278Error;
    
    fn try_into(self) -> Result<{from_type}, Self::Error> {{
        {from_type}::try_from((self.0 & {struct_mask}) >> {struct_shift})
    }}
}}
""")
        else:
            f.write(f"""
impl Into<{self.underlying_types[self.length]}> for {self.name} {{
    fn into(self) -> {self.underlying_types[self.length]} {{
        (self.0 & {struct_mask}) >> {struct_shift}
    }}
}}
""")
        
        if 'W' in self.flag:
            f.write(f"""
impl{'<MODE: SX1278Mode>' if self.mode == 'ALL' else ''} WritableRegister<{'MODE' if self.mode == 'ALL' else self.mode}> for {self.name}{{
    const ADDRESS: u8 = {self.address};
    const SIZE: usize = {self.length};
}}
impl TryWriteInto<[u8], usize> for {self.name} {{
    type Error = SX1278Error;

    fn try_write_into(&self, dest: &mut [u8]) -> Result<usize, Self::Error>{{
        if dest.len() < {self.length} {{
            return Err(SX1278Error::SerializationError)
        }}
        dest.copy_from_slice(&self.0.to_be_bytes()[{self.underlying_types[self.length] - self.length}..]);
        return Ok({self.length})
    }}
}}
impl From<{from_type}> for {self.name} {{
    fn from(v: {from_type}) -> Self {{
        Self(((v as { self.underlying_types[self.length] }) << {struct_shift}) & {struct_mask})
    }}
}}
""")
        

    



if __name__ == '__main__':
    with open(sys.argv[1], "r") as r:
        lines = r.read().split("\n")
        
    regs = []
    cur_reg = None
    cur_field = None

    for line_no, line in enumerate(lines):
        try:
            line = line.strip()
            if line == "" or line.startswith("#"):
                continue
            
            if line.startswith("REG"):
                if cur_reg is not None:
                    if cur_field is not None:
                        cur_reg.fields.append(cur_field)
                    cur_reg.fields = cur_reg.fields[::-1]
                    regs.append(cur_reg)
                _, name, addr, l, mode, flag, desc = line.split(" ", 6)
                cur_reg = Register(name, addr, l, mode, flag, desc)
                cur_field = None
            elif line.startswith("FIELD"):
                if cur_field is not None:
                    cur_reg.fields.append(cur_field)
                _, sz, name, flag, desc = line.split(" ", 4)
                cur_field = Field(name, sz, flag, desc)
            elif line.startswith("SKIP"):
                if cur_field is not None:
                    cur_reg.fields.append(cur_field)
                _, sz = line.split(" ")
                cur_field = Field("__", sz, "R", skip=True)
            elif line.startswith("VALUE"):
                _, v, name, desc = line.split(" ", 3)
                cur_field.values.append((name, v, desc))
        except Exception as e:
            print("Parsing Error at line", line_no + 1)
            print(e)
            exit(1)

    if cur_field is not None:
        cur_reg.fields.append(cur_field)
    if cur_reg is not None:
        cur_reg.fields = cur_reg.fields[::-1]
        regs.append(cur_reg)
        
    mods = {}
        
    for reg in regs:
        fname = f"./src/registers/{reg.mode.lower()}/{snake_case(reg.name)}.rs"
        if not reg.mode.lower() in mods:
            mods[reg.mode.lower()] = []
        mods[reg.mode.lower()].append(f"pub mod {snake_case(reg.name)};")
        # if os.path.exists(fname):
        #     print(f"Skipping file {fname}: exists")
        #     continue
        print(f"Generating file {fname}")
        with open(fname, "w") as f:
            reg.serialize(f)

    for k, v in mods.items():
        with open(f"./src/registers/{k}.rs", "w") as m:
            m.write("\n".join(v) + "\n")

