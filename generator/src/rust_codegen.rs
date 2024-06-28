use std::{collections::HashMap, fmt::Write, sync::OnceLock};

use crate::{
    code_gen::CodeGenBackend,
    ir::{InlineType, InlineTypeInner, IrEnumVariant},
    maybe_rename_camel_case,
};

pub struct RustCodeGen {}

impl RustCodeGen {
    fn inline_type_inner_to_rust(output: &mut String, t: &InlineTypeInner) {
        match t {
            InlineTypeInner::Identifier(ident) => output.write_str(ident).unwrap(),
            InlineTypeInner::Array(items) => {
                output.write_str("Vec<").unwrap();
                Self::inline_type_to_rust(output, &items.inner, items.nullable);
                output.write_str(">").unwrap();
            }
            InlineTypeInner::DictString(items) => {
                output.write_str("HashMap<String, ").unwrap();
                Self::inline_type_to_rust(output, &items.inner, items.nullable);
                output.write_str(">").unwrap();
            }
            InlineTypeInner::String => output.write_str("String").unwrap(),
            InlineTypeInner::Double => output.write_str("f64").unwrap(),
            InlineTypeInner::Int32 => output.write_str("i32").unwrap(),
            InlineTypeInner::Int64 => output.write_str("i64").unwrap(),
            InlineTypeInner::DateTime => output.write_str("String").unwrap(),
            InlineTypeInner::Uri => output.write_str("String").unwrap(),
            InlineTypeInner::Nonce => output.write_str("String").unwrap(),
            InlineTypeInner::Boolean => output.write_str("bool").unwrap(),
            InlineTypeInner::Snowflake => output.write_str("String").unwrap(),
            InlineTypeInner::Null => output.write_str("()").unwrap(),
            InlineTypeInner::Empty => output.write_str("()").unwrap(),
        }
    }

    fn inline_type_to_rust(output: &mut String, t: &InlineTypeInner, nullable: bool) {
        if nullable {
            output.write_str("Option<").unwrap();
        }

        Self::inline_type_inner_to_rust(output, &t);

        if nullable {
            output.write_str(">").unwrap()
        }
    }
}

