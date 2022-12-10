// // use inkwell::{
// //     context::Context,
// //     builder::Builder,
// //     passes::PassManager,
// //     module::Module,
// //     values::{FunctionValue, PointerValue}};
//
//
// use crate::compiler::symtab::SymbolTable;
// use crate::errors::*;
//
// pub struct Compiler<'a> {//, 'ctx> {
//     //pub context: &'ctx Context,
//     //pub builder: &'a Builder<'ctx>,
//     //pub fpm: &'a PassManager<FunctionValue<'ctx>>,
//     //pub module: &'a Module<'ctx>,
//     pub tokens: Vec<(Token, Span)>,
//     pub program: &'a Vec<Stmt>,
//
//     symbol_table: SymbolTable,
// }
//
// impl<'a> Compiler<'a> {
//     pub fn new(//context: &'ctx Context,
//         //builder: &'a Builder<'ctx>,
//         //module: &'a Module<'ctx>,
//         tokens: Vec<(Token, Span)>,
//         program: &'a Vec<Stmt>,
//         program_size: usize) -> Result<Self, OSLCompilerError> {
//
//         Ok(Self {
//             //context,
//             //builder,
//             //module,
//             tokens,
//             program,
//             symbol_table: SymbolTable::new(program_size)?,
//         })
//     }
//
//     pub fn compile(&mut self) -> Result<Vec<u8>, OSLCompilerError> {
//
//         self.build_symbols(self.program)?;
//         // println!("{:#?}", self.symbol_table);
//
//         println!("Checking Semantics...");
//         self.check_semantics()?;
//
//         println!("Building SPIR-V...");
//         
//
//         println!("Done Compiling");
//
//         // println!("{}", self.symbol_table);
//
//         Ok(vec![])
//     }
//
//
//
//
// }
