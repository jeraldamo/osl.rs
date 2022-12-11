use super::*;
use super::ast::*;
use super::symtab::SymbolTable;

use crate::errors::*;

use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicMetadataTypeEnum;


pub fn compile(program: &Vec<Stmt>, _symbol_table: &SymbolTable) -> Result<Vec<u8>, OSLCompilerError> {

    let context = Context::create();
    let mut builder = context.create_builder();

    if let Some(module) = build_shader(&mut builder, &context, program) {
        println!("{}", module.to_string());
    }


    Ok(vec![])
}

fn build_shader<'ctx> (builder: &mut Builder, context: &'ctx Context, program: &Vec<Stmt>) -> Option<Module<'ctx>> {
    match &program[0].statement {
        Stmt_::ShaderDeclaration{name, shader_type: _, params, body: _} => {
            let shader_name = match &name.node {
                Expr_::Ident(s) => s,
                _ => {return None;}
            };

            let module = context.create_module(shader_name);
            // let mut types: HashMap<&str, Box<dyn AnyType>> = HashMap::new();
            
            let f32_type = context.f32_type();
            let v3f32_type = f32_type.vec_type(3);
            let i32_type = context.i32_type();
            let void_type = context.void_type();

            let mut param_types: Vec<BasicMetadataTypeEnum> = Vec::new();


            for param in params {
                match &param.node {
                    Expr_::Parameter{par_type, name, out: _, value: _} => {
                        let param_ident = match &name.node {
                            Expr_::Ident(n) => n,
                            _ => {return None;}
                        };

                        match &par_type.node {
                            Expr_::VariableType(Types::Float) => {
                                module.add_global(f32_type, Some(AddressSpace::Global), &param_ident.to_owned());
                                param_types.push(f32_type.ptr_type(AddressSpace::Global).into());
                            },
                            Expr_::VariableType(Types::Int) => {
                                module.add_global(i32_type, Some(AddressSpace::Global), &param_ident.to_owned());
                                param_types.push(i32_type.ptr_type(AddressSpace::Global).into());
                            },
                            // // Expr_::VariableType(Types::String) => ,
                            // Expr_::VariableType(Types::Void) => void_type,
                            Expr_::VariableType(Types::Color) => {
                                module.add_global(v3f32_type, Some(AddressSpace::Global), &param_ident.to_owned());
                                param_types.push(v3f32_type.ptr_type(AddressSpace::Global).into());
                            },
                            Expr_::VariableType(Types::Point) => {
                                module.add_global(v3f32_type, Some(AddressSpace::Global), &param_ident.to_owned());
                                param_types.push(v3f32_type.ptr_type(AddressSpace::Global).into());
                            },
                            _ => {return None;}
                        };
                    }
                    _ => {return None;}
                }
            }


            let entry_function_type = void_type.fn_type(&param_types, false);

            let function = module.add_function(shader_name, entry_function_type, None);

            let entry_block = context.append_basic_block(function, "entry");

            builder.build_return(None);

            return Some(module);


        },
        _ => {}
    }

    None

}
