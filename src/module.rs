use std::io::Write;
use std::sync::LazyLock;

use xlang::{abi::{traits::DynBox, io::WriteAdapter, traits::DynMut}, plugin::OutputMode};
use xlang::{host::dso::Handle, plugin::XLangCodegen};
use xlang::abi::rustcall;

struct StaticData {
    codegen_handles: Vec<Handle>,
}

unsafe impl Send for StaticData {}
unsafe impl Sync for StaticData {}

static STATIC_DATA: LazyLock<StaticData> = LazyLock::new(|| StaticData {
    codegen_handles: vec![
        Handle::open("/usr/local/lib/lccc/xlang/plugins/libcodegen_x86.so")
            .expect("couldn't load x86 codegen"),
    ],
});

pub struct Module {
    pub codegen: DynBox<dyn XLangCodegen>,
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}

type CodegenInit = rustcall!(extern "rustcall" fn() -> DynBox<dyn XLangCodegen>);

impl Module {
    pub fn new() -> Self {
        let constructor: CodegenInit = unsafe { STATIC_DATA.codegen_handles[0].function_sym("xlang_backend_main") }.expect("codegor forgor constructor");
        Self {
            codegen: constructor(),
        }
    }

    pub fn write(&mut self, x: impl Write + 'static) {
        let mut writer = WriteAdapter::new(x);
        self.codegen.write_output(DynMut::unsize_mut(&mut writer), OutputMode::Obj).unwrap();
    }
}
