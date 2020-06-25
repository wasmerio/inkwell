use crate::context::ContextRef;
use crate::values::MetadataValue;
use llvm_sys::debuginfo::{
    LLVMDIBuilderCreateBasicType, LLVMDIBuilderCreateCompileUnit, LLVMDIBuilderCreateDebugLocation,
    LLVMDIBuilderCreateFile, LLVMDIBuilderCreateFunction, LLVMDIBuilderCreateModule,
    LLVMDIBuilderCreateSubroutineType, LLVMDIBuilderFinalize, LLVMDIFlags, LLVMDWARFEmissionKind,
    LLVMDWARFSourceLanguage, LLVMDebugMetadataVersion, LLVMDisposeDIBuilder,
};
use llvm_sys::prelude::{LLVMDIBuilderRef, LLVMMetadataRef};

/// Gets the version of debug metadata produced by the current LLVM version.
pub fn debug_metadata_version() -> libc::c_uint {
    unsafe { LLVMDebugMetadataVersion() }
}

// TODO: these are meant to be or'd together.
#[llvm_enum(LLVMDIFlags)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DIFlags {
    #[llvm_variant(LLVMDIFlagZero)]
    Zero,
    #[llvm_variant(LLVMDIFlagPrivate)]
    Private,
    #[llvm_variant(LLVMDIFlagProtected)]
    Protected,
    #[llvm_variant(LLVMDIFlagPublic)]
    Public,
    #[llvm_variant(LLVMDIFlagFwdDecl)]
    FwdDecl,
    #[llvm_variant(LLVMDIFlagAppleBlock)]
    AppleBlock,
    #[llvm_variant(LLVMDIFlagReservedBit4)]
    ReservedBit4,
    #[llvm_variant(LLVMDIFlagVirtual)]
    Virtual,
    #[llvm_variant(LLVMDIFlagArtificial)]
    Artificial,
    #[llvm_variant(LLVMDIFlagExplicit)]
    Explicit,
    #[llvm_variant(LLVMDIFlagPrototyped)]
    Prototyped,
    #[llvm_variant(LLVMDIFlagObjcClassComplete)]
    ObjcClassComplete,
    #[llvm_variant(LLVMDIFlagObjectPointer)]
    ObjectPointer,
    #[llvm_variant(LLVMDIFlagVector)]
    Vector,
    #[llvm_variant(LLVMDIFlagStaticMember)]
    StaticMember,
    #[llvm_variant(LLVMDIFlagLValueReference)]
    LValueReference,
    #[llvm_variant(LLVMDIFlagRValueReference)]
    RValueReference,
    #[llvm_variant(LLVMDIFlagReserved)]
    Reserved,
    #[llvm_variant(LLVMDIFlagSingleInheritance)]
    SingleInheritance,
    #[llvm_variant(LLVMDIFlagMultipleInheritance)]
    MultipleInheritance,
    #[llvm_variant(LLVMDIFlagVirtualInheritance)]
    VirtualInheritance,
    #[llvm_variant(LLVMDIFlagIntroducedVirtual)]
    IntroducedVirtual,
    #[llvm_variant(LLVMDIFlagBitField)]
    BitField,
    #[llvm_variant(LLVMDIFlagNoReturn)]
    NoReturn,
    #[llvm_variant(LLVMDIFlagTypePassByValue)]
    TypePassByValue,
    #[llvm_variant(LLVMDIFlagTypePassByReference)]
    TypePassByReference,
    #[llvm_variant(LLVMDIFlagEnumClass)]
    EnumClass,
    #[llvm_variant(LLVMDIFlagThunk)]
    Thunk,
    #[llvm_variant(LLVMDIFlagNonTrivial)]
    NonTrivial,
    #[llvm_variant(LLVMDIFlagBigendian)]
    BigEndian,
    #[llvm_variant(LLVMDIFlagLittleEndian)]
    LittleEndian,
    #[llvm_variant(LLVMDIFlagIndirectVirtualBase)]
    IndirectVirtualBase,
}

