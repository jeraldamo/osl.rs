use crate::compiler::*;
use crate::errors::*;
use crate::stdosl;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Symbols {
    Variable {
        var_type: Types,
        name: String,
        span: Span,
        scope: u64,
        output: bool,
    },
    Function {
        ret_type: Types,
        name: String,
        arg_types: Vec<Types>,
        span: Span,
        scope: u64,
        public: bool,
    },
    Shader {
        shader_type: ShaderTypes,
        name: String,
        span: Span,
        scope: u64,
    },
    Closure,
}

impl Symbols {
    pub fn get_symbol_type(&self) -> String {
        match self {
            Symbols::Variable {..} => String::from("Variable"),
            Symbols::Function {..} => String::from("Function"),
            Symbols::Shader {..} => String::from("Shader"),
            _ => String::new(),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            Symbols::Variable {var_type, ..} => format!("{:?}", var_type.clone()),
            Symbols::Function {ret_type, ..} => format!("{:?}", ret_type.clone()),
            Symbols::Shader {shader_type, ..} => format!("{:?}", shader_type.clone()),
            _ => String::new(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Symbols::Variable {name, ..} => name.clone(),
            Symbols::Function {name, ..} => name.clone(),
            _ => String::new(),
        }
    }

    pub fn get_span(&self) -> Span {
        match self {
            Symbols::Variable {span, ..} => span.clone(),
            Symbols::Function {span, ..} => span.clone(),
            _ => Span{lo: 0, hi: 0, line: 0},
        }
    }

    pub fn get_scope(&self) -> u64 {
        match self {
            Symbols::Variable {scope, ..} => *scope,
            Symbols::Function {scope, ..} => *scope,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Vec<Symbols>>,
    pub cur_scope: u64,
    next_scope: u64,
    scope_stack: Vec<u64>,
    scopes: Vec<u64>,

    pub n_variables: usize,
    pub n_functions: usize,
    pub n_shaders: usize,
}

impl SymbolTable {
    pub fn new(program_size: usize) -> Result<Self, OSLCompilerError> {
        let mut symbol_table = SymbolTable {
            symbols: HashMap::new(),
            cur_scope: 1,
            next_scope: 2,
            scope_stack: Vec::new(),
            scopes: vec![1;program_size],
            n_variables: 0,
            n_functions: 0,
            n_shaders: 0,
        };

        stdosl::populate_stdosl_symbols(&mut symbol_table)?;

        Ok(symbol_table)

    }

    pub fn add_variable(&mut self, var_type: Types, name: String, span: Span, output: bool) -> Result<(), OSLCompilerError> {
        if self.cur_scope == 1 {
            return Err(OSLCompilerError::GlobalScopeVariable{
                var : Item::new(span, name)
            });
        }
        let var = Symbols::Variable {
            var_type,
            name: name.clone(),
            span,
            scope: self.cur_scope,
            output,
        };

        if self.symbols.contains_key(name.as_str()) {

            for symbol in self.symbols.get(name.as_str()).unwrap() {

                if symbol.get_scope() == self.cur_scope {
                    // Duplicate symbol error
                    return Err(OSLCompilerError::ExistingVariable {
                        existing: Item::new(symbol.get_span(), symbol.get_name()),
                        new: Item::new(span, name),
                    });
                }
            }

            self.symbols.get_mut(name.as_str())
                .unwrap()
                .push(var);
        }

        else {
            self.symbols.insert(name.clone(), vec![var]);
        }

        self.n_variables += 1;

        Ok(())
    }

    pub fn add_function(&mut self, ret_type: Types, name: String, arg_types: Vec<Types>, span: Span, public: bool) -> Result<(), OSLCompilerError> {
        let func = Symbols::Function {
            ret_type,
            name: name.clone(),
            arg_types,
            span,
            scope: self.cur_scope,
            public,
        };

        if self.symbols.contains_key(name.as_str()) {
            for symbol in self.symbols.get(name.as_str()).unwrap() {
                if symbol.get_scope() == self.cur_scope {
                    // Duplicate symbol error
                    return Err(OSLCompilerError::ExistingVariable {
                        existing: Item::new(symbol.get_span(), symbol.get_name()),
                        new: Item::new(span, name),
                    });
                }
            }

            self.symbols.get_mut(name.as_str())
                .unwrap()
                .push(func);
        }

        else {
            self.symbols.insert(name.clone(), vec![func]);
        }

        self.n_functions += 1;


        Ok(())
    }

    pub fn add_shader(&mut self, shader_type: ShaderTypes, name: String, span: Span) -> Result<(), OSLCompilerError> {
        let shader = Symbols::Shader {
            shader_type,
            name: name.clone(),
            span,
            scope: self.cur_scope,
        };

        if self.symbols.contains_key(name.as_str()) {
            if let Some(s) = self.symbols.get(name.as_str()) {
                for symbol in s {
                    if symbol.get_scope() == self.cur_scope {
                        // Duplicate symbol error
                        return Err(OSLCompilerError::ExistingVariable {
                            existing: Item::new(symbol.get_span(), symbol.get_name()),
                            new: Item::new(span, name),
                        });
                    }
                }
            }

            self.symbols.get_mut(name.as_str())
                .unwrap()
                .push(shader);
        }

        else {
            self.symbols.insert(name.clone(), vec![shader]);
        }

        self.n_shaders += 1;

        Ok(())
    }

    pub fn up_scope(&mut self, span: Span) {


        self.scope_stack.push(self.cur_scope);
        self.cur_scope |= self.next_scope;
        self.next_scope <<= 1;

        for i in span.lo..span.hi {
            self.scopes[i] = self.cur_scope;
        }
    }

    pub fn down_scope(&mut self) {
        self.cur_scope = self.scope_stack.pop().unwrap();
    }

    pub fn get_scope(&self, loc: usize) -> u64 {
        self.scopes[loc]
    }

    pub fn check_access(&self, origin_span: Span, dest_ident: String) -> Result<(), OSLCompilerError>{

        let mut items: Vec<Item> = Vec::new();
        let scope = self.scopes[origin_span.lo];

        if let Some(s) = self.symbols.get(dest_ident.as_str()) {
            for symbol in s {
                if scope | symbol.get_scope() == scope {
                    return Ok(());
                }

                items.push(Item::new(symbol.get_span(), ""));
            }
        }

        Err(OSLCompilerError::OutOfScopeIdent {
            origin: Item::new(origin_span, ""),
            options: items,
        })
    }

    pub fn get_reference(&self, span: Span, dest_ident: String) -> Symbols {
        let scope = self.scopes[span.lo];
        let mut closest: Symbols = self.symbols.get(dest_ident.as_str()).unwrap()[0].clone();
        let mut closest_distance: isize = isize::MAX;

        for symbol in self.symbols.get(dest_ident.as_str()).unwrap() {
            let distance = self.distance(scope, symbol.get_scope());
            if distance < closest_distance {
                closest = symbol.clone();
                closest_distance = distance;
            }
        }

        closest
    }

    fn distance(&self, scope1: u64, scope2: u64) -> isize {
        let mut mask: u64 = 1;
        let mut count1 = 0;
        let mut count2 = 0;

        for _ in 0..63 {
            if scope1 & mask > 0 {count1 += 1;}
            if scope2 & mask > 0 {count2 += 1;}
            mask <<= 1;
        }

        count1 - count2
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("Symbol Table:\n");

        for (key, value) in self.symbols.iter() {
            s = format!("{}\t{}\n", s, key);
            for sym in value {
                s = format!("{}\t\t{:016b}({})\t{}:{:?}:{}\n", s, sym.get_scope(), sym.get_scope(), sym.get_symbol_type(), sym.get_type(), sym.get_span().line);
            }
        }
        write!(f, "{}", s)
    }
}
