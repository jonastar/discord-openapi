use core::panic;
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use crate::{
    maybe_rename_camel_case_keep_case,
    spec::{
        is_property_single_nullable, is_property_single_nullable_ref_obj, AdditionalProperties,
        ConstValue, ObjectOrReference, ParsedSpecRef, Schema, SchemaType, SchemaTypeContainer,
        Spec,
    },
};

/// Translates the complex open api 3.1 schema (with discord extensions)
/// to our internal intermediate representation that is simpler, has less ambiguity and
/// also takes care of some of the language agnostic resolving/logic to make generating code simpler
pub fn generate_ir_from_spec(spec: &Spec) -> Vec<IrType> {
    let mut out = Vec::new();
    for (name, field) in &spec.components.schemas {
        let mut ctx = IrGenContext {
            spec: &spec,
            schema: field,
            output: Vec::new(),
            frames: vec![IrGenContextFrame {
                name: Some(name.to_owned()),
                schema: field,
            }],
        };

        if let Err(err) = generate_schema_type(&mut ctx) {
            panic!("Failed generating type {name}: {err}");
        }

        for item in ctx.output {
            out.push(item);
        }
    }

    assign_enum_tag_fields(&mut out);

    out
}

fn assign_enum_tag_fields(types: &mut Vec<IrType>) {
    let cloned = types.clone();
    for item in types {
        let IrType::Enum(enum_type) = item else {
            continue;
        };

        let mut tag_field_outer: Option<String> = None;

        for variant in &mut enum_type.variants {
            let IrEnumVariant::Tuple {
                tag_values,
                tag_field,
                inner,
                name,
            } = variant
            else {
                continue;
            };

            let InlineTypeInner::Identifier(ident) = &inner.inner else {
                continue;
            };

            let found = cloned.iter().find(|v| v.identifier() == ident).unwrap();
            match found {
                IrType::Object(o) => {
                    for field in &o.fields {
                        if field.guessed_enum_tag_field {
                            *tag_values = Some(field.enum_options.clone());
                            *tag_field = Some(field.name.clone());

                            if let Some(existing_name) = tag_field_outer.as_ref() {
                                if existing_name != &field.name {
                                    panic!(
                                        "Mismatched names for tag fields! {}::{} wanted {}, but \
                                         {} already used in this enum",
                                        enum_type.name, name, field.name, existing_name
                                    );
                                }
                            } else {
                                tag_field_outer = Some(field.name.clone());
                            }
                            if tag_field_outer.is_none() {
                                tag_field_outer = Some(field.name.clone());
                            } else {
                            }
                            break;
                        }
                    }
                }
                IrType::Enum(_) => todo!(),
                IrType::Tuple(_) => todo!(),
            }

            if tag_field.is_none() {
                println!("Could not assign tag field of {}::{}", enum_type.name, name);
            }
        }

        if let Some(tag_field) = tag_field_outer {
            enum_type.tag_field = Some(tag_field)
        }
    }
}

fn generate_schema_type<'a>(ctx: &mut IrGenContext<'a>) -> Result<String, String> {
    match &ctx.schema.kind {
        Some(v) => match v {
            SchemaTypeContainer::Single(s) => generate_schema_type_with_unwrapped_type(ctx, *s),
            SchemaTypeContainer::Multiple(m) => {
                if let Some(_other) = is_property_single_nullable(m) {
                    // TODO support nullable types here
                    todo!();
                } else {
                    todo!("Mixed types")
                }
            }
        },
        None => {
            let schema = &ctx.schema;

            if let Some(one_of) = &schema.one_of {
                return gen_one_of_standalone(ctx, &one_of);
            }

            if let Some(_all_of) = &schema.all_of {
                todo!()
            }

            if let Some(any_of) = &schema.any_of {
                // Discord uses anyOf as oneOf because of oneOf limitations
                return gen_one_of_standalone(ctx, &any_of);
            }

            if let Some(_options) = &schema.enum_options {
                todo!()
            }

            if let Some(_val) = &schema.const_value {
                todo!()
            }

            todo!()
        }
    }
}