#[llvm_enum(LLVMDWARFSourceLanguage)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Specifies what the source language is.
pub enum DwarfSourceLanguage {
    #[llvm_variant(LLVMDWARFSourceLanguageC89)]
    C89,
    #[llvm_variant(LLVMDWARFSourceLanguageC)]
    C,
    #[llvm_variant(LLVMDWARFSourceLanguageAda83)]
    Ada83,
    #[llvm_variant(LLVMDWARFSourceLanguageC_plus_plus)]
    CPlusPlus,
    #[llvm_variant(LLVMDWARFSourceLanguageCobol74)]
    Cobol74,
    #[llvm_variant(LLVMDWARFSourceLanguageCobol85)]
    Cobol85,
    #[llvm_variant(LLVMDWARFSourceLanguageFortran77)]
    Fortran77,
    #[llvm_variant(LLVMDWARFSourceLanguageFortran90)]
    Fortran90,
    #[llvm_variant(LLVMDWARFSourceLanguagePascal83)]
    Pascal83,
    #[llvm_variant(LLVMDWARFSourceLanguageModula2)]
    Modula2,
    #[llvm_variant(LLVMDWARFSourceLanguageJava)]
    Java,
    #[llvm_variant(LLVMDWARFSourceLanguageC99)]
    C99,
    #[llvm_variant(LLVMDWARFSourceLanguageAda95)]
    Ada95,
    #[llvm_variant(LLVMDWARFSourceLanguageFortran95)]
    Fortran95,
    #[llvm_variant(LLVMDWARFSourceLanguagePLI)]
    PLI,
    #[llvm_variant(LLVMDWARFSourceLanguageObjC)]
    ObjC,
    #[llvm_variant(LLVMDWARFSourceLanguageObjC_plus_plus)]
    ObjCPlusPlus,
    #[llvm_variant(LLVMDWARFSourceLanguageUPC)]
    UPC,
    #[llvm_variant(LLVMDWARFSourceLanguageD)]
    D,
    #[llvm_variant(LLVMDWARFSourceLanguagePython)]
    Python,
    #[llvm_variant(LLVMDWARFSourceLanguageOpenCL)]
    OpenCL,
    #[llvm_variant(LLVMDWARFSourceLanguageGo)]
    Go,
    #[llvm_variant(LLVMDWARFSourceLanguageModula3)]
    Modula3,
    #[llvm_variant(LLVMDWARFSourceLanguageHaskell)]
    Haskell,
    #[llvm_variant(LLVMDWARFSourceLanguageC_plus_plus_03)]
    CPlusPlus03,
    #[llvm_variant(LLVMDWARFSourceLanguageC_plus_plus_11)]
    CPlusPlus11,
    #[llvm_variant(LLVMDWARFSourceLanguageOCaml)]
    OCaml,
    #[llvm_variant(LLVMDWARFSourceLanguageRust)]
    Rust,
    #[llvm_variant(LLVMDWARFSourceLanguageC11)]
    C11,
    #[llvm_variant(LLVMDWARFSourceLanguageSwift)]
    Swift,
    #[llvm_variant(LLVMDWARFSourceLanguageJulia)]
    Julia,
    #[llvm_variant(LLVMDWARFSourceLanguageDylan)]
    Dylan,
    #[llvm_variant(LLVMDWARFSourceLanguageC_plus_plus_14)]
    CPlusPlus14,
    #[llvm_variant(LLVMDWARFSourceLanguageFortran03)]
    Fortran03,
    #[llvm_variant(LLVMDWARFSourceLanguageFortran08)]
    Fortran08,
    #[llvm_variant(LLVMDWARFSourceLanguageRenderScript)]
    RenderScript,
    #[llvm_variant(LLVMDWARFSourceLanguageBLISS)]
    BLISS,
    #[llvm_variant(LLVMDWARFSourceLanguageMips_Assembler)]
    MipsAssembler,
    #[llvm_variant(LLVMDWARFSourceLanguageGOOGLE_RenderScript)]
    GoogleRenderScript,
    #[llvm_variant(LLVMDWARFSourceLanguageBORLAND_Delphi)]
    BorlandDelphi,
}

#[llvm_enum(LLVMDWARFEmissionKind)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DwarfEmissionKind {
    #[llvm_variant(LLVMDWARFEmissionKindNone)]
    None,
    #[llvm_variant(LLVMDWARFEmissionKindFull)]
    Full,
    #[llvm_variant(LLVMDWARFEmissionKindLineTablesOnly)]
    LineTablesOnly,
}

#[derive(Debug)]
pub struct DebugInfoBuilder<'ctx> {
    builder: LLVMDIBuilderRef,
    //context: &'ctx Context,
    context: ContextRef<'ctx>,
}

impl<'ctx> DebugInfoBuilder<'ctx> {
    // The LLVM debug info C API is marked experimental and not stable.
    //
    // Safety:
    //   You must call create_compile_unit exactly once after creating this
    //   object before it is destroyed, or else you trigger UB in LLVM.
    pub(crate) fn new(builder: LLVMDIBuilderRef, context: ContextRef<'ctx>) -> Self {
        debug_assert!(!builder.is_null());

        DebugInfoBuilder { builder, context }
    }

