use xlang::ir;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value<'xlang> {
    value: &'xlang ir::Value,
}
