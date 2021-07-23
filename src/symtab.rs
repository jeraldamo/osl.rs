use crate::*;
use crate::errors::*;

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
    pub fn get_type(&self) -> Types {
        match self {
            Symbols::Variable {var_type, ..} => var_type.clone(),
            Symbols::Function {ret_type, ..} => ret_type.clone(),
            _ => Types::Void,
        }
    }

    pub fn get_shader_type(&self) -> ShaderTypes {
        match self {
            Symbols::Shader {shader_type, ..} => shader_type.clone(),
            _ => ShaderTypes::Shader,
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
}

impl SymbolTable {
    pub fn new(program_size: usize) -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            cur_scope: 1,
            next_scope: 2,
            scope_stack: Vec::new(),
            scopes: vec![1;program_size],
        }
    }

    pub fn add_variable(&mut self, var_type: Types, name: String, span: Span, output: bool) -> Result<(), OSLCompilerError> {
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

        Ok(())
    }

    pub fn add_function(&mut self, ret_type: Types, name: String, span: Span, public: bool) -> Result<(), OSLCompilerError> {
        let func = Symbols::Function {
            ret_type,
            name: name.clone(),
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

    pub fn check_access(&self, origin: String, dest: String) {
        let mut accessible: Vec<Symbols> = Vec::new();
        let mut inaccessible: Vec<Symbols> = Vec::new();

        for symbol in self.symbols.get(dest.as_str()) {
            ;
        }
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