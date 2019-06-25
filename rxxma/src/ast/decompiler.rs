use log::trace;
use std::mem::size_of;

use amxmodx_utils::amx::opcode::Opcode;
use amxmodx_utils::amx::opcode_type::OpcodeType::*;
use amxmodx_utils::amx::File as AmxFile;

use super::function_call::{Argument, FunctionCall};
use super::Function as AstFunction;
use super::Plugin as AstPlugin;
use super::TreeElementType;
use super::TreeElementType::*;

// TODO: Refactor me
const CELLSIZE: usize = size_of::<u32>();

pub struct Decompiler {
    pub amx_plugin: AmxFile,
    pub ast_plugin: AstPlugin,
}

fn read_constant_auto_type<'amx_file_bin>(amx_file: &'amx_file_bin AmxFile, addr: usize) -> Result<ConstantParam, &'amx_file_bin str> {
    if addr > (amx_file.hea() - amx_file.dat()) {
        return Ok(ConstantParam::Cell(addr as u32));
    }

    // TODO: Error handling
    let byte_slice: Vec<u8> = amx_file.dat_slice().unwrap()[addr..]
        .chunks(CELLSIZE)
        .map(|x| x[0])
        .take_while(|&x| x != 0)
        .collect();

    let string = CString::new(byte_slice).unwrap();
    Ok(ConstantParam::String(string))
}

impl Decompiler {
    pub fn from(amx_plugin: AmxFile) -> Decompiler {
        let opcodes = amx_plugin.opcodes().unwrap();

        Decompiler {
            amx_plugin,
            ast_plugin: AstPlugin::from(opcodes).unwrap(),
        }
    }

    pub fn into_tree(self) -> AstPlugin {
        self.ast_plugin
    }

    pub fn opcodes_into_functions(&mut self) {
        trace!("Pack opcodes into functions");
        let public_list = self.amx_plugin.publics().unwrap();

        let mut new_tree: Vec<TreeElementType> = vec![];
        let mut current_function: Option<AstFunction> = None;

        for element in self.ast_plugin.tree_elements.clone().into_iter() {
            let opcode = match element {
                OpcodeType(o) => o,
                _ => {
                    new_tree.push(element);
                    continue;
                }
            };

            // Open function
            if opcode.code() == OpProc {
                // TODO: Check if func already exist
                current_function = Some(AstFunction::from(&opcode, &public_list));
                continue;
            }

            // Close function
            if opcode.code() == OpRetn && current_function.is_some() {
                new_tree.push(FunctionType(current_function.unwrap()));
                current_function = None;
                continue;
            }

            // Accumulate function opcodes
            // should be the last before top level opcodes accumulation
            if let Some(f) = current_function.as_mut() {
                f.tree_elements.push(OpcodeType(opcode));
                continue;
            }

            new_tree.push(OpcodeType(opcode));
        }

        self.ast_plugin.tree_elements = new_tree;
    }

