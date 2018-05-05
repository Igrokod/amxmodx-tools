use super::Function as AstFunction;
use super::Plugin as AstPlugin;
use super::TreeElementType;
use super::TreeElementType::*;
use super::function_call::{Argument, FunctionCall};
use super::super::amxmod::CELLSIZE;
use super::super::amxmod::Opcode;
use super::super::amxmod::OpcodeType::*;
use super::super::amxmod::Plugin as AmxPlugin;

pub struct Decompiler {
    pub amx_plugin: AmxPlugin,
    pub ast_plugin: AstPlugin,
}

impl Decompiler {
    pub fn from(amx_plugin: AmxPlugin) -> Decompiler {
        let opcodes = amx_plugin.opcodes().unwrap();

        Decompiler {
            amx_plugin: amx_plugin,
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
            if opcode.code == OP_PROC {
                // TODO: Check if func already exist
                current_function = Some(AstFunction::from(&opcode, &public_list));
                continue;
            }

            // Close function
            if opcode.code == OP_RETN && current_function.is_some() {
                new_tree.push(FunctionType(current_function.unwrap()));
                current_function = None;
                continue;
            }

            // Accumulate function opcodes
            // should be the last before top level opcodes accumulation
            if let Some(mut f) = current_function.as_mut() {
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
            .map(|e| match e {
                &mut FunctionType(ref mut f) => Some(f),
                _ => None,
            })
            .filter(|e| e.is_some())
            .map(|f| f.unwrap())
            .collect();

        for function in functions {
            let mut addr = 0;
            let mut current_tree = &mut function.tree_elements;

            // Iterate and modify over function tree
            while addr < current_tree.len() {
                addr += 1;
                let mut position = addr - 1;

                let opcode = {
                    // Should never fail
                    let element = current_tree.get(position).unwrap();

                    match element {
                        &OpcodeType(o) => o,
                        _ => continue,
                    }
                };

                if opcode.code != OP_BREAK {
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
            .map(|e| match e {
                &mut FunctionType(ref mut f) => Some(f),
                _ => None,
            })
            .filter(|e| e.is_some())
            .map(|f| f.unwrap())
            .collect();

        for function in functions {
            let mut addr = 0;
            let mut current_tree = &mut function.tree_elements;

            // Iterate and modify over function tree
            while addr < current_tree.len() {
                addr += 1;
                let mut position = addr - 1;

                let opcode = {
                    // Should never fail
                    let element = current_tree.get(position).unwrap();

                    match element {
                        &OpcodeType(o) => o,
                        _ => continue,
                    }
                };

                if opcode.code == OP_SYSREQ_C {
                    let sysreq_opcode = &opcode;
                    // Take previous PUSH.C to get args count
                    let native_arguments_count = {
                        let element = current_tree.get(position - 1).unwrap();

                        let opcode = match element {
                            &OpcodeType(o) => o,
                            _ => continue,
                        };

                        // Weird native call, ignore
                        if opcode.code != OP_PUSH_C {
                            trace!("Native call got no arguments definition");
                            continue;
                        }

                        opcode.param.unwrap() as usize / CELLSIZE
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

                    let is_having_non_opcodes = raw_args.iter().any(|e| match e {
                        &OpcodeType(_) => false,
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
                        .map(|e| match e {
                            &OpcodeType(o) => Some(o),
                            _ => None,
                        })
                        .filter(|e| e.is_some())
                        .map(|o| o.unwrap().clone())
                        .collect();

                    let is_having_non_push_c_opcodes =
                        args_opcodes.iter().any(|o| o.code != OP_PUSH_C);
                    if is_having_non_push_c_opcodes {
                        trace!("Invalid native call arguments");
                        continue;
                    }

                    let native_args: Vec<_> = args_opcodes
                        .iter()
                        .map(|o| o.param.unwrap())
                        .map(|addr| {
                            amx_plugin.read_constant_auto_type(addr as usize).unwrap()
                        })
                        .map(|cstr| Argument::String(cstr.into_string().unwrap()))
                        .collect();

                    let native_index = sysreq_opcode.param.unwrap() as usize;
                    let native_name = (&natives[native_index].name).clone().into_string().unwrap();

                    let ast_function_call = FunctionCall {
                        name: native_name,
                        args: Some(native_args),
                    };

                    current_tree[position] = FunctionCallType(ast_function_call);

                    // Check for trash OP_STACK
                    {
                        let opcode_position = position + 1;

                        let opcode = {
                            let element = match current_tree.get(opcode_position) {
                                Some(e) => e,
                                None => break,
                            };

                            match element {
                                &OpcodeType(o) => o,
                                _ => break,
                            }
                        };

                        if opcode.code == OP_STACK {
                            current_tree.remove(opcode_position);
                        }
                    }

                    // Check for trash OP_ZERO_PRI
                    {
                        let opcode_position = position + 1;

                        let opcode = {
                            let element = match current_tree.get(opcode_position) {
                                Some(e) => e,
                                None => break,
                            };

                            match element {
                                &OpcodeType(o) => o,
                                _ => break,
                            }
                        };

                        if opcode.code == OP_ZERO_PRI {
                            current_tree.remove(opcode_position);
                        }
                    }

                    // Check for trash OP_BREAK
                    {
                        let opcode_position = position + 1;

                        let opcode = {
                            let element = match current_tree.get(opcode_position) {
                                Some(e) => e,
                                None => break,
                            };

                            match element {
                                &OpcodeType(o) => o,
                                _ => break,
                            }
                        };

                        if opcode.code == OP_BREAK {
                            current_tree.remove(opcode_position);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
