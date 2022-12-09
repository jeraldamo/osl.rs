use rspirv::binary::Assemble;
use rspirv::binary::Disassemble;
use rspirv::dr::Builder;


fn main() {

    let mut b = Builder::new();
    //b.set_version(1, 0);
    //b.capability(spirv::Capability::Shader);
    //b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::Simple);
    let type_f32 = b.type_float(32);
    let type_i32 = b.type_int(32, 1);
    let type_bool = b.type_bool();
    let type_v3f32 = b.type_vector(type_f32, 3);
    let type_void = b.type_void();
    let type_voidf = b.type_function(type_void, vec![]);
    let fun = b
        .begin_function(
            type_void,
            None,
            // spirv::FunctionControl::DONT_INLINE | spirv::FunctionControl::CONST,
            spirv::FunctionControl::NONE,
            type_voidf,
        )
        .unwrap();


    let pointer_input_f32 = b.type_pointer(None, spirv::StorageClass::Input, type_f32);
    let pointer_input_v3f32 = b.type_pointer(None, spirv::StorageClass::Input, type_v3f32);
    let pointer_output_v3f32 = b.type_pointer(None,  spirv::StorageClass::Output, type_v3f32);

    let var_pos = b.variable(pointer_input_v3f32, None, spirv::StorageClass::Input, None);
    let var_scale = b.variable(pointer_input_f32, None, spirv::StorageClass::Input, None);
    let var_color1 = b.variable(pointer_input_v3f32, None, spirv::StorageClass::Input, None);
    let var_color2 = b.variable(pointer_input_v3f32, None, spirv::StorageClass::Input, None);
    let var_col = b.variable(pointer_output_v3f32, None, spirv::StorageClass::Output, None);


    let f32_2 = b.constant_f32(type_f32, 2.0);
    let i32_2 = b.constant_u32(type_i32, 2);

    b.begin_block(None).unwrap();

    let _1 = b.load(type_v3f32, None, var_pos, None, vec![]).unwrap();
    let _2 = b.load(type_f32, None, var_scale, None, vec![]).unwrap();
    let _3 = b.vector_times_scalar(type_v3f32, None, _1, _2).unwrap();


    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(spirv::ExecutionModel::Vertex, 
                  fun, 
                  "checker", 
                  vec![
                    var_pos,
                    var_scale,
                    var_color1,
                    var_color2,
                    var_col,
                  ]);
    let module = b.module();

    // Assembling
    let code = module.assemble();
    assert!(code.len() > 20); // Module header contains 5 words
    assert_eq!(spirv::MAGIC_NUMBER, code[0]);

    // Parsing
    let mut loader = rspirv::dr::Loader::new();
    rspirv::binary::parse_words(&code, &mut loader).unwrap();
    let module = loader.module();

    println!("{}", module.disassemble());
}
