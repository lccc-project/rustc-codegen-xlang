use rustc_codegen_ssa::traits::{ModuleBufferMethods, ThinBufferMethods};

pub struct ModuleBuffer {}

impl ModuleBufferMethods for ModuleBuffer {
    fn data(&self) -> &[u8] {
        todo!()
    }
}

pub struct ThinBuffer {}

impl ThinBufferMethods for ThinBuffer {
    fn data(&self) -> &[u8] {
        todo!()
    }
}