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
    dibuilder.finish();
}
