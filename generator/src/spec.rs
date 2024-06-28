use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Spec {
    pub openapi: String,
    pub info: SpecInfo,
    #[serde(rename = "externalDocs")]
    pub external_docs: ExternalDocs,

    pub paths: HashMap<String, PathItemOuter>,
    pub components: Components,
}

pub enum ParsedSpecRef {
    Component(ComponentRef),
}

impl ParsedSpecRef {
    pub fn name(&self) -> &str {
        match self {
            ParsedSpecRef::Component(c) => match c {
                ComponentRef::Schema(name) => name,
                ComponentRef::Response(name) => name,
            },
        }
    }
}

impl FromStr for ParsedSpecRef {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(component_rest) = s.strip_prefix("#/components/") {
            if let Some(schema) = component_rest.strip_prefix("schemas/") {
                Ok(ParsedSpecRef::Component(ComponentRef::Schema(
                    schema.to_owned(),
                )))
            } else if let Some(response) = component_rest.strip_prefix("responses/") {
                Ok(ParsedSpecRef::Component(ComponentRef::Schema(
                    response.to_owned(),
                )))
            } else {
                Err(format!("Unknown component selector {}", component_rest))
            }
        } else {
            Err(format!("Unknown top level selector {}", s))
        }
    }
}

pub enum ComponentRef {
    Schema(String),
    Response(String),
}

impl Spec {
    pub fn resolve_ref(&self, str: &str) -> ResolvedReference {
        let parsed = ParsedSpecRef::from_str(str).unwrap();
        match parsed {
            ParsedSpecRef::Component(c) => self.components.resolve_ref(c),
        }
    }
}

