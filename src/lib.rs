#![feature(lazy_cell, rustc_private)]

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
use xlang::{ir, plugin::XLangPlugin, targets::Target};
use core::any::Any;
use module::Module;
use rustc_ast::expand::allocator::AllocatorKind;
use rustc_codegen_ssa::back::lto::{LtoModuleCodegen, SerializedModule, ThinModule};
use rustc_codegen_ssa::back::write::{
    CodegenContext, EmitObj, FatLTOInput, ModuleConfig, OngoingCodegen, TargetMachineFactoryFn,
};
use rustc_codegen_ssa::traits::{CodegenBackend, ExtraBackendMethods, WriteBackendMethods};
use rustc_codegen_ssa::{CodegenResults, CompiledModule, ModuleCodegen, ModuleKind};
use rustc_errors::{FatalError, Handler, SubdiagnosticMessage};
use rustc_hash::FxHashMap;
use rustc_macros::fluent_messages;
use rustc_metadata::EncodedMetadata;
use rustc_middle::dep_graph::{WorkProduct, WorkProductId};
use rustc_middle::ty::query::Providers;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::{OptLevel, OutputFilenames, OutputType};
use rustc_session::Session;
use rustc_span::{ErrorGuaranteed, Symbol};
use std::{sync::Arc, fs::File};
use xlang::abi::collection::HashMap as AbiHashMap;

#[derive(Clone, Default)]
pub struct XLangCodegenBackend {}

fluent_messages! { "../locale/en_US.ftl" }

impl CodegenBackend for XLangCodegenBackend {
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
        ongoing_codegen: Box<dyn Any>,
        sess: &Session,
        _outputs: &OutputFilenames,
    ) -> Result<(CodegenResults, FxHashMap<WorkProductId, WorkProduct>), ErrorGuaranteed> {
        let (codegen_results, work_products) = ongoing_codegen
            .downcast::<OngoingCodegen<XLangCodegenBackend>>()
            .expect("Expected XlangCodegenBackend's OngoingCodegen, found Box<Any>")
            .join(sess);
        Ok((codegen_results, work_products))
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

impl ExtraBackendMethods for XLangCodegenBackend {
    fn codegen_allocator<'tcx>(
        &self,
        _tcx: TyCtxt<'tcx>,
        _module_name: &str,
        _kind: AllocatorKind,
        _alloc_error_handler_kind: AllocatorKind,
    ) -> Module {
        let module = Module::new();
        module
    }

    fn compile_codegen_unit(
        &self,
        tcx: TyCtxt<'_>,
        cgu_name: Symbol,
    ) -> (ModuleCodegen<Module>, u64) {
        let dep_node = tcx.codegen_unit(cgu_name).codegen_dep_node(tcx);
        let (module, _) = tcx.dep_graph.with_task(
            dep_node,
            tcx,
            cgu_name,
            module_codegen,
            Some(rustc_middle::dep_graph::hash_result),
        );
        fn module_codegen(tcx: TyCtxt<'_>, cgu_name: Symbol) -> ModuleCodegen<Module> {
            let _cgu = tcx.codegen_unit(cgu_name);
            let module = Module::new();

            ModuleCodegen {
                name: cgu_name.to_string(),
                module_llvm: module, // Not LLVM
                kind: ModuleKind::Regular,
            }
        }
        (module, 0)
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

impl WriteBackendMethods for XLangCodegenBackend {
    type Module = Module;
    type TargetMachine = ();
    type TargetMachineError = ();
    type ModuleBuffer = ModuleBuffer;
    type ThinData = ();
    type ThinBuffer = ThinBuffer;

    fn run_link(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _modules: Vec<ModuleCodegen<Module>>,
    ) -> Result<ModuleCodegen<Module>, FatalError> {
        todo!()
    }

    fn run_fat_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<FatLTOInput<Self>>,
        _cached_modules: Vec<(SerializedModule<ModuleBuffer>, WorkProduct)>,
    ) -> Result<LtoModuleCodegen<Self>, FatalError> {
        todo!()
    }

    fn run_thin_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<(String, ThinBuffer)>,
        _cached_modules: Vec<(SerializedModule<ModuleBuffer>, WorkProduct)>,
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError> {
        todo!()
    }

    fn print_pass_timings(&self) {
        todo!();
    }

    unsafe fn optimize(
        _cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        _module: &ModuleCodegen<Module>,
        _config: &ModuleConfig,
    ) -> Result<(), FatalError> {
        Ok(())
    }

    fn optimize_fat(
        _cgcx: &CodegenContext<Self>,
        _llmod: &mut ModuleCodegen<Module>,
    ) -> Result<(), FatalError> {
        todo!()
    }

    unsafe fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        _thin: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Module>, FatalError> {
        todo!()
    }

    unsafe fn codegen(
        cgcx: &CodegenContext<Self>,
        _diag_handler: &Handler,
        mut module: ModuleCodegen<Module>,
        config: &ModuleConfig,
    ) -> Result<CompiledModule, FatalError> {
        let module_name = module.name.clone();
        let module_name = Some(&module_name[..]);
        let obj_out = cgcx.output_filenames.temp_path(OutputType::Object, module_name);
        let xlang_module = &mut module.module_llvm;
        let root = ir::Scope {
            annotations: ir::AnnotatedElement::default(),
            members: AbiHashMap::default(),
        };
        let target: Target = target_tuples::Target::parse("x86_64-pc-linux-gnu").into();
        let mut file = ir::File {
            target: target.clone(),
            root,
        };
        match config.emit_obj {
            EmitObj::ObjectCode(_) => {
                xlang_module.codegen.set_target(target);
                xlang_module.codegen.accept_ir(&mut file);
                xlang_module.write(File::create(obj_out).unwrap());
            }
            EmitObj::Bitcode => todo!(),
            EmitObj::None => {}
        }

        Ok(module.into_compiled_module(
            config.emit_obj != EmitObj::None,
            false,
            config.emit_bc,
            &cgcx.output_filenames,
        ))
    }

    fn prepare_thin(_module: ModuleCodegen<Module>) -> (String, ThinBuffer) {
        todo!()
    }

    fn serialize_module(_module: ModuleCodegen<Module>) -> (String, ModuleBuffer) {
        todo!()
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::<XLangCodegenBackend>::default()
}
