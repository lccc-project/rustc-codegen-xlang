use rustc_abi::{HasDataLayout, TargetDataLayout};
use rustc_ast::Param;
use rustc_codegen_ssa::traits::{PreDefineMethods, BackendTypes, AsmMethods, DebugInfoMethods, CoverageInfoMethods, StaticMethods, ConstMethods, MiscMethods, TypeMembershipMethods, LayoutTypeMethods, BaseTypeMethods};
use rustc_hir::def_id::DefId;
use rustc_middle::{ty::{TyCtxt, layout::{FnAbiOfHelpers, LayoutOfHelpers, HasParamEnv, HasTyCtxt, TyAndLayout}, ParamEnv, Ty, Instance, self, TypeVisitableExt}, mir::mono::{Linkage, Visibility}};
use rustc_target::{spec::{HasTargetSpec, Target}, abi::call::FnAbi};
use xlang::ir;

use crate::value::Value;

pub struct CodegenCx<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> CodegenCx<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx
        }
    }
}

impl<'tcx> AsmMethods<'tcx> for CodegenCx<'tcx> {
    fn codegen_global_asm(
        &self,
        template: &[rustc_ast::InlineAsmTemplatePiece],
        operands: &[rustc_codegen_ssa::traits::GlobalAsmOperandRef<'tcx>],
        options: rustc_ast::InlineAsmOptions,
        line_spans: &[rustc_span::Span],
    ) {
        todo!()
    }
}

impl<'tcx> BackendTypes for CodegenCx<'tcx> {
    type Value = Value;
    type Function = ();
    type BasicBlock = ();
    type Type = ();
    type Funclet = ();

    type DIScope = ();
    type DILocation = ();
    type DIVariable = ();
}

impl<'tcx> BaseTypeMethods<'tcx> for CodegenCx<'tcx> {
    fn type_i1(&self) -> Self::Type {
        todo!()
    }

    fn type_i8(&self) -> Self::Type {
        todo!()
    }

    fn type_i16(&self) -> Self::Type {
        todo!()
    }

    fn type_i32(&self) -> Self::Type {
        todo!()
    }

    fn type_i64(&self) -> Self::Type {
        todo!()
    }

    fn type_i128(&self) -> Self::Type {
        todo!()
    }

    fn type_isize(&self) -> Self::Type {
        todo!()
    }

    fn type_f32(&self) -> Self::Type {
        todo!()
    }

    fn type_f64(&self) -> Self::Type {
        todo!()
    }

    fn type_array(&self, ty: Self::Type, len: u64) -> Self::Type {
        todo!()
    }

    fn type_func(&self, args: &[Self::Type], ret: Self::Type) -> Self::Type {
        todo!()
    }

    fn type_struct(&self, els: &[Self::Type], packed: bool) -> Self::Type {
        todo!()
    }

    fn type_kind(&self, ty: Self::Type) -> rustc_codegen_ssa::common::TypeKind {
        todo!()
    }

    fn type_ptr_to(&self, ty: Self::Type) -> Self::Type {
        todo!()
    }

    fn type_ptr_to_ext(&self, ty: Self::Type, address_space: rustc_abi::AddressSpace) -> Self::Type {
        todo!()
    }

    fn element_type(&self, ty: Self::Type) -> Self::Type {
        todo!()
    }

    fn vector_length(&self, ty: Self::Type) -> usize {
        todo!()
    }

    fn float_width(&self, ty: Self::Type) -> usize {
        todo!()
    }

    fn int_width(&self, ty: Self::Type) -> u64 {
        todo!()
    }

    fn val_ty(&self, v: Self::Value) -> Self::Type {
        todo!()
    }
}