pub enum ResolvedReference<'a> {
    Schema(&'a Schema),
    Content(&'a Content),
    PathResponse(&'a PathResponse),
}

#[derive(Deserialize, Debug)]
pub struct SpecInfo {
    pub title: String,
    pub description: String,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct PathItemOuter {
    #[serde(default)]
    pub parameters: Vec<PathParameter>,
    pub get: Option<PathItemInner>,
    pub post: Option<PathItemInner>,
    pub put: Option<PathItemInner>,
    pub delete: Option<PathItemInner>,
    pub patch: Option<PathItemInner>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathParameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_field: String,
    pub schema: ObjectOrReference<Schema>,
    pub required: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct PathItemInner {
    #[serde(rename = "operationId")]
    pub operation_id: String,

    #[serde(default)]
    pub parameters: Vec<PathParameter>,

    #[serde(rename = "requestBody")]
    pub request_body: Option<Content>,

    pub responses: HashMap<String, ObjectOrReference<PathResponse>>,

    #[serde(default)]
    pub security: Vec<PathSecurity>,
}

#[derive(Deserialize, Debug)]
pub struct PathResponse {
    pub description: Option<String>,
    pub content: Option<Content>,
}

#[derive(Deserialize, Debug)]
pub struct PathSecurity {
    #[serde(rename = "BotToken")]
    pub bot_token: Option<Vec<String>>,
    #[serde(rename = "OAuth2")]
    pub oauth: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ExternalDocs {}

#[derive(Deserialize, Debug)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,

    #[serde(rename = "securitySchemes")]
    pub security_schemes: SecuritySchemes,

    pub responses: HashMap<String, PathResponse>,
}

impl Components {
    pub fn resolve_ref(&self, selector: ComponentRef) -> ResolvedReference {
        match selector {
            ComponentRef::Schema(schema) => {
                ResolvedReference::Schema(self.schemas.get(&schema).unwrap())
            }
            ComponentRef::Response(response) => {
                ResolvedReference::PathResponse(self.responses.get(&response).unwrap())
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SecuritySchemes {
    #[serde(rename = "BotToken")]
    pub bot_token: BotSecurityScheme,
    #[serde(rename = "OAuth2")]
    pub oauth2: OauthSecurityScheme,
}

#[derive(Deserialize, Debug)]
pub struct BotSecurityScheme {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub name: String,
    #[serde(rename = "in")]
    pub in_field: String,
}

#[derive(Deserialize, Debug)]
pub struct OauthSecurityScheme {
    #[serde(rename = "type")]
    pub type_field: String,
    pub flows: OauthFlows,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OauthFlows {
    pub implicit: FlowImplicit,
    pub client_credentials: FlowClientCredentials,
    pub authorization_code: FlowAuthorizationCode,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowImplicit {
    pub authorization_url: String,
    pub refresh_url: String,
    pub scopes: ScopesImplicit,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesImplicit {
    #[serde(rename = "activities.read")]
    pub activities_read: String,
    #[serde(rename = "activities.write")]
    pub activities_write: String,
    #[serde(rename = "applications.builds.read")]
    pub applications_builds_read: String,
    #[serde(rename = "applications.builds.upload")]
    pub applications_builds_upload: String,
    #[serde(rename = "applications.commands")]
    pub applications_commands: String,
    #[serde(rename = "applications.commands.permissions.update")]
    pub applications_commands_permissions_update: String,
    #[serde(rename = "applications.entitlements")]
    pub applications_entitlements: String,
    #[serde(rename = "applications.store.update")]
    pub applications_store_update: String,
    pub bot: String,
    pub connections: String,
    #[serde(rename = "dm_channels.read")]
    pub dm_channels_read: String,
    pub email: String,
    #[serde(rename = "gdm.join")]
    pub gdm_join: String,
    pub guilds: String,
    #[serde(rename = "guilds.join")]
    pub guilds_join: String,
    #[serde(rename = "guilds.members.read")]
    pub guilds_members_read: String,
    pub identify: String,
    #[serde(rename = "messages.read")]
    pub messages_read: String,
    pub openid: String,
    #[serde(rename = "relationships.read")]
    pub relationships_read: String,
    pub rpc: String,
    #[serde(rename = "rpc.activities.write")]
    pub rpc_activities_write: String,
    #[serde(rename = "rpc.notifications.read")]
    pub rpc_notifications_read: String,
    #[serde(rename = "rpc.screenshare.read")]
    pub rpc_screenshare_read: String,
    #[serde(rename = "rpc.screenshare.write")]
    pub rpc_screenshare_write: String,
    #[serde(rename = "rpc.video.read")]
    pub rpc_video_read: String,
    #[serde(rename = "rpc.video.write")]
    pub rpc_video_write: String,
    #[serde(rename = "rpc.voice.read")]
    pub rpc_voice_read: String,
    #[serde(rename = "rpc.voice.write")]
    pub rpc_voice_write: String,
    pub voice: String,
    #[serde(rename = "webhook.incoming")]
    pub webhook_incoming: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowClientCredentials {
    pub token_url: String,
    pub refresh_url: String,
    pub scopes: ScopesClientCredentials,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesClientCredentials {
    #[serde(rename = "activities.read")]
    pub activities_read: String,
    #[serde(rename = "activities.write")]
    pub activities_write: String,
    #[serde(rename = "applications.builds.read")]
    pub applications_builds_read: String,
    #[serde(rename = "applications.builds.upload")]
    pub applications_builds_upload: String,
    #[serde(rename = "applications.commands")]
    pub applications_commands: String,
    #[serde(rename = "applications.commands.permissions.update")]
    pub applications_commands_permissions_update: String,
    #[serde(rename = "applications.commands.update")]
    pub applications_commands_update: String,
    #[serde(rename = "applications.entitlements")]
    pub applications_entitlements: String,
    #[serde(rename = "applications.store.update")]
    pub applications_store_update: String,
    pub bot: String,
    pub connections: String,
    #[serde(rename = "dm_channels.read")]
    pub dm_channels_read: String,
    pub email: String,
    #[serde(rename = "gdm.join")]
    pub gdm_join: String,
    pub guilds: String,
    #[serde(rename = "guilds.join")]
    pub guilds_join: String,
    #[serde(rename = "guilds.members.read")]
    pub guilds_members_read: String,
    pub identify: String,
    #[serde(rename = "messages.read")]
    pub messages_read: String,
    pub openid: String,
    #[serde(rename = "relationships.read")]
    pub relationships_read: String,
    pub rpc: String,
    #[serde(rename = "rpc.activities.write")]
    pub rpc_activities_write: String,
    #[serde(rename = "rpc.notifications.read")]
    pub rpc_notifications_read: String,
    #[serde(rename = "rpc.screenshare.read")]
    pub rpc_screenshare_read: String,
    #[serde(rename = "rpc.screenshare.write")]
    pub rpc_screenshare_write: String,
    #[serde(rename = "rpc.video.read")]
    pub rpc_video_read: String,
    #[serde(rename = "rpc.video.write")]
    pub rpc_video_write: String,
    #[serde(rename = "rpc.voice.read")]
    pub rpc_voice_read: String,
    #[serde(rename = "rpc.voice.write")]
    pub rpc_voice_write: String,
    pub voice: String,
    #[serde(rename = "webhook.incoming")]
    pub webhook_incoming: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAuthorizationCode {
    pub authorization_url: String,
    pub token_url: String,
    pub refresh_url: String,
    pub scopes: ScopesAuthorizationCode,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesAuthorizationCode {
    #[serde(rename = "activities.read")]
    pub activities_read: String,
    #[serde(rename = "activities.write")]
    pub activities_write: String,
    #[serde(rename = "applications.builds.read")]
    pub applications_builds_read: String,
    #[serde(rename = "applications.builds.upload")]
    pub applications_builds_upload: String,
    #[serde(rename = "applications.commands")]
    pub applications_commands: String,
    #[serde(rename = "applications.commands.permissions.update")]
    pub applications_commands_permissions_update: String,
    #[serde(rename = "applications.entitlements")]
    pub applications_entitlements: String,
    #[serde(rename = "applications.store.update")]
    pub applications_store_update: String,
    pub bot: String,
    pub connections: String,
    #[serde(rename = "dm_channels.read")]
    pub dm_channels_read: String,
    pub email: String,
    #[serde(rename = "gdm.join")]
    pub gdm_join: String,
    pub guilds: String,
    #[serde(rename = "guilds.join")]
    pub guilds_join: String,
    #[serde(rename = "guilds.members.read")]
    pub guilds_members_read: String,
    pub identify: String,
    #[serde(rename = "messages.read")]
    pub messages_read: String,
    pub openid: String,
    #[serde(rename = "relationships.read")]
    pub relationships_read: String,
    #[serde(rename = "role_connections.write")]
    pub role_connections_write: String,
    pub rpc: String,
    #[serde(rename = "rpc.activities.write")]
    pub rpc_activities_write: String,
    #[serde(rename = "rpc.notifications.read")]
    pub rpc_notifications_read: String,
    #[serde(rename = "rpc.screenshare.read")]
    pub rpc_screenshare_read: String,
    #[serde(rename = "rpc.screenshare.write")]
    pub rpc_screenshare_write: String,
    #[serde(rename = "rpc.video.read")]
    pub rpc_video_read: String,
    #[serde(rename = "rpc.video.write")]
    pub rpc_video_write: String,
    #[serde(rename = "rpc.voice.read")]
    pub rpc_voice_read: String,
    #[serde(rename = "rpc.voice.write")]
    pub rpc_voice_write: String,
    pub voice: String,
    #[serde(rename = "webhook.incoming")]
    pub webhook_incoming: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "application/json")]
    pub application_json: Option<ContentApplicationJson>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentApplicationJson {
    pub schema: ObjectOrReference<Schema>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSchemaRef {
    #[serde(rename = "$ref")]
    pub ref_field: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "type")]
    pub kind: Option<SchemaTypeContainer>,
    // pub kind: Option<ObjectOrReference<SchemaTypeContainer>>,
    pub properties: Option<HashMap<String, ObjectOrReference<Box<Schema>>>>,
    pub additional_properties: Option<ObjectOrReference<AdditionalProperties>>,
    pub required: Option<Vec<String>>,

    pub items: Option<ObjectOrReference<Box<Schema>>>,

    #[serde(rename = "minLength")]
    pub min_length: Option<u32>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u32>,

    pub format: Option<String>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<ObjectOrReference<Schema>>>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<ObjectOrReference<Schema>>>,
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<ObjectOrReference<Schema>>>,
    #[serde(rename = "enum")]
    pub enum_options: Option<Vec<ConstValue>>,

    #[serde(rename = "const")]
    pub const_value: Option<ConstValue>,
    pub title: Option<String>,
    pub description: Option<String>,

    #[serde(rename = "x-discord-union")]
    pub x_discord_union: Option<String>,
}

impl Schema {
    pub fn is_null(&self) -> bool {
        matches!(
            self.kind,
            Some(SchemaTypeContainer::Single(SchemaType::Null))
        )
    }

    pub fn name(&self) -> Option<String> {
        if let Some(title) = &self.title {
            return Some(title.to_owned());
        }

        if let Some(additional_props) = &self.additional_properties {
            match additional_props {
                ObjectOrReference::Ref { ref_path } => {
                    let parsed = ParsedSpecRef::from_str(&ref_path).unwrap();
                    return Some(parsed.name().to_owned());
                }
                ObjectOrReference::Object(_) => todo!(),
            }
        }

        if let Some(kind) = &self.kind {
            match kind {
                SchemaTypeContainer::Single(s) => return Some(s.enum_name()),
                SchemaTypeContainer::Multiple(m) => {
                    if let Some(kind) = is_property_single_nullable(m) {
                        return Some(kind.enum_name());
                    }
                }
            }
        }

        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConstValue {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaTypeContainer {
    Single(SchemaType),
    Multiple(Vec<SchemaType>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum SchemaType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "number")]
    Number,
}

impl SchemaType {
    pub fn enum_name(&self) -> String {
        match self {
            SchemaType::String => "String".to_owned(),
            SchemaType::Null => "Null".to_owned(),
            SchemaType::Object => "Object".to_owned(),
            SchemaType::Integer => "Integer".to_owned(),
            SchemaType::Array => "Array".to_owned(),
            SchemaType::Boolean => "Boolean".to_owned(),
            SchemaType::Number => "Number".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ObjectOrReference<T> {
    Ref {
        #[serde(rename = "$ref")]
        ref_path: String,
    },
    Object(T),
}

impl ObjectOrReference<Schema> {
    pub fn resolve<'a>(&'a self, spec: &'a Spec) -> ResolvedReference<'a> {
        match self {
            ObjectOrReference::Ref { ref_path } => spec.resolve_ref(ref_path),
            ObjectOrReference::Object(o) => ResolvedReference::Schema(o),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    False(bool),
    Schema(Box<Schema>),
}

pub fn is_property_single_nullable(t: &Vec<SchemaType>) -> Option<SchemaType> {
    if t.len() != 2 {
        return None;
    }

    if t[0] == SchemaType::Null {
        Some(t[1])
    } else if t[1] == SchemaType::Null {
        Some(t[0])
    } else {
        None
    }
}

pub fn is_property_single_nullable_ref_obj(
    t: &Vec<ObjectOrReference<Schema>>,
) -> Option<&ObjectOrReference<Schema>> {
    if t.len() != 2 {
        return None;
    }

    let maybe = t.iter().position(|v| match v {
        ObjectOrReference::Ref { .. } => false,
        ObjectOrReference::Object(o) => match o.kind {
            // Some(ObjectOrReference::Object(SchemaTypeContainer::Single(SchemaType::Null))) => true,
            Some(SchemaTypeContainer::Single(SchemaType::Null)) => true,
            _ => false,
        },
    });

    maybe.map(|v| if v == 0 { &t[1] } else { &t[0] })
}