    pub fn create_file(&self, filename: &str, directory: &str) -> MetadataValue<'ctx> {
        MetadataValue::new_with_metadata(self.context.get(), unsafe {
            LLVMDIBuilderCreateFile(
                self.builder,
                filename.as_ptr() as _,
                filename.len(),
                directory.as_ptr() as _,
                directory.len(),
            )
        })
    }

    pub fn create_compile_unit(
        &self,
        lang: DwarfSourceLanguage,
        file: MetadataValue<'ctx>,
        producer: &str,
        is_optimized: bool,
        flags: &str,
        runtime_version: u32,
        split_name: &str,
        kind: DwarfEmissionKind,
        dwo_id: u32,
        split_debug_inlining: bool,
        debug_info_for_profiling: bool,
    ) -> MetadataValue<'ctx> {
        MetadataValue::new_with_metadata(self.context.get(), unsafe {
            LLVMDIBuilderCreateCompileUnit(
                self.builder,
                lang.into(),
                file.as_metadata_ref(),
                producer.as_ptr() as _,
                producer.len(),
                is_optimized.into(),
                flags.as_ptr() as _,
                flags.len(),
                runtime_version,
                split_name.as_ptr() as _,
                split_name.len(),
                kind.into(),
                dwo_id,
                split_debug_inlining.into(),
                debug_info_for_profiling.into(),
            )
        })
    }

    pub fn create_module(
        &self,
        parent_scope: MetadataValue<'ctx>,
        name: &str,
        config_macros: &str,
        include_path: &str,
        isysroot: &str,
    ) -> MetadataValue<'ctx> {
        MetadataValue::new_with_metadata(self.context.get(), unsafe {
            LLVMDIBuilderCreateModule(
                self.builder,
                parent_scope.as_metadata_ref(),
                name.as_ptr() as _,
                name.len(),
                config_macros.as_ptr() as _,
                config_macros.len(),
                include_path.as_ptr() as _,
                include_path.len(),
                isysroot.as_ptr() as _,
                isysroot.len(),
            )
        })
    }

    /// Create debug info for a function.
    ///
    /// Safety:
    ///   If this DebugInfoBuilder was created with allow_unresolved=false then
    ///   you must set `is_definition` to true, or else you get UB in LLVM
    ///   during finalize (or an assertion in assertions-enabled builds of LLVM).
    pub fn create_function(
        &self,
        scope: MetadataValue<'ctx>,
        name: &str,
        linkage_name: &str,
        file: MetadataValue<'ctx>,
        line_no: u32,
        ty: MetadataValue<'ctx>,
        is_local_to_unit: bool,
        is_definition: bool,
        scope_line: u32,
        flags: DIFlags,
        is_optimized: bool,
    ) -> MetadataValue<'ctx> {
        MetadataValue::new_with_metadata(self.context.get(), unsafe {
            LLVMDIBuilderCreateFunction(
                self.builder,
                scope.as_metadata_ref(),
                name.as_ptr() as _,
                name.len(),
                linkage_name.as_ptr() as _,
                linkage_name.len(),
                file.as_metadata_ref(),
                line_no,
                ty.as_metadata_ref(),
                is_local_to_unit.into(),
                is_definition.into(),
                scope_line,
                flags.into(),
                is_optimized.into(),
            )
        })
    }

    // TODO lexical block, lexical block file, imported module from namespace,
    // imported module from alias, imported module from module, imported
    // declaration

    pub fn create_debug_location(
        &self,
        line: u32,
        column: u32,
        scope: MetadataValue<'ctx>,
        inlined_at: Option<MetadataValue<'ctx>>,
    ) -> MetadataValue<'ctx> {
        let inlined_at: LLVMMetadataRef = if let Some(inlined_at) = inlined_at {
            inlined_at.as_metadata_ref()
        } else {
            std::ptr::null_mut()
        };
        MetadataValue::new_with_metadata(self.context.get(), unsafe {
            LLVMDIBuilderCreateDebugLocation(
                self.context.get().context,
                line,
                column,
                scope.as_metadata_ref(),
                inlined_at,
            )
        })
    }

    pub fn create_subroutine_type(
        &self,
        file: MetadataValue<'ctx>,
        parameter_types: &[MetadataValue<'ctx>],
        flags: DIFlags,
    ) -> MetadataValue<'ctx> {
        let mut parameter_types = parameter_types
            .iter()
            .map(|t| t.as_metadata_ref())
            .collect::<Vec<_>>();
        MetadataValue::new_with_metadata(self.context.get(), unsafe {
            LLVMDIBuilderCreateSubroutineType(
                self.builder,
                file.as_metadata_ref(),
                parameter_types.as_mut_ptr(),
                parameter_types.len() as _,
                flags.into(),
            )
        })
    }

    /// Emit any deferred debug info.
    pub fn finish(&self) {
        unsafe { LLVMDIBuilderFinalize(self.builder) };
    }
}

impl<'ctx> Drop for DebugInfoBuilder<'ctx> {
    fn drop(&mut self) {
        self.finish();
        unsafe { LLVMDisposeDIBuilder(self.builder) };
    }
}

/*
#[derive(Debug)]
pub struct DebugLoc<'ctx>(LLVMMetadataRef);

impl DebugLoc<'ctx> {
    pub(crate) fn new(value: LLVMMetadataRef) -> Self {
        DebugLoc(value)
    }

    pub(crate) fn as_metadata(&self) -> MetadataValue<'ctx> {
        self.0
    }

    pub fn create(context: &'ctx Context, line: usize, column: usize, scope: MetadataValue, inlined_at: Option<MetadataValue>) -> Self {
        let inlined_at = if let Some(inlined_at) == inlined_at {
            inlined_at.as_metadata_ref()
        } else {
            std::ptr::null() as LLVMMetadataRef
        };
        Self::new(unsafe { LLVMDIBuilderCreateDebugLocation(context.context, line, column, scope.as_metadata_ref(), inlined_at) })
    }
}
     */