fn generate_schema_type_with_unwrapped_type<'a>(
    ctx: &mut IrGenContext<'a>,
    t: SchemaType,
) -> Result<String, String> {
    let translated_format = ctx
        .schema
        .format
        .as_deref()
        .map(InlineTypeInner::from_format);
    match &t {
        SchemaType::String => {
            if let Some(one_of) = &ctx.schema.one_of {
                generate_enum(ctx, &one_of)
            } else {
                ctx.push_tuple(IrTuple {
                    name: ctx.full_type_name(),
                    field: InlineType {
                        optional: false,
                        nullable: false,
                        inner: translated_format.unwrap_or(InlineTypeInner::String),
                    },
                });

                Ok(ctx.full_type_name())
            }
        }
        SchemaType::Integer => {
            if let Some(one_of) = &ctx.schema.one_of {
                generate_enum(ctx, &one_of)
            } else {
                // let format = ctx.schema.format.as_ref().unwrap();

                // let rust_type = match format.as_str() {
                //     "in32" => "i32",
                //     "int64" => "i64",
                //     "double" => "f64",
                //     other => return Err(format!("Unknown format for integer {other}")),
                // };
                // println!("{format}");

                ctx.push_tuple(IrTuple {
                    name: ctx.full_type_name(),
                    field: InlineType {
                        optional: false,
                        nullable: false,
                        inner: translated_format.unwrap(),
                    },
                });

                // ctx.push_tuple(GenTuple {
                //     name: ctx.full_type_name(),
                //     field: rust_type.to_string(),
                // });
                Ok(ctx.full_type_name())
            }
        }
        SchemaType::Null => todo!(),
        SchemaType::Object => generate_schema_type_object(ctx),
        SchemaType::Array => todo!(),
        SchemaType::Boolean => todo!(),
        SchemaType::Number => todo!(),
    }
}

fn generate_schema_type_object<'a>(ctx: &mut IrGenContext<'a>) -> Result<String, String> {
    let mut current_object_fields: Vec<IrObjectField> = Vec::new();

    if let Some(props) = &ctx.schema.properties {
        for (key, property) in props {
            let is_required = ctx
                .schema
                .required
                .as_ref()
                .map(|v| v.iter().any(|r| r == key))
                .unwrap_or(false);

            // Try to extract potential enum tag

            let (guessed_enum_tag_field, enum_options) =
                if let ObjectOrReference::Object(obj) = &property {
                    extract_enum_field_options(ctx, obj)
                } else {
                    (false, Vec::new())
                };

            if guessed_enum_tag_field {
                println!("Guessed tag field of {}: {key}", ctx.full_type_name());
            }

            match generate_object_field_type_or_ref(ctx, Some(key.to_owned()), &property) {
                Ok(v) => current_object_fields.push(IrObjectField {
                    name: key.to_owned(),
                    guessed_enum_tag_field,
                    enum_options,

                    kind: InlineType {
                        optional: !is_required,
                        nullable: v.nullable,
                        inner: v.inner,
                    },
                }),
                Err(err) => {
                    return Err(format!("Failed generating field {key}: {err}",));
                }
            }
        }
    }

    let name = ctx.full_type_name();

    ctx.push_object(IrObjectType {
        fields: current_object_fields,
        name: name.clone(),
    });

    Ok(name)
}

fn generate_object_field_type_or_ref<'a>(
    ctx: &mut IrGenContext<'a>,
    field_name: Option<String>,
    field_schema: &'a ObjectOrReference<Box<Schema>>,
) -> Result<InlineType, String> {
    match field_schema {
        ObjectOrReference::Ref { ref_path } => {
            let parsed = ParsedSpecRef::from_str(&ref_path).unwrap();
            Ok(InlineType {
                nullable: false,
                optional: false,
                inner: InlineTypeInner::Identifier(parsed.name().to_owned()),
            })
        }
        ObjectOrReference::Object(obj) => {
            ctx.push_frame(field_name, &obj);
            let result = generate_object_type_inline(ctx, obj);
            ctx.pop_frame();

            result
        }
    }
}