    pub fn decompile_opcodes_by_templates(&mut self) -> Result<(), &'static str> {
        self.clean_functions_break()?;
        self.decompile_native_calls()?;
        Ok(())
    }

    pub fn clean_functions_break(&mut self) -> Result<(), &'static str> {
        trace!("Clean functions from trash break");

        let ast_plugin = &mut self.ast_plugin;

        let functions: Vec<_> = ast_plugin
            .tree_elements
            .iter_mut()
            .map(|e| match *e {
                FunctionType(ref mut f) => Some(f),
                _ => None,
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        for function in functions {
            let mut addr = 0;
            let current_tree = &mut function.tree_elements;

            // Iterate and modify over function tree
            while addr < current_tree.len() {
                addr += 1;
                let position = addr - 1;

                let opcode = match current_tree[position] {
                    OpcodeType(o) => o,
                    _ => continue,
                };

                if opcode.code() != OpBreak {
                    break;
                }

                current_tree.remove(position);
                addr -= 1;
            }
        }
        Ok(())
    }

    pub fn decompile_native_calls(&mut self) -> Result<(), &'static str> {
        trace!("Decompile native calls");
        let ast_plugin = &mut self.ast_plugin;
        let amx_plugin = &mut self.amx_plugin;
        // TODO: Error handling
        let natives = amx_plugin.natives().unwrap();

        let functions: Vec<_> = ast_plugin
            .tree_elements
            .iter_mut()
            .map(|e| match *e {
                FunctionType(ref mut f) => Some(f),
                _ => None,
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        for function in functions {
            let mut addr = 0;
            let current_tree = &mut function.tree_elements;

            // Iterate and modify over function tree
            while addr < current_tree.len() {
                addr += 1;
                let mut position = addr - 1;

                let opcode = match current_tree[position] {
                    OpcodeType(o) => o,
                    _ => continue,
                };

                if opcode.code() == OpSysreqC {
                    let sysreq_opcode = &opcode;
                    // Take previous PUSH.C to get args count
                    let native_arguments_count = {
                        let element = &current_tree[position - 1];

                        let opcode = match element {
                            OpcodeType(o) => o,
                            _ => continue,
                        };

                        // Weird native call, ignore
                        if opcode.code() != OpPushC {
                            trace!("Native call got no arguments definition");
                            continue;
                        }

                        opcode.argument().unwrap() as usize / CELLSIZE
                    };

                    // Sysreq.c (current) - PUSH.c with args count - args count
                    let args_start = position - 1 - native_arguments_count;
                    // Sysreq.c (current) - PUSH.c with args count
                    let args_end = position - 1;
                    let raw_args: Vec<_> = current_tree.drain(args_start..args_end).collect();
                    addr -= args_end - args_start;
                    position -= args_end - args_start;

                    // Remove push.c with num of args
                    current_tree.remove(position - 1);
                    addr -= 1;
                    position -= 1;

                    let is_having_non_opcodes = raw_args.iter().any(|e| match *e {
                        OpcodeType(_) => false,
                        _ => true,
                    });

                    // Internal error
                    // Something else modified ast tree
                    if is_having_non_opcodes {
                        trace!(
                            "Function tree was modified by bad AST transformation. Ignoring call"
                        );
                        continue;
                    }

                    let args_opcodes: Vec<Opcode> = raw_args
                        .iter()
                        .map(|e| match *e {
                            OpcodeType(o) => Some(o),
                            _ => None,
                        })
                        .filter(Option::is_some)
                        .map(Option::unwrap)
                        .collect();

                    let is_having_non_push_c_opcodes =
                        args_opcodes.iter().any(|o| o.code() != OpPushC);
                    if is_having_non_push_c_opcodes {
                        trace!("Invalid native call arguments");
                        continue;
                    }

                    let native_args: Vec<_> = args_opcodes
                        .iter()
                        .map(|o| o.argument().unwrap())
                        .map(|addr| amx_plugin.read_constant_auto_type(addr as usize).unwrap())
                        .map(Argument::from)
                        .rev()
                        .collect();

                    let native_index = sysreq_opcode.argument().unwrap() as usize;
                    let native_name = (&natives[native_index].name).to_string();

                    let ast_function_call = FunctionCall {
                        name: native_name,
                        args: Some(native_args),
                    };

                    current_tree[position] = FunctionCallType(ast_function_call);

                    // Check for trash OpStack
                    {
                        let opcode_position = position + 1;

                        let opcode = {
                            let element = match current_tree.get(opcode_position) {
                                Some(e) => e,
                                None => break,
                            };

                            match element {
                                OpcodeType(o) => o,
                                _ => break,
                            }
                        };

                        if opcode.code() == OpStack {
                            current_tree.remove(opcode_position);
                        }
                    }

                    // Check for trash OpZeroPri
                    {
                        let opcode_position = position + 1;

                        let opcode = {
                            let element = match current_tree.get(opcode_position) {
                                Some(e) => e,
                                None => break,
                            };

                            match element {
                                OpcodeType(o) => o,
                                _ => break,
                            }
                        };

                        if opcode.code() == OpZeroPri {
                            current_tree.remove(opcode_position);
                        }
                    }

                    // Check for trash OpBreak
                    {
                        let opcode_position = position + 1;

                        let opcode = {
                            let element = match current_tree.get(opcode_position) {
                                Some(e) => e,
                                None => break,
                            };

                            match element {
                                OpcodeType(o) => o,
                                _ => break,
                            }
                        };

                        if opcode.code() == OpBreak {
                            current_tree.remove(opcode_position);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
