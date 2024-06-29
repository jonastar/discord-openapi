use std::{collections::HashMap, fmt::Write, sync::OnceLock};

use crate::{
    code_gen::CodeGenBackend,
    ir::{InlineTypeInner, IrEnumVariant, StringOrInt},
    maybe_rename_camel_case,
};

pub struct RustCodeGen {}

impl RustCodeGen {
    fn inline_type_inner_to_rust(output: &mut String, t: &InlineTypeInner) {
        match t {
            InlineTypeInner::Identifier(ident) => output.write_str(ident).unwrap(),
            InlineTypeInner::Array(items) => {
                output.write_str("Vec<").unwrap();
                Self::inline_type_to_rust_buf(output, &items.inner, items.nullable);
                output.write_str(">").unwrap();
            }
            InlineTypeInner::DictString(items) => {
                output.write_str("HashMap<String, ").unwrap();
                Self::inline_type_to_rust_buf(output, &items.inner, items.nullable);
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

    fn inline_type_to_rust_buf(output: &mut String, t: &InlineTypeInner, nullable: bool) {
        if nullable {
            output.write_str("Option<").unwrap();
        }

        Self::inline_type_inner_to_rust(output, &t);

        if nullable {
            output.write_str(">").unwrap()
        }
    }

    fn inline_type_to_rust(t: &InlineTypeInner, nullable: bool) -> String {
        let mut buf = String::new();
        Self::inline_type_to_rust_buf(&mut buf, t, nullable);
        buf
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

        let mut gen_into_i16 = false;
        let mut try_from_buf = String::new();

        let mut gen_internal_tag_des: Vec<GenDesVariant> = Vec::new();
        let mut gen_internal_tag_des_kind: Option<EnumTagKind> = None;
        let mut add_serde_deserialize = true;
        let mut add_serde_untagged = false;

        // Potentially generate conversion methods and custom deserialization
        for variant in &gen_enum.variants {
            match variant {
                IrEnumVariant::Tuple {
                    name,
                    tag_field,
                    tag_values,
                    inner,
                } => {
                    if let (Some(_), Some(values)) = (tag_field, tag_values) {
                        gen_internal_tag_des.push(GenDesVariant {
                            ident: name.to_owned(),
                            inner_struct_ident: Self::inline_type_to_rust(&inner.inner, false),
                            tags: values.clone(),
                        });
                        add_serde_deserialize = false;
                        add_serde_untagged = true;
                        gen_internal_tag_des_kind = Some(match values.first() {
                            Some(StringOrInt::Int(_)) => EnumTagKind::Int,
                            Some(StringOrInt::String(_)) => EnumTagKind::String,
                            None => todo!(),
                        });
                    }
                }
                _ => {}
            }
        }

        buf.write_str("#[derive(Serialize, ").unwrap();
        if add_serde_deserialize {
            buf.write_str("Deserialize, ").unwrap();
        }
        buf.write_str("Debug, Clone)]\n").unwrap();

        if add_serde_untagged {
            buf.write_str("#[serde(untagged)]\n").unwrap();
        }

        if let Some(first) = gen_enum.variants.first() {
            match first {
                IrEnumVariant::Tuple { .. } => {}
                IrEnumVariant::Constant { value, .. } => match value {
                    crate::ir::StringOrInt::String(_) => {}
                    crate::ir::StringOrInt::Int(_) => {
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
                    Self::inline_type_to_rust_buf(&mut buf, &inner.inner, inner.nullable);
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

        if let Some(gen_enum_kind) = gen_internal_tag_des_kind {
            buf.write_str(&generate_internal_tag_enum_deserialization(
                gen_enum.name.to_owned(),
                gen_enum.tag_field.clone().unwrap(),
                gen_enum_kind,
                gen_internal_tag_des,
            ))
            .unwrap();
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
            Self::inline_type_to_rust_buf(output, &field.kind.inner, field.kind.nullable);
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
        Self::inline_type_to_rust_buf(output, &gen_tuple.field.inner, gen_tuple.field.nullable);
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

struct GenDesVariant {
    tags: Vec<StringOrInt>,
    ident: String,
    inner_struct_ident: String,
}

pub enum EnumTagKind {
    String,
    Int,
}

fn generate_internal_tag_enum_deserialization(
    enum_ident: String,
    tag_field: String,
    tag_kind: EnumTagKind,
    variants: Vec<GenDesVariant>,
) -> String {
    let visitor = match tag_kind {
        EnumTagKind::String => {
            format!(
                "StringTagVisitor {{
    field_name: \"{tag_field}\".to_string(),
}}"
            )
        }
        EnumTagKind::Int => {
            format!(
                "IntTagVisitor {{
    field_name: \"{tag_field}\".to_string(),
}}"
            )
        }
    };

    let mut tag_matches = String::new();
    let mut expected = String::new();

    for variant in variants {
        let match_arm = variant
            .tags
            .iter()
            .map(|v| match v {
                StringOrInt::String(s) => format!("\"{s}\""),
                StringOrInt::Int(i) => i.to_string(),
            })
            .collect::<Vec<_>>()
            .join(" | ");

        let variant_ident = variant.ident;
        let inner_ident = variant.inner_struct_ident;

        tag_matches
            .write_str(&format!(
                r#"            {match_arm} => Ok({enum_ident}::{variant_ident}({inner_ident}::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
"#
            ))
            .unwrap();

        for tag in variant.tags {
            if !expected.is_empty() {
                expected.write_str(", ").unwrap();
            }

            expected.write_str(&format!("\"{tag}\"")).unwrap();
        }
    }

    let output = format!(
        r#"impl<'de> Deserialize<'de> for {enum_ident} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{{Content, ContentRefDeserializer}};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = {visitor};
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {{
{tag_matches}
            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &[{expected}],
            )),
        }}
    }}
}}
"#
    );

    output
}