fn generate_object_type_inline<'a>(
    ctx: &mut IrGenContext<'a>,
    schema: &'a Schema,
) -> Result<InlineType, String> {
    if let Some(schema_type) = schema
        .kind
        .as_ref()
        .map(|schema_kind| match schema_kind {
            SchemaTypeContainer::Single(s) => match s {
                SchemaType::Array => Ok(InlineType {
                    inner: get_array_type(ctx, &schema)?,
                    nullable: false,
                    optional: false,
                }),
                SchemaType::Object => generate_object_inline_type_object(ctx, schema),
                other => Ok(InlineType {
                    nullable: false,
                    optional: false,
                    inner: InlineTypeInner::from_schema_type_format(*other, &schema.format),
                }),
            },

            SchemaTypeContainer::Multiple(m) => {
                if let Some(other) = is_property_single_nullable(&m) {
                    match other {
                        SchemaType::Array => Ok(InlineType {
                            inner: get_array_type(ctx, schema)?,
                            nullable: true,
                            optional: false,
                        }),
                        SchemaType::Object => {
                            let inner = generate_object_inline_type_object(ctx, schema)?;
                            // Ok(format!("Option<{inner}>"))
                            Ok(InlineType {
                                inner: inner.inner,
                                nullable: true,
                                optional: false,
                            })
                        }
                        other => Ok(InlineType {
                            nullable: true,
                            optional: false,
                            inner: InlineTypeInner::from_schema_type_format(other, &schema.format),
                        }),
                    }
                } else {
                    todo!()
                }
            }
        })
        .transpose()?
    {
        return Ok(schema_type);
    }

    if let Some(one_of) = &schema.one_of {
        return gen_one_of_inline(ctx, &one_of);
    }

    if let Some(all_of) = &schema.all_of {
        let ident = generate_enum_combined_all_of(ctx, schema, all_of, &schema.enum_options)?;
        return Ok(InlineType {
            inner: InlineTypeInner::Identifier(ident),
            nullable: false,
            optional: false,
        });
    }

    if let Some(any_of) = &schema.any_of {
        // Discord uses anyOf as oneOf because of oneOf limitations
        return gen_one_of_inline(ctx, &any_of);
    }

    if let Some(_options) = &schema.enum_options {
        todo!()
    }

    if let Some(_val) = &schema.const_value {
        todo!()
    }

    // return
    Ok(InlineType {
        inner: InlineTypeInner::Empty,
        nullable: false,
        optional: false,
    })
    // Ok("()".to_string())
    // return Err("not yet implemented support".to_string());
}

fn gen_one_of_standalone<'a>(
    ctx: &mut IrGenContext<'a>,
    one_of: &'a Vec<ObjectOrReference<Schema>>,
) -> Result<String, String> {
    if let Some(_other) = is_property_single_nullable_ref_obj(&one_of) {
        // Discord has no instances of a "one_of" for nullable single variant on a top level type
        // This could be handled either by Option<T> or a enum with none being a variant

        todo!()
    } else {
        // Generate an enum
        let ident = generate_enum(ctx, one_of)?;
        Ok(ident)
    }
}

fn gen_one_of_inline<'a>(
    ctx: &mut IrGenContext<'a>,
    one_of: &'a Vec<ObjectOrReference<Schema>>,
) -> Result<InlineType, String> {
    if let Some(other) = is_property_single_nullable_ref_obj(&one_of) {
        match other {
            ObjectOrReference::Ref { ref_path } => {
                let parsed = ParsedSpecRef::from_str(&ref_path).unwrap();
                Ok(InlineType {
                    inner: InlineTypeInner::Identifier(parsed.name().to_owned()),
                    nullable: true,
                    optional: false,
                })
                // Ok(format!("Option<{}>", parsed.name()))
            }
            ObjectOrReference::Object(obj) => {
                let inner = generate_object_type_inline(ctx, obj)?;
                // Ok(format!("Option<{inner}>"))

                Ok(InlineType {
                    inner: inner.inner,
                    nullable: true,
                    optional: false,
                })
            }
        }
    } else {
        // Generate an enum
        let ident = generate_enum(ctx, one_of)?;
        Ok(InlineType {
            inner: InlineTypeInner::Identifier(ident),
            nullable: false,
            optional: false,
        })
        // todo!()
    }
}

