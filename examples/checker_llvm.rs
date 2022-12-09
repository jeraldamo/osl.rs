use inkwell::AddressSpace;
use inkwell::context::Context;

fn main() {

    let context = Context::create();
    let builder = context.create_builder();
    
    let module = context.create_module("checker");
    let i32_type = context.i32_type();
    let f32_type = context.f32_type();
    let v3f32_type = f32_type.vec_type(3);
    
    let global = module.add_global(v3f32_type, Some(AddressSpace::Global), "pos");
    let global = module.add_global(f32_type, Some(AddressSpace::Global), "scale");

    let function_type = context.void_type().fn_type(&[
        v3f32_type.ptr_type(AddressSpace::Global).into(),
        f32_type.ptr_type(AddressSpace::Global).into()], false);
            

    let function = module.add_function("checker",
                                             function_type,
                                             None);

    let entry_block = context.append_basic_block(function, "entry");

    let pos_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
    let pos = builder.build_load(pos_ptr, "load").into_vector_value();
    let scale_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
    let scale = builder.build_load(scale_ptr, "load").into_float_value();

    let pos_x = builder.build_extract_element(pos, i32_type.const_int(0, false), "pos_x");
    let pos_y = builder.build_extract_element(pos, i32_type.const_int(1, false), "pos_y");
    let pos_z = builder.build_extract_element(pos, i32_type.const_int(2, false), "pos_z");

    let pos_x = builder.build_float_mul(pos_x.into_float_value(), scale, "pos_x");
    let pos_y = builder.build_float_mul(pos_y.into_float_value(), scale, "pos_y");
    let pos_z = builder.build_float_mul(pos_z.into_float_value(), scale, "pos_z");

    let pos = builder.build_insert_element(pos, pos_x, i32_type.const_int(0, false), "pos");
    let pos = builder.build_insert_element(pos, pos_y, i32_type.const_int(1, false), "pos");
    let pos = builder.build_insert_element(pos, pos_z, i32_type.const_int(2, false), "pos");

    builder.build_store(pos_ptr, pos);

    builder.build_return(None);



    



}