impl<'tcx> ConstMethods<'tcx> for CodegenCx<'tcx> {
    fn const_null(&self, t: Self::Type) -> Self::Value {
        todo!()
    }

    fn const_undef(&self, t: Self::Type) -> Self::Value {
        todo!()
    }

    fn const_poison(&self, t: Self::Type) -> Self::Value {
        todo!()
    }

    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value {
        todo!()
    }

    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value {
        todo!()
    }

    fn const_uint_big(&self, t: Self::Type, u: u128) -> Self::Value {
        todo!()
    }

    fn const_bool(&self, val: bool) -> Self::Value {
        todo!()
    }

    fn const_i16(&self, i: i16) -> Self::Value {
        todo!()
    }

    fn const_i32(&self, i: i32) -> Self::Value {
        todo!()
    }

    fn const_u32(&self, i: u32) -> Self::Value {
        todo!()
    }

    fn const_u64(&self, i: u64) -> Self::Value {
        todo!()
    }

    fn const_usize(&self, i: u64) -> Self::Value {
        todo!()
    }

    fn const_u8(&self, i: u8) -> Self::Value {
        todo!()
    }

    fn const_real(&self, t: Self::Type, val: f64) -> Self::Value {
        todo!()
    }

    fn const_str(&self, s: &str) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn const_struct(&self, elts: &[Self::Value], packed: bool) -> Self::Value {
        todo!()
    }

    fn const_to_opt_uint(&self, v: Self::Value) -> Option<u64> {
        todo!()
    }

    fn const_to_opt_u128(&self, v: Self::Value, sign_ext: bool) -> Option<u128> {
        todo!()
    }

    fn const_data_from_alloc(&self, alloc: rustc_const_eval::interpret::ConstAllocation<'tcx>) -> Self::Value {
        todo!()
    }

    fn scalar_to_backend(&self, cv: rustc_const_eval::interpret::Scalar, layout: rustc_target::abi::Scalar, llty: Self::Type) -> Self::Value {
        todo!()
    }

    fn from_const_alloc(
        &self,
        layout: rustc_middle::ty::layout::TyAndLayout<'tcx>,
        alloc: rustc_const_eval::interpret::ConstAllocation<'tcx>,
        offset: rustc_abi::Size,
    ) -> rustc_codegen_ssa::mir::place::PlaceRef<'tcx, Self::Value> {
        todo!()
    }

    fn const_ptrcast(&self, val: Self::Value, ty: Self::Type) -> Self::Value {
        todo!()
    }
}

impl<'tcx> CoverageInfoMethods<'tcx> for CodegenCx<'tcx> {
    fn coverageinfo_finalize(&self) {
        todo!()
    }

    fn define_unused_fn(&self, def_id: rustc_hir::def_id::DefId) {
        todo!()
    }

    fn get_pgo_func_name_var(&self, instance: rustc_middle::ty::Instance<'tcx>) -> Self::Value {
        todo!()
    }
}

impl<'tcx> DebugInfoMethods<'tcx> for CodegenCx<'tcx> {
    fn create_vtable_debuginfo(
        &self,
        ty: rustc_middle::ty::Ty<'tcx>,
        trait_ref: Option<rustc_middle::ty::PolyExistentialTraitRef<'tcx>>,
        vtable: Self::Value,
    ) {
        todo!()
    }

    fn create_function_debug_context(
        &self,
        instance: rustc_middle::ty::Instance<'tcx>,
        fn_abi: &rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
        llfn: Self::Function,
        mir: &rustc_middle::mir::Body<'tcx>,
    ) -> Option<rustc_codegen_ssa::mir::debuginfo::FunctionDebugContext<Self::DIScope, Self::DILocation>> {
        todo!()
    }

    fn dbg_scope_fn(
        &self,
        instance: rustc_middle::ty::Instance<'tcx>,
        fn_abi: &rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
        maybe_definition_llfn: Option<Self::Function>,
    ) -> Self::DIScope {
        todo!()
    }

    fn dbg_loc(
        &self,
        scope: Self::DIScope,
        inlined_at: Option<Self::DILocation>,
        span: rustc_span::Span,
    ) -> Self::DILocation {
        todo!()
    }

    fn extend_scope_to_file(
        &self,
        scope_metadata: Self::DIScope,
        file: &rustc_span::SourceFile,
    ) -> Self::DIScope {
        todo!()
    }

    fn debuginfo_finalize(&self) {
        todo!()
    }

    fn create_dbg_var(
        &self,
        variable_name: rustc_span::Symbol,
        variable_type: rustc_middle::ty::Ty<'tcx>,
        scope_metadata: Self::DIScope,
        variable_kind: rustc_codegen_ssa::mir::debuginfo::VariableKind,
        span: rustc_span::Span,
    ) -> Self::DIVariable {
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for CodegenCx<'tcx> {
    type FnAbiOfResult = &'tcx FnAbi<'tcx, Ty<'tcx>>;

    fn handle_fn_abi_err(
        &self,
        err: rustc_middle::ty::layout::FnAbiError<'tcx>,
        span: rustc_span::Span,
        fn_abi_request: rustc_middle::ty::layout::FnAbiRequest<'tcx>,
    ) -> ! {
        todo!()
    }
}

impl<'tcx> HasDataLayout for CodegenCx<'tcx> {
    fn data_layout(&self) -> &TargetDataLayout {
        &self.tcx.data_layout
    }
}

