#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_macros;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod archive;
mod buffers;
mod module;

use buffers::{ModuleBuffer, ThinBuffer};
use core::any::Any;
use module::Module;
use parking_lot::RwLock;
use rustc_ast::expand::allocator::AllocatorKind;
use rustc_codegen_ssa::back::lto::{LtoModuleCodegen, SerializedModule, ThinModule};
use rustc_codegen_ssa::back::write::{
    CodegenContext, FatLTOInput, ModuleConfig, TargetMachineFactoryFn,
};
use rustc_codegen_ssa::traits::{CodegenBackend, ExtraBackendMethods, WriteBackendMethods};
use rustc_codegen_ssa::{CodegenResults, CompiledModule, ModuleCodegen};
use rustc_errors::{FatalError, Handler, SubdiagnosticMessage};
use rustc_hash::FxHashMap;
use rustc_macros::fluent_messages;
use rustc_metadata::EncodedMetadata;
use rustc_middle::dep_graph::{WorkProduct, WorkProductId};
use rustc_middle::ty::query::Providers;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{OptLevel, OutputFilenames};
use rustc_session::Session;
use rustc_span::{ErrorGuaranteed, Symbol};
use std::sync::Arc;
use xlang::abi::traits::DynBox;
use xlang::host::rustcall;
use xlang::plugin::XLangCodegen;

#[derive(Default)]
pub struct XlangCodegenState {}

#[derive(Clone, Default)]
pub struct XlangCodegenBackend {
    state: Arc<RwLock<XlangCodegenState>>,
}

fluent_messages! { "../locale/en_US.ftl" }

type CodegenInit = rustcall!(extern "rustcall" fn() -> DynBox<dyn XLangCodegen>);

impl CodegenBackend for XlangCodegenBackend {
    fn locale_resource(&self) -> &'static str {
        crate::DEFAULT_LOCALE_RESOURCE
    }

    fn codegen_crate(
        &self,
        tcx: TyCtxt<'_>,
        metadata: EncodedMetadata,
        need_metadata_modules: bool,
    ) -> Box<dyn Any> {
        Box::new(rustc_codegen_ssa::base::codegen_crate(
            self.clone(),
            tcx,
            "x86_64-unknown-linux-gnu".into(),
            metadata,
            need_metadata_modules,
        ))
    }

    fn join_codegen(
        &self,
        _ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        _outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxHashMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        todo!()
    }

    fn link(
        &self,
        sess: &Session,
        codegen_results: CodegenResults,
        outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        rustc_codegen_ssa::back::link::link_binary(
            sess,
            &archive::XlangArchiveBuilderBuilder::default(),
            &codegen_results,
            outputs,
        )
    }

    fn provide(&self, providers: &mut Providers) {
        providers.global_backend_features = |_tcx, _| vec![];
    }
}

impl ExtraBackendMethods for XlangCodegenBackend {
    fn codegen_allocator<'tcx>(
        &self,
        _tcx: TyCtxt<'tcx>,
        _module_name: &str,
        _kind: AllocatorKind,
        _alloc_error_handler_kind: AllocatorKind,
    ) -> Module {
        let module = Module::new(&self.state);
        module
    }

    fn compile_codegen_unit(
        &self,
        _tcx: TyCtxt<'_>,
        _cgu_name: Symbol,
    ) -> (ModuleCodegen<Module>, u64) {
        todo!()
    }

    fn target_machine_factory(
        &self,
        _sess: &Session,
        _opt_level: OptLevel,
        _target_features: &[String],
    ) -> TargetMachineFactoryFn<Self> {
        Arc::new(|_| Ok(()))
    }
}

impl WriteBackendMethods for XlangCodegenBackend {
    type Module = Module;
    type TargetMachine = ();
    type TargetMachineError = ();
    type ModuleBuffer = ModuleBuffer;
    type ThinData = ();
    type ThinBuffer = ThinBuffer;

    fn run_link(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _modules: Vec<ModuleCodegen<Self::Module>>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }

    fn run_fat_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<FatLTOInput<Self>>,
        _cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<LtoModuleCodegen<Self>, FatalError> {
        todo!()
    }

    fn run_thin_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<(String, Self::ThinBuffer)>,
        _cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError> {
        todo!()
    }

    fn print_pass_timings(&self) {
        todo!();
    }

    unsafe fn optimize(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _module: &ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<(), FatalError> {
        todo!()
    }

    fn optimize_fat(
        _cgcx: &CodegenContext<Self>,
        _llmod: &mut ModuleCodegen<Self::Module>,
    ) -> Result<(), FatalError> {
        todo!()
    }

    unsafe fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        _thin: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }

    unsafe fn codegen(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _module: ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<CompiledModule, FatalError> {
        todo!()
    }

    fn prepare_thin(_module: ModuleCodegen<Self::Module>) -> (String, Self::ThinBuffer) {
        todo!()
    }

    fn serialize_module(_module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        todo!()
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::<XlangCodegenBackend>::default()
}