fn generate_enum<'a>(
    ctx: &mut IrGenContext<'a>,
    one_of: &'a Vec<ObjectOrReference<Schema>>,
) -> Result<String, String> {
    let mut building_enum = Vec::<IrEnumVariant>::new();
    let mut is_optional = false;
    for item in one_of {
        match item {
            ObjectOrReference::Ref { ref_path } => {
                let parsed = ParsedSpecRef::from_str(&ref_path).unwrap();
                building_enum.push(IrEnumVariant::Tuple {
                    name: parsed.name().to_string(),
                    inner: InlineTypeItem {
                        nullable: false,
                        inner: InlineTypeInner::Identifier(parsed.name().to_string()),
                    },
                    tag_values: None,
                    tag_field: None,
                });
            }
            ObjectOrReference::Object(o) => {
                if o.is_null() {
                    is_optional = true;
                    continue;
                }

                let name = o.name().unwrap();
                if let Some(constant_value) = &o.const_value {
                    match constant_value {
                        ConstValue::String(s) => {
                            building_enum.push(IrEnumVariant::Constant {
                                name,
                                value: StringOrInt::String(s.to_owned()),
                            });

                            // building_enum
                            //     .push(GenEnumVariant::ConstantString(name, format!("\"{}\"", s)));
                        }
                        ConstValue::Number(n) => {
                            building_enum.push(IrEnumVariant::Constant {
                                name,
                                value: StringOrInt::Int(*n as i32),
                            });

                            // building_enum
                            //     .push(GenEnumVariant::ConstantInteger(name, format!("{}", n)));
                        }
                    }
                } else {
                    let t = generate_object_type_inline(ctx, o)?;

                    building_enum.push(IrEnumVariant::Tuple {
                        name: name,
                        inner: InlineTypeItem {
                            nullable: t.nullable,
                            inner: t.inner,
                        },
                        tag_values: None,
                        tag_field: None,
                    });
                    // building_enum.push(GenEnumVariant::Tuple(name, t));
                }
            }
        }
    }

    let enum_name = ctx.full_type_name();

    ctx.push_enum(IrEnum {
        name: enum_name.clone(),
        variants: building_enum,
        tag_field: None,
    });

    if is_optional {
        return Ok(format!("Option<{enum_name}>"));
    } else {
        Ok(enum_name)
    }
}

fn generate_enum_combined_all_of<'a>(
    _ctx: &mut IrGenContext<'a>,
    _schema: &'a Schema,
    all_of: &'a Vec<ObjectOrReference<Schema>>,
    _enum_options: &'a Option<Vec<ConstValue>>,
) -> Result<String, String> {
    if all_of.len() == 1 {
        match &all_of[0] {
            ObjectOrReference::Ref { ref_path } => {
                let parsed: ParsedSpecRef = ParsedSpecRef::from_str(&ref_path).unwrap();
                return Ok(parsed.name().to_string());
            }
            ObjectOrReference::Object(_) => todo!(),
        }
    }

    todo!()
}