impl CodeGenBackend for RustCodeGen {
    fn generate_enum(
        &self,
        ctx: &crate::code_gen::GenContext,
        gen_enum: &crate::ir::IrEnum,
        output: &mut String,
    ) {
        let mut buf = String::new();
        buf.write_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n")
            .unwrap();

        let mut gen_into_i16 = false;
        let mut try_from_buf = String::new();

        // Potentially generate conversion methods for a integer constant enum
        if let Some(first) = gen_enum.variants.first() {
            match first {
                IrEnumVariant::Tuple {
                    name,
                    tag_field,
                    tag_values,
                    inner,
                } => {
                    // if tag.is_none() {
                    //     panic!("Tag field is none for {}::{}", gen_enum.name, name)
                    // }
                }
                IrEnumVariant::Constant { name, value } => match value {
                    crate::ir::StringOrInt::String(_) => {}
                    crate::ir::StringOrInt::Int(i) => {
                        buf.write_str("#[serde(try_from = \"i16\")]\n").unwrap();
                        buf.write_str("#[serde(into = \"i16\")]\n").unwrap();
                        gen_into_i16 = true;

                        try_from_buf
                            .write_str(&format!("impl TryFrom<i16> for {} {{\n", gen_enum.name))
                            .unwrap();
                        try_from_buf
                            .write_str("    type Error = String;\n")
                            .unwrap();
                        try_from_buf
                            .write_str("    fn try_from(v: i16) -> Result<Self, Self::Error> {\n")
                            .unwrap();
                        try_from_buf.write_str("        match v {\n").unwrap();
                    }
                },
            }
        }

        buf.write_str("pub enum ").unwrap();
        buf.write_str(&gen_enum.name).unwrap();
        buf.write_str(" {\n").unwrap();

        for item in &gen_enum.variants {
            buf.write_str("    ").unwrap();
            match item {
                IrEnumVariant::Tuple {
                    name,
                    tag_field,
                    tag_values,
                    inner,
                } => {
                    if let (Some(field), Some(values)) = (tag_field, tag_values) {
                        buf.write_str("// ").unwrap();
                        buf.write_str(&field).unwrap();
                        buf.write_str(" = ").unwrap();
                        buf.write_str(
                            &values
                                .iter()
                                .map(|v| v.to_string())
                                .collect::<Vec<_>>()
                                .join(", "),
                        )
                        .unwrap();
                        buf.write_str("\n    ").unwrap();
                    }
                    buf.write_str(&name).unwrap();
                    buf.write_str("(").unwrap();
                    Self::inline_type_to_rust(&mut buf, &inner.inner, inner.nullable);
                    buf.write_str("),\n").unwrap();
                }
                IrEnumVariant::Constant { name, value } => {
                    let renamed_name = maybe_rename_camel_case(name);
                    let use_name = renamed_name.as_ref().unwrap_or(name);

                    match value {
                        crate::ir::StringOrInt::String(value_string) => {
                            buf.write_str("#[serde(rename=\"").unwrap();
                            buf.write_str(&value_string).unwrap();
                            buf.write_str("\")]\n    ").unwrap();

                            buf.write_str(&use_name).unwrap();
                            buf.write_str(",\n").unwrap();
                        }
                        crate::ir::StringOrInt::Int(value_int) => {
                            buf.write_str(&use_name).unwrap();
                            buf.write_str(" = ").unwrap();
                            buf.write_str(&value_int.to_string()).unwrap();
                            buf.write_str(",\n").unwrap();

                            try_from_buf.write_str("            ").unwrap();
                            try_from_buf.write_str(&value_int.to_string()).unwrap();
                            try_from_buf.write_str(" => Ok(Self::").unwrap();
                            try_from_buf.write_str(&use_name).unwrap();
                            try_from_buf.write_str("),\n").unwrap();
                        }
                    }
                }
            }
        }

        buf.write_str("}\n").unwrap();

        if gen_into_i16 {
            buf.write_str(&format!(
                "impl From<{}> for i16 {{
    fn from(v: {}) -> Self {{
        v as i16
    }}
}}
",
                &gen_enum.name, &gen_enum.name,
            ))
            .unwrap();
        }

        if !try_from_buf.is_empty() {
            try_from_buf
                .write_str(
                    "            other => Err(format!(\"Unimplemented variant {}\", other)),\n",
                )
                .unwrap();
            try_from_buf.write_str("        }\n    }\n}\n").unwrap();

            buf.write_str(&try_from_buf).unwrap();
        }

        output.write_str(&buf).unwrap()
    }

    fn generate_object(
        &self,
        ctx: &crate::code_gen::GenContext,
        gen_obj: &crate::ir::IrObjectType,
        output: &mut String,
    ) {
        output
            .write_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n")
            .unwrap();
        output.write_str("pub struct ").unwrap();
        output.write_str(&gen_obj.name).unwrap();
        output.write_str(" {\n").unwrap();

        for field in &gen_obj.fields {
            let rename_name = check_rename_field_keyword(&field.name);
            let name = rename_name.unwrap_or(&field.name);

            if field.kind.optional {
                output.write_str("    // Optional TODO\n").unwrap();
            }

            if rename_name.is_some() {
                output.write_str("    #[serde(rename=\"").unwrap();
                output.write_str(&field.name).unwrap();
                output.write_str("\")]\n").unwrap();
            }

            output.write_str("    pub ").unwrap();
            output.write_str(&name).unwrap();
            output.write_str(": ").unwrap();
            Self::inline_type_to_rust(output, &field.kind.inner, field.kind.nullable);
            // output.write_str(&field.kind).unwrap();
            output.write_str(",\n").unwrap();
        }

        output.write_str("}\n").unwrap();
    }

    fn generate_tuple(
        &self,
        ctx: &crate::code_gen::GenContext,
        gen_tuple: &crate::ir::IrTuple,
        output: &mut String,
    ) {
        output
            .write_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n")
            .unwrap();
        output.write_str("pub struct ").unwrap();
        output.write_str(&gen_tuple.name).unwrap();
        output.write_str("(pub ").unwrap();
        Self::inline_type_to_rust(output, &gen_tuple.field.inner, gen_tuple.field.nullable);
        // output.write_str(&gen_tuple.field).unwrap();
        output.write_str(");\n").unwrap();
    }
}

fn check_rename_field_keyword(input: &str) -> Option<&'static str> {
    // n.b. static items do not call [`Drop`] on program termination, so if
    // [`DeepThought`] impls Drop, that will not be used for this instance.
    static COMPUTATION: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    let map = COMPUTATION
        .get_or_init(|| [("type", "kind"), ("ref", "reference"), ("use", "_use")].into());
    map.get(input).copied()
}
