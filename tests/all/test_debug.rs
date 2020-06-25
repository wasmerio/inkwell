use inkwell::context::Context;
use inkwell::debug_info::{DIFlags, DebugInfoBuilder, DwarfEmissionKind, DwarfSourceLanguage};

#[test]
fn test_debug_line_tables() {
    let context = Context::create();
    let module = context.create_module("");

    let dibuilder = module.create_debug_info_builder(true);
    let file = dibuilder.create_file("filename", "directory");
    let cu = dibuilder.create_compile_unit(
        DwarfSourceLanguage::Rust,
        file,
        "inkwell test",
        false,
        "",
        0,
        "",
        DwarfEmissionKind::LineTablesOnly,
        0,
        false,
        false,
    );
    let func_ty = dibuilder.create_subroutine_type(file, &[], DIFlags::Zero);
    let func = dibuilder.create_function(
        cu,
        "function",
        "_Z8functioni",
        file,
        3,
        func_ty,
        false,
        true,
        3,
        DIFlags::Public,
        false,
    );
    let debug_loc = dibuilder.create_debug_location(3, 1, func, None);

    let builder = context.create_builder();
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("_Z8functioni", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");

    assert_eq!(function.get_subprogram(), None);
    function.set_subprogram(func);
    assert_eq!(function.get_subprogram(), Some(func));

    builder.position_at_end(basic_block);
    builder.set_debug_location(debug_loc);
    builder.build_return(None);

    dibuilder.finish();

    module.print_to_stderr();
}
