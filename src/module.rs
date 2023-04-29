use crate::XlangCodegenState;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct Module {}

impl Module {
    pub fn new(_codegen_state: &Arc<RwLock<XlangCodegenState>>) -> Self {
        Self {}
    }
}