fn generate_object_inline_type_object<'a>(
    ctx: &mut IrGenContext<'a>,
    schema: &'a Schema,
) -> Result<InlineType, String> {
    if let Some(additional_props) = &schema.additional_properties {
        match additional_props {
            ObjectOrReference::Ref { ref_path } => {
                let parsed: ParsedSpecRef = ParsedSpecRef::from_str(&ref_path).unwrap();
                return Ok(InlineType {
                    inner: InlineTypeInner::DictString(Box::new(InlineTypeItem {
                        nullable: false,
                        inner: InlineTypeInner::Identifier(parsed.name().to_owned()),
                    })),
                    nullable: false,
                    optional: false,
                });

                // return Ok(format!("HashMap<String,{}>", parsed.name()));
            }
            ObjectOrReference::Object(obj) => match obj {
                AdditionalProperties::False(_) => {}
                AdditionalProperties::Schema(props) => {
                    if let Some(schema_type) = props
                        .kind
                        .as_ref()
                        .map(|schema_kind| match schema_kind {
                            SchemaTypeContainer::Single(s) => match s {
                                SchemaType::Array => Ok(InlineType {
                                    inner: get_array_type(ctx, schema)?,
                                    nullable: false,
                                    optional: false,
                                }),
                                SchemaType::Object => {
                                    Err("Object in additional properties".to_string())
                                }
                                other => Ok(InlineType {
                                    nullable: false,
                                    optional: false,
                                    inner: InlineTypeInner::from_schema_type_format(
                                        *other,
                                        &props.format,
                                    ),
                                }),
                            },

                            SchemaTypeContainer::Multiple(m) => {
                                if let Some(other) = is_property_single_nullable(&m) {
                                    match other {
                                        SchemaType::Array => {
                                            Ok(InlineType {
                                                inner: get_array_type(ctx, &schema)?,
                                                nullable: true,
                                                optional: false,
                                            })
                                            // Ok(format!("Option<{}>", get_array_type(ctx, schema)?))
                                        }
                                        SchemaType::Object => {
                                            Err("Object in additional properties".to_string())
                                        }
                                        other => Ok(InlineType {
                                            nullable: true,
                                            optional: false,
                                            inner: InlineTypeInner::from_schema_type_format(
                                                other,
                                                &props.format,
                                            ),
                                        }),
                                    }
                                } else {
                                    todo!()
                                }
                            }
                        })
                        .transpose()?
                    {
                        return Ok(InlineType {
                            nullable: false,
                            optional: false,
                            inner: InlineTypeInner::DictString(Box::new(InlineTypeItem {
                                nullable: schema_type.nullable,
                                inner: schema_type.inner,
                            })),
                        });
                        // return Ok(format!("HashMap<String,{schema_type}>"));
                    }
                }
            },
        }
    }

    let (kind, optional) = match &schema.kind {
        Some(k) => match k {
            SchemaTypeContainer::Single(s) => (*s, false),
            SchemaTypeContainer::Multiple(m) => {
                if let Some(other) = is_property_single_nullable(&m) {
                    (other, true)
                } else {
                    todo!()
                }
            }
        },
        None => todo!(), // None => return Ok("()".to_string()),
    };

    // let nested_type_name: String = format!("{}_{}", ctx.schema_name, field_name);
    // let mut nested_ctx = GenContext {
    //     output: Vec::new(),
    //     schema: &schema,
    //     schema_name: &nested_type_name,
    //     spec: &ctx.spec,
    // };

    // let type_name = ctx.full_type_name();

    let ident = generate_schema_type_with_unwrapped_type(ctx, kind)?;
    Ok(InlineType {
        optional: false,
        nullable: optional,
        inner: InlineTypeInner::Identifier(ident),
    })

    // if optional {
    //     Ok(format!("Option<{type_name}>"))
    // } else {
    //     Ok(type_name)
    // }
}

fn get_array_type<'a>(
    ctx: &mut IrGenContext<'a>,
    schema: &'a Schema,
) -> Result<InlineTypeInner, String> {
    if let Some(items) = &schema.items {
        let t = generate_object_field_type_or_ref(ctx, None, items)?;
        return Ok(InlineTypeInner::Array(Box::new(InlineTypeItem {
            nullable: t.nullable,
            inner: t.inner,
        })));
    }

    unreachable!()
}
pub struct IrGenContextFrame<'a> {
    name: Option<String>,
    schema: &'a Schema,
}

pub struct IrGenContext<'a> {
    spec: &'a Spec,

    schema: &'a Schema,
    // schema_name: &'a str,
    output: Vec<IrType>,

    frames: Vec<IrGenContextFrame<'a>>,
}

impl<'a> IrGenContext<'a> {
    pub fn push_object(&mut self, object: IrObjectType) {
        self.output.push(IrType::Object(object))
    }

    pub fn push_tuple(&mut self, tuple: IrTuple) {
        self.output.push(IrType::Tuple(tuple))
    }

    pub fn push_enum(&mut self, e: IrEnum) {
        self.output.push(IrType::Enum(e))
    }

    pub fn push_frame(&mut self, name: Option<String>, schema: &'a Schema) {
        self.frames.push(IrGenContextFrame { name, schema });
        self.schema = schema
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
        if let Some(frame) = self.frames.last() {
            self.schema = frame.schema
        } else {
            panic!("Popped too many frames!");
        }
    }

