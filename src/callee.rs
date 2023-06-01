use rustc_middle::ty::{layout::{HasTyCtxt, FnAbiOf}, Instance, TypeVisitableExt, self};

use crate::{cx::CodegenCx, value::Value};

pub fn get_fn<'tcx, 'xlang>(cx: &CodegenCx<'tcx, 'xlang>, instance: Instance<'tcx>) -> Value<'xlang> {
    let tcx = cx.tcx();
    // assert!(!instance.substs.needs_infer());
    assert!(!instance.substs.has_escaping_bound_vars());

    if let Some(&func) = cx.function_instances.borrow().get(&instance) {
        return func;
    }

    let sym = tcx.symbol_name(instance).name;

    let fn_abi = cx.fn_abi_of_instance(instance, ty::List::empty());

    let xlfn = if let Some(xlfn) = cx.get_declared_value(sym) {

    } else {

    };

    todo!("{sym}, {fn_abi:?}")
}
