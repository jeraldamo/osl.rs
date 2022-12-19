use super::*;
use super::ast::*;
use super::symtab::SymbolTable;

use crate::errors::*;

use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::BasicValueEnum;

pub fn compile(program: &Vec<Stmt>, symbol_table: &SymbolTable) -> Result<Vec<u8>, OSLCompilerError> {

    let context = Context::create();
    let builder = context.create_builder();

    if let Some(module) = build_shader(&context, &builder, &symbol_table, program) {
        println!("{}", module.to_string());
    }


    Ok(vec![])
}

fn build_shader<'ctx> (context: &'ctx Context, builder: &Builder, symbol_table: &SymbolTable, program: &Vec<Stmt>) -> Option<Module<'ctx>> {
    match &program[0].statement {
        Stmt_::ShaderDeclaration{name, shader_type: _, params, body} => {
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
                            Expr_::VariableType(Types::Color) |
                            Expr_::VariableType(Types::Normal) |
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

            let _entry_block = context.append_basic_block(function, "entry");

            if let Stmt_::BlockStatement(statements) = &body.statement {
                for statement in statements.clone().into_iter() {
                    match statement.statement {
                        Stmt_::ExpressionStatement(s) => {
                            build_expression(&context, &builder, &module, &symbol_table, &s.node);
                        }
                        _ => {}
                    }
                }
            }

            builder.build_return(None);

            return Some(module);


        },
        _ => {}
    }

    None

}

fn build_expression<'ctx> (context: &'ctx Context,
                           builder: &'ctx Builder,
                           module: &'ctx Module,
                           symbol_table: &'ctx SymbolTable, 
                           expr: &'ctx Expr_
                           ) -> Option<BasicValueEnum<'ctx>> {
    return match expr {
        Expr_::Ident(s) => {
            println!("Fetching {}", s);
            let global_val = module.get_global(s)?;
            println!("{:?}", builder.build_load(global_val.as_pointer_value(), "In"));
            let val = builder.build_load(global_val.as_pointer_value(), "");
            println!("Done");
            Some(val)

        },
        Expr_::FloatLiteral(f) => {
            Some(context.f32_type().const_float(*f).into())
        },
        Expr_::IntLiteral(i) => {
            Some(context.i32_type().const_int(*i as u64, true).into())
        },
        Expr_::BinaryExpression(op, lhs, rhs) => {
            match op {
                Operators::Multiply => {
                    match (get_expr_type(&lhs, symbol_table).expect("Placeholder"),
                           get_expr_type(&rhs, symbol_table).expect("Placeholder")) {
                        (Types::Float, Types::Float) => {
                            let lhs_value = build_expression(context, builder, module, symbol_table, &lhs.node)?.into_float_value();
                            let rhs_value = build_expression(context, builder, module, symbol_table, &rhs.node)?.into_float_value();
                            Some(builder.build_float_mul(
                                    lhs_value,
                                    rhs_value,
                                    "").into())
                        },
                        (Types::Color, Types::Float) |
                        (Types::Normal, Types::Float) => {
                            let lhs_value = build_expression(context, builder, module, symbol_table, &lhs.node)?.into_vector_value();
                            let rhs_value = build_expression(context, builder, module, symbol_table, &rhs.node)?.into_float_value();

                            let i32_type = context.i32_type();

                            let x = builder.build_extract_element(lhs_value, i32_type.const_int(0, false), "");
                            let y = builder.build_extract_element(lhs_value, i32_type.const_int(1, false), "");
                            let z = builder.build_extract_element(lhs_value, i32_type.const_int(2, false), "");

                            let x = builder.build_float_mul(x.into_float_value(), rhs_value, "");
                            let y = builder.build_float_mul(y.into_float_value(), rhs_value, "");
                            let z = builder.build_float_mul(z.into_float_value(), rhs_value, "");

                            let lhs_value = builder.build_insert_element(lhs_value, x, i32_type.const_int(0, false), "");
                            let lhs_value = builder.build_insert_element(lhs_value, y, i32_type.const_int(1, false), "");
                            let lhs_value = builder.build_insert_element(lhs_value, z, i32_type.const_int(2, false), "");
                            
                            Some(lhs_value.into())
                        }

                        _ => {None}

                    }
                }
                
                _ => {None}
            }
        }
        Expr_::Assignment(lhs, rhs) => {
            let val = build_expression(context, builder, module, symbol_table, &rhs.node)?;
            if let Expr_::Ident(name) = &lhs.node {
               let global_val = module.get_global(name)?;
               builder.build_store(global_val.as_pointer_value(), val);
            }

            None
        },

        _ => {None},
    }
}
