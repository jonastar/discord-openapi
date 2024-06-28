use std::fmt::Write;

use crate::ir::{IrEnum, IrObjectType, IrTuple, IrType};

pub fn generate_code<T: CodeGenBackend>(types: Vec<IrType>, backend: T) -> String {
    let mut buf = String::new();

    let ctx = GenContext { types: types };

    for ir_type in &ctx.types {
        match ir_type {
            IrType::Object(o) => backend.generate_object(&ctx, &o, &mut buf),
            IrType::Enum(e) => backend.generate_enum(&ctx, &e, &mut buf),
            IrType::Tuple(t) => backend.generate_tuple(&ctx, &t, &mut buf),
        }

        buf.write_str("\n").unwrap();
    }

    buf
}

pub struct GenContext {
    pub types: Vec<IrType>,
}
pub trait CodeGenBackend {
    fn generate_enum(&self, ctx: &GenContext, gen_enum: &IrEnum, output: &mut String);
    fn generate_object(&self, ctx: &GenContext, gen_obj: &IrObjectType, output: &mut String);
    fn generate_tuple(&self, ctx: &GenContext, gen_tuple: &IrTuple, output: &mut String);
}