    pub fn full_type_name(&self) -> String {
        let mut buf = String::new();
        for item in &self.frames {
            if let Some(name) = &item.name {
                for (i, char) in name.chars().enumerate() {
                    if i == 0 {
                        for uppercase_char in char.to_uppercase() {
                            buf.write_char(uppercase_char).unwrap();
                        }
                    } else {
                        buf.write_char(char).unwrap();
                    }
                }
            }
        }

        maybe_rename_camel_case_keep_case(&buf).unwrap_or(buf)
        // buf
    }
}

fn extract_enum_field_options(ctx: &IrGenContext, obj: &Schema) -> (bool, Vec<StringOrInt>) {
    if let (Some(options), Some(all_of)) = (&obj.enum_options, &obj.all_of) {
        if all_of.len() != 1 {
            // not sure how to handle this case yet, crash for now and figure it out when its actually sued
            todo!();
        }
        let enum_options = options
            .iter()
            .map(|v| match v {
                ConstValue::String(s) => StringOrInt::String(s.clone()),
                ConstValue::Number(n) => StringOrInt::Int(*n as i32),
            })
            .collect::<Vec<_>>();

        return (true, enum_options);
    }

    (false, Vec::new())
}

#[derive(Debug, Clone)]
pub struct IrObjectField {
    pub name: String,
    pub kind: InlineType,
    pub guessed_enum_tag_field: bool,
    pub enum_options: Vec<StringOrInt>,
}

#[derive(Debug, Clone)]
pub struct IrObjectType {
    pub name: String,
    pub fields: Vec<IrObjectField>,
}

#[derive(Debug, Clone)]
pub struct IrTuple {
    pub name: String,
    pub field: InlineType,
}

#[derive(Debug, Clone)]
pub enum IrType {
    Object(IrObjectType),
    Enum(IrEnum),
    Tuple(IrTuple),
}

impl IrType {
    pub fn identifier(&self) -> &str {
        match self {
            IrType::Object(v) => &v.name,
            IrType::Enum(v) => &v.name,
            IrType::Tuple(v) => &v.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrEnum {
    pub name: String,
    pub variants: Vec<IrEnumVariant>,
    pub tag_field: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IrEnumVariant {
    Tuple {
        name: String,
        tag_values: Option<Vec<StringOrInt>>,
        tag_field: Option<String>,
        inner: InlineTypeItem,
    },
    Constant {
        name: String,
        value: StringOrInt,
    },
}

#[derive(Debug, Clone)]
pub struct InlineType {
    pub optional: bool,
    pub nullable: bool,
    pub inner: InlineTypeInner,
}
#[derive(Debug, Clone)]
pub struct InlineTypeItem {
    pub nullable: bool,
    pub inner: InlineTypeInner,
}

#[derive(Debug, Clone)]
pub enum InlineTypeInner {
    Identifier(String),
    Array(Box<InlineTypeItem>),
    DictString(Box<InlineTypeItem>),
    String,
    Double,
    Int32,
    Int64,
    DateTime,
    Uri,
    Nonce,
    Boolean,
    Snowflake,
    Null,
    Empty,
}

#[derive(Debug, Clone)]
pub enum StringOrInt {
    String(String),
    Int(i32),
}

impl Display for StringOrInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrInt::String(s) => f.write_str(s),
            StringOrInt::Int(i) => i.fmt(f),
        }
    }
}

impl InlineTypeInner {
    pub fn from_schema_type_format(
        schema_type: SchemaType,
        format: &Option<String>,
    ) -> InlineTypeInner {
        let translated_format = format.as_deref().map(Self::from_format);
        match schema_type {
            SchemaType::Object => todo!(),
            SchemaType::Array => todo!(),

            SchemaType::Null => Self::Null,
            SchemaType::String => translated_format.unwrap_or(InlineTypeInner::String),
            SchemaType::Integer => translated_format.unwrap_or(InlineTypeInner::Int32),
            SchemaType::Boolean => Self::Boolean,
            SchemaType::Number => translated_format.unwrap_or(InlineTypeInner::Double),
        }
    }

    pub fn from_format(format: &str) -> Self {
        match format {
            "int32" => Self::Int32,
            "int64" => Self::Int64,
            "uri" => Self::Uri,
            "date-time" => Self::DateTime,
            "nonce" => Self::Nonce,
            "double" => Self::Double,
            "snowflake" => Self::Snowflake,
            other => {
                panic!("Unknown format type {}", other);
            }
        }
    }
}
