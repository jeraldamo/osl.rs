use crate::compiler::symtab::SymbolTable;
use crate::compiler::{Types, Span};
use crate::errors::OSLCompilerError;






pub fn populate_stdosl_symbols(symbol_table: &mut SymbolTable) -> Result<(), OSLCompilerError>{
    let default_span = Span {lo: 0, hi: 0, line: 0};

    symbol_table.add_function(Types::Color, String::from("color"), Vec::new(), default_span, true)?;
    symbol_table.add_function(Types::Float, String::from("mod"), vec![Types::Float, Types::Float], default_span, true)?;

    Ok(())
}

// pub fn generate_stdosl(context: &Context, builder: &Builder, module: &Module) -> Option<()> {
//
//     build_radians_float(context, builder, module);
//
//     Some(())
// }
//
// pub fn build_radians_float(context: &Context, builder: &Builder, module: &Module) -> Option<()> {
//     let f64_type = context.f64_type();
//     let function = module.add_function("_stdosl_radians_float",
//     f64_type.fn_type(&[f64_type.into()], false),
//     None);
//     let basic_block_entry = context.append_basic_block(function, "entry");
//
//     builder.position_at_end(basic_block_entry);
//
//     let degrees = function.get_nth_param(0)?.into_float_value();
//
//     let radians = builder.build_float_mul(degrees,
//     f64_type.const_float(0.0174532925199432957692369076848861271344287188854172545609719144),
//     "deg");
//
//     builder.build_return(Some(&radians));
//
//     Some(())
// }