impl<'tcx> HasParamEnv<'tcx> for CodegenCx<'tcx> {
    fn param_env(&self) -> ParamEnv<'tcx> {
        ParamEnv::reveal_all()
    }
}

impl<'tcx> HasTargetSpec for CodegenCx<'tcx> {
    fn target_spec(&self) -> &Target {
        &self.tcx.sess.target
    }
}

impl<'tcx> HasTyCtxt<'tcx> for CodegenCx<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for CodegenCx<'tcx> {
    type LayoutOfResult = TyAndLayout<'tcx>;

    fn handle_layout_err(
        &self,
        err: rustc_middle::ty::layout::LayoutError<'tcx>,
        span: rustc_span::Span,
        ty: Ty<'tcx>,
    ) -> ! {
        todo!()
    }
}

impl<'tcx> LayoutTypeMethods<'tcx> for CodegenCx<'tcx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        todo!()
    }

    fn cast_backend_type(&self, ty: &rustc_target::abi::call::CastTarget) -> Self::Type {
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn reg_backend_type(&self, ty: &rustc_target::abi::call::Reg) -> Self::Type {
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        todo!()
    }

    fn is_backend_immediate(&self, layout: TyAndLayout<'tcx>) -> bool {
        todo!()
    }

    fn is_backend_scalar_pair(&self, layout: TyAndLayout<'tcx>) -> bool {
        todo!()
    }

    fn backend_field_index(&self, layout: TyAndLayout<'tcx>, index: usize) -> u64 {
        todo!()
    }

    fn scalar_pair_element_backend_type(
        &self,
        layout: TyAndLayout<'tcx>,
        index: usize,
        immediate: bool,
    ) -> Self::Type {
        todo!()
    }
}

impl<'tcx> MiscMethods<'tcx> for CodegenCx<'tcx> {
    fn vtables(
        &self,
    ) -> &std::cell::RefCell<rustc_hash::FxHashMap<(Ty<'tcx>, Option<rustc_middle::ty::PolyExistentialTraitRef<'tcx>>), Self::Value>> {
        todo!()
    }

    fn check_overflow(&self) -> bool {
        todo!()
    }

    fn get_fn(&self, instance: Instance<'tcx>) -> Self::Function {
        todo!()
    }

    fn get_fn_addr(&self, instance: rustc_middle::ty::Instance<'tcx>) -> Self::Value {
        todo!()
    }

    fn eh_personality(&self) -> Self::Value {
        todo!()
    }

    fn sess(&self) -> &rustc_session::Session {
        todo!()
    }

    fn codegen_unit(&self) -> &'tcx rustc_middle::mir::mono::CodegenUnit<'tcx> {
        todo!()
    }

    fn set_frame_pointer_type(&self, llfn: Self::Function) {
        todo!()
    }

    fn apply_target_cpu_attr(&self, llfn: Self::Function) {
        todo!()
    }

    fn declare_c_main(&self, fn_type: Self::Type) -> Option<Self::Function> {
        todo!()
    }
}

impl<'tcx> PreDefineMethods<'tcx> for CodegenCx<'tcx> {
    fn predefine_static(
        &self,
        def_id: DefId,
        _linkage: Linkage,
        _visibility: Visibility,
        _symbol_name: &str,
    ) {
        let _attrs = self.tcx.codegen_fn_attrs(def_id);
        let instance = Instance::mono(self.tcx, def_id);
        let _ty = instance.ty(self.tcx, ty::ParamEnv::reveal_all());
    }

    fn predefine_fn(
        &self,
        instance: Instance<'tcx>,
        _linkage: Linkage,
        _visibility: Visibility,
        _symbol_name: &str,
    ) {
        assert!(!instance.substs.needs_infer());
    }
}

impl<'tcx> StaticMethods for CodegenCx<'tcx> {
    fn static_addr_of(&self, cv: Self::Value, align: rustc_abi::Align, kind: Option<&str>) -> Self::Value {
        todo!()
    }

    fn codegen_static(&self, def_id: rustc_hir::def_id::DefId, is_mutable: bool) {
        todo!()
    }

    fn add_used_global(&self, global: Self::Value) {
        todo!()
    }

    fn add_compiler_used_global(&self, global: Self::Value) {
        todo!()
    }
}

impl<'tcx> TypeMembershipMethods<'tcx> for CodegenCx<'tcx> {
    fn set_type_metadata(&self, function: Self::Function, typeid: String) {
        todo!()
    }

    fn typeid_metadata(&self, typeid: String) -> Self::Value {
        todo!()
    }

    fn set_kcfi_type_metadata(&self, function: Self::Function, typeid: u32) {
        todo!()
    }
}