use std::io::Write;
use std::sync::LazyLock;

use xlang::abi::collection::HashMap as AbiHashMap;
use xlang::abi::rustcall;
use xlang::plugin::XLangPlugin;
use xlang::{
    abi::{io::WriteAdapter, traits::DynBox, traits::DynMut},
    ir::{self, AnnotatedElement},
    plugin::OutputMode,
    targets::Target,
};
use xlang::{host::dso::Handle, plugin::XLangCodegen};

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
    codegen: DynBox<dyn XLangCodegen>,
    file: ir::File,
}

unsafe impl Send for Module {}
unsafe impl Sync for Module {}

type CodegenInit = rustcall!(extern "rustcall" fn() -> DynBox<dyn XLangCodegen>);

impl Module {
    pub fn new(target: impl Into<Target>) -> Self {
        let constructor: CodegenInit =
            unsafe { STATIC_DATA.codegen_handles[0].function_sym("xlang_backend_main") }
                .expect("codegor forgor constructor");
        let mut codegen = constructor();
        let target = target.into();
        codegen.set_target(target.clone());
        Self {
            codegen,
            file: ir::File {
                target,
                root: ir::Scope {
                    annotations: AnnotatedElement::default(),
                    members: AbiHashMap::default(),
                },
            },
        }
    }

    pub fn write(&mut self, x: impl Write + 'static) {
        let mut writer = WriteAdapter::new(x);
        self.codegen.accept_ir(&mut self.file);
        self.codegen
            .write_output(DynMut::unsize_mut(&mut writer), OutputMode::Obj)
            .unwrap();
    }
}
