use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::{IntTagVisitor, StringTagVisitor};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageScheduledEventPatchRequestPartial {
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub image: Option<String>,
    // Optional TODO
    pub scheduled_start_time: String,
    // Optional TODO
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub entity_type: Option<i32>,
    // Optional TODO
    pub status: Option<GuildScheduledEventStatuses>,
    // Optional TODO
    pub entity_metadata: Option<EntityMetadataStageInstance>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerPackResponse {
    pub sku_id: SnowflakeType,
    // Optional TODO
    pub banner_asset_id: Option<SnowflakeType>,
    pub name: String,
    pub stickers: Vec<StandardStickerResponse>,
    pub id: SnowflakeType,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub cover_sticker_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMessageInteractionCallbackRequest {
    // Optional TODO
    pub data: Option<IncomingWebhookUpdateForInteractionCallbackRequestPartial>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCheckRunOutput {
    // Optional TODO
    pub summary: Option<String>,
    // Optional TODO
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuth2Key {
    #[serde(rename = "use")]
    pub _use: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
    pub e: String,
    pub alg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VanityURLErrorResponse {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetChannel {
    pub name: String,
    pub position: i32,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialGuildSubscriptionIntegrationResponse {
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub account: Option<AccountResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildScheduledEventPrivacyLevels {
    GuildOnly = 2,
}
impl From<GuildScheduledEventPrivacyLevels> for i16 {
    fn from(v: GuildScheduledEventPrivacyLevels) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildScheduledEventPrivacyLevels {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            2 => Ok(Self::GuildOnly),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentInputTextResponse {
    // Optional TODO
    pub max_length: Option<i32>,
    pub style: TextStyleTypes,
    // Optional TODO
    pub min_length: Option<i32>,
    // Optional TODO
    pub value: Option<String>,
    // Optional TODO
    pub placeholder: Option<String>,
    pub custom_id: String,
    // Optional TODO
    pub required: Option<bool>,
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandChannelOptionResponse {
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub description_localized: Option<String>,
    pub name: String,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub channel_types: Option<Vec<ChannelTypes>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockMessageAction {
    // Optional TODO
    pub metadata: Option<BlockMessageActionMetadata>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMessageInteractionCallbackRequest {
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub data: Option<IncomingWebhookInteractionRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSelectDefaultValue {
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlagToChannelActionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub metadata: FlagToChannelActionMetadataResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MLSpamTriggerMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceRegionResponse {
    pub name: String,
    pub custom: bool,
    pub deprecated: bool,
    pub optimal: bool,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandAttachmentOption {
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionNumberChoiceResponse {
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: f64,
    // Optional TODO
    pub name_localized: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum EntitlementTenantFulfillmentStatusResponse {
    Unknown = 0,
    FulfillmentNotNeeded = 1,
    FulfillmentNeeded = 2,
    Fulfilled = 3,
    FulfillmentFailed = 4,
    UnfulfillmentNeeded = 5,
    Unfulfilled = 6,
    UnfulfillmentFailed = 7,
}
impl From<EntitlementTenantFulfillmentStatusResponse> for i16 {
    fn from(v: EntitlementTenantFulfillmentStatusResponse) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for EntitlementTenantFulfillmentStatusResponse {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::FulfillmentNotNeeded),
            2 => Ok(Self::FulfillmentNeeded),
            3 => Ok(Self::Fulfilled),
            4 => Ok(Self::FulfillmentFailed),
            5 => Ok(Self::UnfulfillmentNeeded),
            6 => Ok(Self::Unfulfilled),
            7 => Ok(Self::UnfulfillmentFailed),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub message: String,
    pub code: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubReview {
    pub user: GithubUser,
    // Optional TODO
    pub body: Option<String>,
    pub html_url: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Int53Type(pub i64);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageInstanceResponse {
    // Optional TODO
    pub guild_scheduled_event_id: Option<SnowflakeType>,
    pub channel_id: SnowflakeType,
    pub privacy_level: StageInstancesPrivacyLevels,
    pub topic: String,
    pub id: SnowflakeType,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub discoverable_disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntitlementResponse {
    // Optional TODO
    pub ends_at: Option<String>,
    pub deleted: bool,
    pub sku_id: SnowflakeType,
    pub id: SnowflakeType,
    // Optional TODO
    pub starts_at: Option<String>,
    pub application_id: SnowflakeType,
    // Optional TODO
    pub fulfilled_at: Option<String>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    // Optional TODO
    pub fulfillment_status: Option<EntitlementTenantFulfillmentStatusResponse>,
    pub user_id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: EntitlementTypes,
    // Optional TODO
    pub consumed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageScheduledEventResponse {
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub user_rsvp: Option<ScheduledEventUserResponse>,
    pub name: String,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub creator_id: Option<SnowflakeType>,
    // Optional TODO
    pub creator: Option<UserResponse>,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub image: Option<String>,
    pub id: SnowflakeType,
    // Optional TODO
    pub entity_id: Option<SnowflakeType>,
    pub status: GuildScheduledEventStatuses,
    pub entity_type: i32,
    // Optional TODO
    pub entity_metadata: Option<EntityMetadataStageInstanceResponse>,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    pub scheduled_start_time: String,
    // Optional TODO
    pub user_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Button {
    pub style: ButtonStyleTypes,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub label: Option<String>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub disabled: Option<bool>,
    // Optional TODO
    pub emoji: Option<Emoji>,
    // Optional TODO
    pub sku_id: Option<SnowflakeType>,
    // Optional TODO
    pub custom_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCommunicationDisabledActionMetadataResponse {
    pub duration_seconds: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionIntegerChoice {
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: Int53Type,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageResponseStickers {
    // type = 2
    GuildStickerResponse(GuildStickerResponse),
    // type = 1
    StandardStickerResponse(StandardStickerResponse),
}
impl<'de> Deserialize<'de> for MessageResponseStickers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            2 => Ok(MessageResponseStickers::GuildStickerResponse(
                GuildStickerResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            1 => Ok(MessageResponseStickers::StandardStickerResponse(
                StandardStickerResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["2", "1"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageResponseNonce {
    Integer(i64),
    String(String),
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageResponseComponents {
    // type = 1
    MessageComponentActionRowResponse(MessageComponentActionRowResponse),
    // type = 2
    MessageComponentButtonResponse(MessageComponentButtonResponse),
    // type = 8
    MessageComponentChannelSelectResponse(MessageComponentChannelSelectResponse),
    // type = 4
    MessageComponentInputTextResponse(MessageComponentInputTextResponse),
    // type = 7
    MessageComponentMentionableSelectResponse(MessageComponentMentionableSelectResponse),
    // type = 6
    MessageComponentRoleSelectResponse(MessageComponentRoleSelectResponse),
    // type = 3
    MessageComponentStringSelectResponse(MessageComponentStringSelectResponse),
    // type = 5
    MessageComponentUserSelectResponse(MessageComponentUserSelectResponse),
}
impl<'de> Deserialize<'de> for MessageResponseComponents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(
                MessageResponseComponents::MessageComponentActionRowResponse(
                    MessageComponentActionRowResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            2 => Ok(MessageResponseComponents::MessageComponentButtonResponse(
                MessageComponentButtonResponse::deserialize(
                    ContentRefDeserializer::<D::Error>::new(&content),
                )?,
            )),
            8 => {
                Ok(
                    MessageResponseComponents::MessageComponentChannelSelectResponse(
                        MessageComponentChannelSelectResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            4 => Ok(
                MessageResponseComponents::MessageComponentInputTextResponse(
                    MessageComponentInputTextResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            7 => Ok(
                MessageResponseComponents::MessageComponentMentionableSelectResponse(
                    MessageComponentMentionableSelectResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            6 => Ok(
                MessageResponseComponents::MessageComponentRoleSelectResponse(
                    MessageComponentRoleSelectResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            3 => {
                Ok(
                    MessageResponseComponents::MessageComponentStringSelectResponse(
                        MessageComponentStringSelectResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            5 => Ok(
                MessageResponseComponents::MessageComponentUserSelectResponse(
                    MessageComponentUserSelectResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "8", "4", "7", "6", "3", "5"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageResponse {
    pub mentions: Vec<UserResponse>,
    // Optional TODO
    pub purchase_notification: Option<PurchaseNotificationResponse>,
    // Optional TODO
    pub thread: Option<ThreadResponse>,
    pub author: UserResponse,
    pub flags: i32,
    // Optional TODO
    pub webhook_id: Option<SnowflakeType>,
    pub pinned: bool,
    // Optional TODO
    pub application: Option<BasicApplicationResponse>,
    // Optional TODO
    pub edited_timestamp: Option<String>,
    // Optional TODO
    pub resolved: Option<ResolvedObjectsResponse>,
    pub tts: bool,
    // Optional TODO
    pub call: Option<MessageCallResponse>,
    // Optional TODO
    pub mention_channels: Option<Vec<Option<MessageMentionChannelResponse>>>,
    // Optional TODO
    pub stickers: Option<Vec<MessageResponseStickers>>,
    // Optional TODO
    pub activity: Option<MessageActivityResponse>,
    // Optional TODO
    pub sticker_items: Option<Vec<MessageStickerItemResponse>>,
    // Optional TODO
    pub message_reference: Option<MessageReferenceResponse>,
    // Optional TODO
    pub reactions: Option<Vec<MessageReactionResponse>>,
    pub content: String,
    pub embeds: Vec<MessageEmbedResponse>,
    pub mention_roles: Vec<SnowflakeType>,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub timestamp: String,
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    // Optional TODO
    pub interaction: Option<MessageInteractionResponse>,
    // Optional TODO
    pub nonce: Option<MessageResponseNonce>,
    // Optional TODO
    pub role_subscription_data: Option<MessageRoleSubscriptionDataResponse>,
    pub channel_id: SnowflakeType,
    // Optional TODO
    pub referenced_message: Option<BasicMessageResponse>,
    pub id: SnowflakeType,
    pub components: Vec<MessageResponseComponents>,
    pub mention_everyone: bool,
    pub attachments: Vec<MessageAttachmentResponse>,
    // Optional TODO
    pub position: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadResponse {
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub user_limit: Option<i32>,
    // Optional TODO
    pub rtc_region: Option<String>,
    // Optional TODO
    pub bitrate: Option<i32>,
    pub name: String,
    // Optional TODO
    pub last_message_id: Option<SnowflakeType>,
    pub owner_id: SnowflakeType,
    pub message_count: i32,
    // Optional TODO
    pub member: Option<ThreadMemberResponse>,
    // Optional TODO
    pub permissions: Option<String>,
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub thread_metadata: Option<ThreadMetadataResponse>,
    // Optional TODO
    pub last_pin_timestamp: Option<String>,
    // Optional TODO
    pub applied_tags: Option<Vec<SnowflakeType>>,
    #[serde(rename = "type")]
    pub kind: i32,
    pub id: SnowflakeType,
    pub member_count: i32,
    pub total_message_sent: i32,
    pub flags: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandUserOptionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub description: String,
    pub name: String,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandIntegerOption {
    // Optional TODO
    pub choices: Option<Vec<ApplicationCommandOptionIntegerChoice>>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub name: String,
    // Optional TODO
    pub min_value: Option<Int53Type>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub max_value: Option<Int53Type>,
    pub description: String,
    // Optional TODO
    pub autocomplete: Option<bool>,
    // Optional TODO
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSelect {
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub min_values: Option<i32>,
    // Optional TODO
    pub max_values: Option<i32>,
    // Optional TODO
    pub default_values: Option<Vec<ChannelSelectDefaultValue>>,
    // Optional TODO
    pub channel_types: Option<Vec<ChannelTypes>>,
    pub custom_id: String,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandIntegerOptionResponse {
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    pub name: String,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub max_value: Option<Int53Type>,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub min_value: Option<Int53Type>,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub autocomplete: Option<bool>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub choices: Option<Vec<ApplicationCommandOptionIntegerChoiceResponse>>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTemplateChannelResponse {
    pub template: String,
    // Optional TODO
    pub icon_emoji: Option<IconEmojiResponse>,
    // Optional TODO
    pub topic: Option<String>,
    pub user_limit: i32,
    // Optional TODO
    pub available_tags: Option<Vec<GuildTemplateChannelTags>>,
    pub nsfw: bool,
    pub rate_limit_per_user: i32,
    pub permission_overwrites: Vec<Option<ChannelPermissionOverwriteResponse>>,
    // Optional TODO
    pub id: Option<i32>,
    // Optional TODO
    pub default_thread_rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub default_forum_layout: Option<ForumLayout>,
    pub bitrate: i32,
    // Optional TODO
    pub theme_color: Option<i32>,
    // Optional TODO
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    // Optional TODO
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub default_sort_order: Option<ThreadSortOrder>,
    // Optional TODO
    pub position: Option<i32>,
    // Optional TODO
    pub default_reaction_emoji: Option<DefaultReactionEmojiResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentStringSelectResponse {
    pub custom_id: String,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub min_values: Option<i32>,
    pub id: SnowflakeType,
    // Optional TODO
    pub max_values: Option<i32>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub disabled: Option<bool>,
    // Optional TODO
    pub options: Option<Vec<Option<SelectOptionResponse>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectedAccountGuildResponse {
    // Optional TODO
    pub icon: Option<String>,
    pub name: String,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAvatarDecorationResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandBooleanOptionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub required: Option<bool>,
    pub description: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentChannelSelectResponse {
    // Optional TODO
    pub min_values: Option<i32>,
    // Optional TODO
    pub channel_types: Option<Vec<ChannelTypes>>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub max_values: Option<i32>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub disabled: Option<bool>,
    pub id: SnowflakeType,
    pub custom_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPermissionOverwriteResponse {
    #[serde(rename = "type")]
    pub kind: ChannelPermissionOverwrites,
    pub allow: String,
    pub id: SnowflakeType,
    pub deny: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VanityURLResponse {
    // Optional TODO
    pub error: Option<VanityURLErrorResponse>,
    pub uses: i32,
    // Optional TODO
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubAuthor {
    // Optional TODO
    pub username: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarantineUserActionMetadata {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingWebhookRequestPartial {
    // Optional TODO
    pub applied_tags: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub avatar_url: Option<String>,
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub content: Option<String>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub tts: Option<bool>,
    // Optional TODO
    pub thread_name: Option<String>,
    // Optional TODO
    pub username: Option<String>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupDMInviteResponse {
    pub code: String,
    // Optional TODO
    pub approximate_member_count: Option<i32>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    // Optional TODO
    pub max_age: Option<i32>,
    // Optional TODO
    pub created_at: Option<String>,
    // Optional TODO
    pub channel: Option<InviteChannelResponse>,
    // Optional TODO
    pub inviter: Option<UserResponse>,
    // Optional TODO
    pub expires_at: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandCreateRequestOptions {
    // type = 11
    ApplicationCommandAttachmentOption(ApplicationCommandAttachmentOption),
    // type = 5
    ApplicationCommandBooleanOption(ApplicationCommandBooleanOption),
    // type = 7
    ApplicationCommandChannelOption(ApplicationCommandChannelOption),
    // type = 4
    ApplicationCommandIntegerOption(ApplicationCommandIntegerOption),
    // type = 9
    ApplicationCommandMentionableOption(ApplicationCommandMentionableOption),
    // type = 10
    ApplicationCommandNumberOption(ApplicationCommandNumberOption),
    // type = 8
    ApplicationCommandRoleOption(ApplicationCommandRoleOption),
    // type = 3
    ApplicationCommandStringOption(ApplicationCommandStringOption),
    // type = 2
    ApplicationCommandSubcommandGroupOption(ApplicationCommandSubcommandGroupOption),
    // type = 1
    ApplicationCommandSubcommandOption(ApplicationCommandSubcommandOption),
    // type = 6
    ApplicationCommandUserOption(ApplicationCommandUserOption),
}
impl<'de> Deserialize<'de> for ApplicationCommandCreateRequestOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            11 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandAttachmentOption(
                    ApplicationCommandAttachmentOption::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            5 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandBooleanOption(
                    ApplicationCommandBooleanOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            7 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandChannelOption(
                    ApplicationCommandChannelOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            4 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandIntegerOption(
                    ApplicationCommandIntegerOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            9 => {
                Ok(
                    ApplicationCommandCreateRequestOptions::ApplicationCommandMentionableOption(
                        ApplicationCommandMentionableOption::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            10 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandNumberOption(
                    ApplicationCommandNumberOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            8 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandRoleOption(
                    ApplicationCommandRoleOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            3 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandStringOption(
                    ApplicationCommandStringOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            2 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandSubcommandGroupOption(
                    ApplicationCommandSubcommandGroupOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            1 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandSubcommandOption(
                    ApplicationCommandSubcommandOption::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            6 => Ok(
                ApplicationCommandCreateRequestOptions::ApplicationCommandUserOption(
                    ApplicationCommandUserOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["11", "5", "7", "4", "9", "10", "8", "3", "2", "1", "6"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandCreateRequest {
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandCreateRequestOptions>>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationCommandType>,
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub dm_permission: Option<bool>,
    // Optional TODO
    pub default_member_permissions: Option<i32>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeywordUpsertRequestActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for KeywordUpsertRequestActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(KeywordUpsertRequestActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(KeywordUpsertRequestActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(KeywordUpsertRequestActions::QuarantineUserAction(
                QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            3 => Ok(
                KeywordUpsertRequestActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeywordUpsertRequest {
    pub event_type: AutomodEventType,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    pub trigger_type: i32,
    pub name: String,
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub trigger_metadata: Option<KeywordTriggerMetadata>,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub actions: Option<Vec<KeywordUpsertRequestActions>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOptionResponse {
    pub label: String,
    pub value: String,
    // Optional TODO
    pub emoji: Option<MessageComponentEmojiResponse>,
    // Optional TODO
    pub default: Option<bool>,
    // Optional TODO
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ForumLayout {
    Default = 0,
    List = 1,
    Grid = 2,
}
impl From<ForumLayout> for i16 {
    fn from(v: ForumLayout) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ForumLayout {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Default),
            1 => Ok(Self::List),
            2 => Ok(Self::Grid),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteApplicationResponse {
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub install_params: Option<ApplicationOAuth2InstallParamsResponse>,
    // Optional TODO
    pub slug: Option<String>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    // Optional TODO
    pub bot_require_code_grant: Option<bool>,
    // Optional TODO
    pub rpc_origins: Option<Vec<Option<String>>>,
    // Optional TODO
    pub privacy_policy_url: Option<String>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationTypes>,
    // Optional TODO
    pub primary_sku_id: Option<SnowflakeType>,
    pub description: String,
    pub id: SnowflakeType,
    // Optional TODO
    pub max_participants: Option<i32>,
    pub name: String,
    // Optional TODO
    pub cover_image: Option<String>,
    // Optional TODO
    pub bot: Option<UserResponse>,
    // Optional TODO
    pub bot_public: Option<bool>,
    // Optional TODO
    pub terms_of_service_url: Option<String>,
    // Optional TODO
    pub custom_install_url: Option<String>,
    // Optional TODO
    pub tags: Option<Vec<String>>,
    pub verify_key: String,
    pub flags: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubWebhook {
    pub sender: GithubUser,
    // Optional TODO
    pub forced: Option<bool>,
    // Optional TODO
    pub commits: Option<Vec<GithubCommit>>,
    // Optional TODO
    pub discussion: Option<GithubDiscussion>,
    // Optional TODO
    pub action: Option<String>,
    // Optional TODO
    pub compare: Option<String>,
    // Optional TODO
    pub review: Option<GithubReview>,
    // Optional TODO
    #[serde(rename = "ref")]
    pub reference: Option<String>,
    // Optional TODO
    pub pull_request: Option<GithubIssue>,
    // Optional TODO
    pub forkee: Option<GithubRepository>,
    // Optional TODO
    pub check_run: Option<GithubCheckRun>,
    // Optional TODO
    pub issue: Option<GithubIssue>,
    // Optional TODO
    pub check_suite: Option<GithubCheckSuite>,
    // Optional TODO
    pub ref_type: Option<String>,
    // Optional TODO
    pub release: Option<GithubRelease>,
    // Optional TODO
    pub repository: Option<GithubRepository>,
    // Optional TODO
    pub member: Option<GithubUser>,
    // Optional TODO
    pub head_commit: Option<GithubCommit>,
    // Optional TODO
    pub answer: Option<GithubComment>,
    // Optional TODO
    pub comment: Option<GithubComment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsEmojiResponse {
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    // Optional TODO
    pub animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub id: SnowflakeType,
    pub flags: Int53Type,
    pub public_flags: i32,
    // Optional TODO
    pub bot: Option<bool>,
    // Optional TODO
    pub avatar: Option<String>,
    // Optional TODO
    pub system: Option<bool>,
    // Optional TODO
    pub banner: Option<String>,
    pub username: String,
    // Optional TODO
    pub accent_color: Option<i32>,
    // Optional TODO
    pub global_name: Option<String>,
    pub discriminator: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookSourceChannelResponse {
    pub name: String,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandSubcommandGroupOption {
    pub description: String,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandSubcommandOption>>,
    // Optional TODO
    pub required: Option<bool>,
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlackWebhook {
    // Optional TODO
    pub text: Option<String>,
    // Optional TODO
    pub icon_url: Option<String>,
    // Optional TODO
    pub username: Option<String>,
    // Optional TODO
    pub attachments: Option<Vec<WebhookSlackEmbed>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ThreadSortOrder {
    LatestActivity = 0,
    CreationDate = 1,
}
impl From<ThreadSortOrder> for i16 {
    fn from(v: ThreadSortOrder) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ThreadSortOrder {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::LatestActivity),
            1 => Ok(Self::CreationDate),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentEmojiResponse {
    pub name: String,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    // Optional TODO
    pub animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurchaseNotificationResponse {
    #[serde(rename = "type")]
    pub kind: PurchaseType,
    // Optional TODO
    pub guild_product_purchase: Option<GuildProductPurchaseResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum PremiumGuildTiers {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}
impl From<PremiumGuildTiers> for i16 {
    fn from(v: PremiumGuildTiers) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for PremiumGuildTiers {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier3),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicApplicationResponse {
    pub description: String,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub bot: Option<UserResponse>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationTypes>,
    pub name: String,
    // Optional TODO
    pub primary_sku_id: Option<SnowflakeType>,
    pub id: SnowflakeType,
    // Optional TODO
    pub cover_image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildWelcomeChannel {
    // Optional TODO
    pub emoji_name: Option<String>,
    pub channel_id: SnowflakeType,
    pub description: String,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AvailableLocalesEnum {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-419")]
    Es419,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sv-SE")]
    SvSe,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "zh-TW")]
    ZhTw,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyGuildResponse {
    // Optional TODO
    pub approximate_member_count: Option<i32>,
    pub owner: bool,
    pub permissions: String,
    // Optional TODO
    pub icon: Option<String>,
    pub features: Vec<GuildFeatures>,
    pub name: String,
    // Optional TODO
    pub approximate_presence_count: Option<i32>,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubIssue {
    pub title: String,
    pub number: i32,
    // Optional TODO
    pub body: Option<String>,
    pub id: i32,
    // Optional TODO
    pub pull_request: (),
    pub html_url: String,
    pub user: GithubUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum PurchaseType {
    GuildProduct = 0,
}
impl From<PurchaseType> for i16 {
    fn from(v: PurchaseType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for PurchaseType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::GuildProduct),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StandardStickerResponse {
    pub sort_value: i32,
    // Optional TODO
    pub format_type: Option<StickerFormatTypes>,
    pub name: String,
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: i32,
    pub pack_id: SnowflakeType,
    // Optional TODO
    pub description: Option<String>,
    pub tags: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringSelect {
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub min_values: Option<i32>,
    pub custom_id: String,
    // Optional TODO
    pub max_values: Option<i32>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub disabled: Option<bool>,
    pub options: Vec<SelectOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WidgetUserDiscriminator {
    #[serde(rename = "0000")]
    Zeroes,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandUpdateRequestOptions {
    // type = 11
    ApplicationCommandAttachmentOption(ApplicationCommandAttachmentOption),
    // type = 5
    ApplicationCommandBooleanOption(ApplicationCommandBooleanOption),
    // type = 7
    ApplicationCommandChannelOption(ApplicationCommandChannelOption),
    // type = 4
    ApplicationCommandIntegerOption(ApplicationCommandIntegerOption),
    // type = 9
    ApplicationCommandMentionableOption(ApplicationCommandMentionableOption),
    // type = 10
    ApplicationCommandNumberOption(ApplicationCommandNumberOption),
    // type = 8
    ApplicationCommandRoleOption(ApplicationCommandRoleOption),
    // type = 3
    ApplicationCommandStringOption(ApplicationCommandStringOption),
    // type = 2
    ApplicationCommandSubcommandGroupOption(ApplicationCommandSubcommandGroupOption),
    // type = 1
    ApplicationCommandSubcommandOption(ApplicationCommandSubcommandOption),
    // type = 6
    ApplicationCommandUserOption(ApplicationCommandUserOption),
}
impl<'de> Deserialize<'de> for ApplicationCommandUpdateRequestOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            11 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandAttachmentOption(
                    ApplicationCommandAttachmentOption::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            5 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandBooleanOption(
                    ApplicationCommandBooleanOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            7 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandChannelOption(
                    ApplicationCommandChannelOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            4 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandIntegerOption(
                    ApplicationCommandIntegerOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            9 => {
                Ok(
                    ApplicationCommandUpdateRequestOptions::ApplicationCommandMentionableOption(
                        ApplicationCommandMentionableOption::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            10 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandNumberOption(
                    ApplicationCommandNumberOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            8 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandRoleOption(
                    ApplicationCommandRoleOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            3 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandStringOption(
                    ApplicationCommandStringOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            2 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandSubcommandGroupOption(
                    ApplicationCommandSubcommandGroupOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            1 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandSubcommandOption(
                    ApplicationCommandSubcommandOption::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            6 => Ok(
                ApplicationCommandUpdateRequestOptions::ApplicationCommandUserOption(
                    ApplicationCommandUserOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["11", "5", "7", "4", "9", "10", "8", "3", "2", "1", "6"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandUpdateRequest {
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub default_member_permissions: Option<i32>,
    pub name: String,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandUpdateRequestOptions>>,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    // Optional TODO
    pub dm_permission: Option<bool>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationCommandType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandSubcommandGroupOptionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub name: String,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandSubcommandOptionResponse>>,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationOAuth2InstallParams {
    // Optional TODO
    pub scopes: Option<Vec<String>>,
    // Optional TODO
    pub permissions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubComment {
    // Optional TODO
    pub commit_id: Option<String>,
    pub html_url: String,
    pub id: i32,
    pub user: GithubUser,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ButtonStyleTypes {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
    Premium = 6,
}
impl From<ButtonStyleTypes> for i16 {
    fn from(v: ButtonStyleTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ButtonStyleTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Primary),
            2 => Ok(Self::Secondary),
            3 => Ok(Self::Success),
            4 => Ok(Self::Danger),
            5 => Ok(Self::Link),
            6 => Ok(Self::Premium),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmojiResponse {
    pub roles: Vec<SnowflakeType>,
    // Optional TODO
    pub user: Option<UserResponse>,
    pub require_colons: bool,
    pub name: String,
    pub id: SnowflakeType,
    pub animated: bool,
    pub available: bool,
    pub managed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionIntegerChoiceResponse {
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub name: String,
    pub value: Int53Type,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandAttachmentOptionResponse {
    pub name: String,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description_localized: Option<String>,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub name_localized: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationFormPartialDescription {
    pub default: String,
    // Optional TODO
    pub localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationFormPartial {
    // Optional TODO
    pub team_id: Option<SnowflakeType>,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub max_participants: Option<i32>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationTypes>,
    // Optional TODO
    pub tags: Option<Vec<String>>,
    // Optional TODO
    pub description: Option<ApplicationFormPartialDescription>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub custom_install_url: Option<String>,
    // Optional TODO
    pub install_params: Option<ApplicationOAuth2InstallParams>,
    // Optional TODO
    pub cover_image: Option<String>,
    // Optional TODO
    pub interactions_endpoint_url: Option<String>,
    // Optional TODO
    pub role_connections_verification_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTemplateSnapshotResponse {
    // Optional TODO
    pub system_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub afk_channel_id: Option<SnowflakeType>,
    pub name: String,
    pub system_channel_flags: i32,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub region: Option<String>,
    pub verification_level: VerificationLevels,
    pub roles: Vec<GuildTemplateRoleResponse>,
    pub afk_timeout: AfkTimeouts,
    pub preferred_locale: AvailableLocalesEnum,
    pub default_message_notifications: UserNotificationSettings,
    pub channels: Vec<GuildTemplateChannelResponse>,
    pub explicit_content_filter: GuildExplicitContentFilterTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AllowedMentionTypes {
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "roles")]
    Roles,
    #[serde(rename = "everyone")]
    Everyone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookSourceGuildResponse {
    pub name: String,
    pub id: SnowflakeType,
    // Optional TODO
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildMFALevel {
    None = 0,
    Elevated = 1,
}
impl From<GuildMFALevel> for i16 {
    fn from(v: GuildMFALevel) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildMFALevel {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Elevated),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleSelectDefaultValue {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ApplicationCommandPermissionType {
    Role = 1,
    User = 2,
    Channel = 3,
}
impl From<ApplicationCommandPermissionType> for i16 {
    fn from(v: ApplicationCommandPermissionType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ApplicationCommandPermissionType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Role),
            2 => Ok(Self::User),
            3 => Ok(Self::Channel),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildHomeSettingsResponse {
    // Optional TODO
    pub resource_channels: Option<Vec<Option<ResourceChannelResponse>>>,
    pub enabled: bool,
    // Optional TODO
    pub welcome_message: Option<WelcomeMessageResponse>,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub new_member_actions: Option<Vec<Option<NewMemberActionResponse>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionApplicationCommandAutocompleteCallbackStringData {
    // Optional TODO
    pub choices: Option<Vec<Option<ApplicationCommandOptionStringChoice>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedAuthor {
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEntitlementRequestData {
    pub sku_id: SnowflakeType,
    pub owner_id: SnowflakeType,
    pub owner_type: EntitlementOwnerTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUser {
    pub html_url: String,
    pub avatar_url: String,
    pub login: String,
    pub id: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum GuildAuditLogResponseGuildScheduledEvents {
    // entity_type = 3
    ExternalScheduledEventResponse(ExternalScheduledEventResponse),
    // entity_type = 1
    StageScheduledEventResponse(StageScheduledEventResponse),
    // entity_type = 2
    VoiceScheduledEventResponse(VoiceScheduledEventResponse),
}
impl<'de> Deserialize<'de> for GuildAuditLogResponseGuildScheduledEvents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "entity_type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            3 => Ok(
                GuildAuditLogResponseGuildScheduledEvents::ExternalScheduledEventResponse(
                    ExternalScheduledEventResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            1 => Ok(
                GuildAuditLogResponseGuildScheduledEvents::StageScheduledEventResponse(
                    StageScheduledEventResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            2 => Ok(
                GuildAuditLogResponseGuildScheduledEvents::VoiceScheduledEventResponse(
                    VoiceScheduledEventResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["3", "1", "2"],
            )),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum GuildAuditLogResponseIntegrations {
    // type = discord
    PartialDiscordIntegrationResponse(PartialDiscordIntegrationResponse),
    // type = twitch, youtube
    PartialExternalConnectionIntegrationResponse(PartialExternalConnectionIntegrationResponse),
    // type = guild_subscription
    PartialGuildSubscriptionIntegrationResponse(PartialGuildSubscriptionIntegrationResponse),
}
impl<'de> Deserialize<'de> for GuildAuditLogResponseIntegrations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = StringTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            "discord" => Ok(
                GuildAuditLogResponseIntegrations::PartialDiscordIntegrationResponse(
                    PartialDiscordIntegrationResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            "twitch" | "youtube" => Ok(
                GuildAuditLogResponseIntegrations::PartialExternalConnectionIntegrationResponse(
                    PartialExternalConnectionIntegrationResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            "guild_subscription" => Ok(
                GuildAuditLogResponseIntegrations::PartialGuildSubscriptionIntegrationResponse(
                    PartialGuildSubscriptionIntegrationResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["discord", "twitch", "youtube", "guild_subscription"],
            )),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum GuildAuditLogResponseAutoModerationRules {
    // trigger_type = 4
    DefaultKeywordRuleResponse(DefaultKeywordRuleResponse),
    // trigger_type = 1
    KeywordRuleResponse(KeywordRuleResponse),
    // trigger_type = 3
    MLSpamRuleResponse(MLSpamRuleResponse),
    // trigger_type = 5
    MentionSpamRuleResponse(MentionSpamRuleResponse),
    // trigger_type = 2
    SpamLinkRuleResponse(SpamLinkRuleResponse),
}
impl<'de> Deserialize<'de> for GuildAuditLogResponseAutoModerationRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "trigger_type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            4 => Ok(
                GuildAuditLogResponseAutoModerationRules::DefaultKeywordRuleResponse(
                    DefaultKeywordRuleResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            1 => Ok(
                GuildAuditLogResponseAutoModerationRules::KeywordRuleResponse(
                    KeywordRuleResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            3 => Ok(
                GuildAuditLogResponseAutoModerationRules::MLSpamRuleResponse(
                    MLSpamRuleResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            5 => Ok(
                GuildAuditLogResponseAutoModerationRules::MentionSpamRuleResponse(
                    MentionSpamRuleResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            2 => Ok(
                GuildAuditLogResponseAutoModerationRules::SpamLinkRuleResponse(
                    SpamLinkRuleResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["4", "1", "3", "5", "2"],
            )),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum GuildAuditLogResponseWebhooks {
    // type = 3
    ApplicationIncomingWebhookResponse(ApplicationIncomingWebhookResponse),
    // type = 2
    ChannelFollowerWebhookResponse(ChannelFollowerWebhookResponse),
    // type = 1
    GuildIncomingWebhookResponse(GuildIncomingWebhookResponse),
}
impl<'de> Deserialize<'de> for GuildAuditLogResponseWebhooks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            3 => Ok(
                GuildAuditLogResponseWebhooks::ApplicationIncomingWebhookResponse(
                    ApplicationIncomingWebhookResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            2 => Ok(
                GuildAuditLogResponseWebhooks::ChannelFollowerWebhookResponse(
                    ChannelFollowerWebhookResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            1 => Ok(GuildAuditLogResponseWebhooks::GuildIncomingWebhookResponse(
                GuildIncomingWebhookResponse::deserialize(
                    ContentRefDeserializer::<D::Error>::new(&content),
                )?,
            )),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["3", "2", "1"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildAuditLogResponse {
    pub application_commands: Vec<ApplicationCommandResponse>,
    pub guild_scheduled_events: Vec<GuildAuditLogResponseGuildScheduledEvents>,
    pub integrations: Vec<GuildAuditLogResponseIntegrations>,
    pub threads: Vec<ThreadResponse>,
    pub auto_moderation_rules: Vec<Option<GuildAuditLogResponseAutoModerationRules>>,
    pub users: Vec<UserResponse>,
    pub audit_log_entries: Vec<AuditLogEntryResponse>,
    pub webhooks: Vec<GuildAuditLogResponseWebhooks>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeywordTriggerMetadataResponse {
    pub keyword_filter: Vec<String>,
    pub allow_list: Vec<String>,
    pub regex_patterns: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedThreadResponse {
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    pub flags: i32,
    pub message_count: i32,
    // Optional TODO
    pub user_limit: Option<i32>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    pub owner_id: SnowflakeType,
    // Optional TODO
    pub last_pin_timestamp: Option<String>,
    pub id: SnowflakeType,
    // Optional TODO
    pub member: Option<ThreadMemberResponse>,
    pub name: String,
    // Optional TODO
    pub rtc_region: Option<String>,
    // Optional TODO
    pub bitrate: Option<i32>,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub thread_metadata: Option<ThreadMetadataResponse>,
    pub total_message_sent: i32,
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub permissions: Option<String>,
    pub member_count: i32,
    // Optional TODO
    pub applied_tags: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub last_message_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedProviderResponse {
    pub name: String,
    // Optional TODO
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSelect {
    #[serde(rename = "type")]
    pub kind: i32,
    pub custom_id: String,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub min_values: Option<i32>,
    // Optional TODO
    pub max_values: Option<i32>,
    // Optional TODO
    pub disabled: Option<bool>,
    // Optional TODO
    pub default_values: Option<Vec<UserSelectDefaultValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandNumberOption {
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub choices: Option<Vec<ApplicationCommandOptionNumberChoice>>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub min_value: Option<f64>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub max_value: Option<f64>,
    // Optional TODO
    pub autocomplete: Option<bool>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InnerErrors {
    pub _errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultKeywordListTriggerMetadata {
    // Optional TODO
    pub allow_list: Option<Vec<String>>,
    // Optional TODO
    pub presets: Option<Vec<AutomodKeywordPresetType>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DefaultKeywordListUpsertRequestPartialActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for DefaultKeywordListUpsertRequestPartialActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(
                DefaultKeywordListUpsertRequestPartialActions::BlockMessageAction(
                    BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            2 => Ok(
                DefaultKeywordListUpsertRequestPartialActions::FlagToChannelAction(
                    FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            4 => Ok(
                DefaultKeywordListUpsertRequestPartialActions::QuarantineUserAction(
                    QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            3 => Ok(
                DefaultKeywordListUpsertRequestPartialActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultKeywordListUpsertRequestPartial {
    // Optional TODO
    pub trigger_metadata: DefaultKeywordListTriggerMetadata,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub trigger_type: i32,
    // Optional TODO
    pub event_type: AutomodEventType,
    // Optional TODO
    pub actions: Option<Vec<DefaultKeywordListUpsertRequestPartialActions>>,
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalScheduledEventPatchRequestPartial {
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub scheduled_start_time: String,
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub entity_metadata: EntityMetadataExternal,
    // Optional TODO
    pub image: Option<String>,
    // Optional TODO
    pub entity_type: Option<i32>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub status: Option<GuildScheduledEventStatuses>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGuildInviteRequest {
    // Optional TODO
    pub unique: Option<bool>,
    // Optional TODO
    pub target_type: Option<i32>,
    // Optional TODO
    pub max_age: Option<i32>,
    // Optional TODO
    pub temporary: Option<bool>,
    // Optional TODO
    pub target_application_id: Option<SnowflakeType>,
    // Optional TODO
    pub target_user_id: Option<SnowflakeType>,
    // Optional TODO
    pub max_uses: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentUserSelectResponse {
    pub custom_id: String,
    // Optional TODO
    pub max_values: Option<i32>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub min_values: Option<i32>,
    // Optional TODO
    pub disabled: Option<bool>,
    #[serde(rename = "type")]
    pub kind: i32,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteChannelResponse {
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub recipients: Option<Vec<InviteChannelRecipientResponse>>,
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: ChannelTypes,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandRoleOptionResponse {
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub name_localized: Option<String>,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookSlackEmbed {
    // Optional TODO
    pub footer_icon: Option<String>,
    // Optional TODO
    pub author_name: Option<String>,
    // Optional TODO
    pub title: Option<String>,
    // Optional TODO
    pub image_url: Option<String>,
    // Optional TODO
    pub fields: Option<Vec<WebhookSlackEmbedField>>,
    // Optional TODO
    pub thumb_url: Option<String>,
    // Optional TODO
    pub ts: Option<i32>,
    // Optional TODO
    pub title_link: Option<String>,
    // Optional TODO
    pub text: Option<String>,
    // Optional TODO
    pub color: Option<String>,
    // Optional TODO
    pub pretext: Option<String>,
    // Optional TODO
    pub author_link: Option<String>,
    // Optional TODO
    pub author_icon: Option<String>,
    // Optional TODO
    pub footer: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ModalInteractionCallbackDataComponents {
    // type = 1
    ActionRow(ActionRow),
    // type = 4
    InputText(InputText),
}
impl<'de> Deserialize<'de> for ModalInteractionCallbackDataComponents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(ModalInteractionCallbackDataComponents::ActionRow(
                ActionRow::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            4 => Ok(ModalInteractionCallbackDataComponents::InputText(
                InputText::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "4"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModalInteractionCallbackData {
    pub components: Vec<ModalInteractionCallbackDataComponents>,
    pub custom_id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLogObjectChangeResponse {
    // Optional TODO
    pub old_value: (),
    // Optional TODO
    pub key: Option<String>,
    // Optional TODO
    pub new_value: (),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateChannelResponse {
    pub flags: i32,
    #[serde(rename = "type")]
    pub kind: i32,
    pub recipients: Vec<UserResponse>,
    // Optional TODO
    pub last_pin_timestamp: Option<String>,
    // Optional TODO
    pub last_message_id: Option<SnowflakeType>,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReactionCountDetailsResponse {
    pub normal: i32,
    pub burst: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ActionRowComponents {
    // type = 2
    Button(Button),
    // type = 8
    ChannelSelect(ChannelSelect),
    // type = 4
    InputText(InputText),
    // type = 7
    MentionableSelect(MentionableSelect),
    // type = 6
    RoleSelect(RoleSelect),
    // type = 3
    StringSelect(StringSelect),
    // type = 5
    UserSelect(UserSelect),
}
impl<'de> Deserialize<'de> for ActionRowComponents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            2 => Ok(ActionRowComponents::Button(Button::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            8 => Ok(ActionRowComponents::ChannelSelect(
                ChannelSelect::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            4 => Ok(ActionRowComponents::InputText(InputText::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            7 => Ok(ActionRowComponents::MentionableSelect(
                MentionableSelect::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            6 => Ok(ActionRowComponents::RoleSelect(RoleSelect::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            3 => Ok(ActionRowComponents::StringSelect(
                StringSelect::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            5 => Ok(ActionRowComponents::UserSelect(UserSelect::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["2", "8", "4", "7", "6", "3", "5"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionRow {
    pub components: Vec<ActionRowComponents>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateGroupChannelResponse {
    // Optional TODO
    pub icon: Option<String>,
    pub flags: i32,
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    pub recipients: Vec<UserResponse>,
    // Optional TODO
    pub managed: Option<bool>,
    // Optional TODO
    pub owner_id: Option<SnowflakeType>,
    pub id: SnowflakeType,
    // Optional TODO
    pub last_pin_timestamp: Option<String>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub last_message_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandStringOption {
    // Optional TODO
    pub required: Option<bool>,
    pub description: String,
    // Optional TODO
    pub autocomplete: Option<bool>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub name: String,
    // Optional TODO
    pub min_length: Option<i32>,
    // Optional TODO
    pub choices: Option<Vec<ApplicationCommandOptionStringChoice>>,
    // Optional TODO
    pub max_length: Option<i32>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandSubcommandOptionOptions {
    // type = 11
    ApplicationCommandAttachmentOption(ApplicationCommandAttachmentOption),
    // type = 5
    ApplicationCommandBooleanOption(ApplicationCommandBooleanOption),
    // type = 7
    ApplicationCommandChannelOption(ApplicationCommandChannelOption),
    // type = 4
    ApplicationCommandIntegerOption(ApplicationCommandIntegerOption),
    // type = 9
    ApplicationCommandMentionableOption(ApplicationCommandMentionableOption),
    // type = 10
    ApplicationCommandNumberOption(ApplicationCommandNumberOption),
    // type = 8
    ApplicationCommandRoleOption(ApplicationCommandRoleOption),
    // type = 3
    ApplicationCommandStringOption(ApplicationCommandStringOption),
    // type = 6
    ApplicationCommandUserOption(ApplicationCommandUserOption),
}
impl<'de> Deserialize<'de> for ApplicationCommandSubcommandOptionOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            11 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandAttachmentOption(
                    ApplicationCommandAttachmentOption::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            5 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandBooleanOption(
                    ApplicationCommandBooleanOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            7 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandChannelOption(
                    ApplicationCommandChannelOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            4 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandIntegerOption(
                    ApplicationCommandIntegerOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            9 => {
                Ok(
                    ApplicationCommandSubcommandOptionOptions::ApplicationCommandMentionableOption(
                        ApplicationCommandMentionableOption::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            10 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandNumberOption(
                    ApplicationCommandNumberOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            8 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandRoleOption(
                    ApplicationCommandRoleOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            3 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandStringOption(
                    ApplicationCommandStringOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            6 => Ok(
                ApplicationCommandSubcommandOptionOptions::ApplicationCommandUserOption(
                    ApplicationCommandUserOption::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["11", "5", "7", "4", "9", "10", "8", "3", "6"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandSubcommandOption {
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandSubcommandOptionOptions>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEditRequestPartial {
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
    // Optional TODO
    pub sticker_ids: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateThreadRequestPartial {
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub locked: Option<bool>,
    // Optional TODO
    pub user_limit: Option<i32>,
    // Optional TODO
    pub invitable: Option<bool>,
    // Optional TODO
    pub rtc_region: Option<String>,
    // Optional TODO
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub bitrate: Option<i32>,
    // Optional TODO
    pub applied_tags: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub archived: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePrivateChannelRequest {
    // Optional TODO
    pub nicks: Option<HashMap<String, Option<String>>>,
    // Optional TODO
    pub access_tokens: Option<Vec<String>>,
    // Optional TODO
    pub recipient_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum InviteTargetTypes {
    Stream = 1,
    EmbeddedApplication = 2,
    RoleSubscriptionsPurchase = 3,
}
impl From<InviteTargetTypes> for i16 {
    fn from(v: InviteTargetTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for InviteTargetTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Stream),
            2 => Ok(Self::EmbeddedApplication),
            3 => Ok(Self::RoleSubscriptionsPurchase),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceScheduledEventResponse {
    pub status: GuildScheduledEventStatuses,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub user_rsvp: Option<ScheduledEventUserResponse>,
    // Optional TODO
    pub creator: Option<UserResponse>,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub entity_id: Option<SnowflakeType>,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub user_count: Option<i32>,
    pub id: SnowflakeType,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub entity_metadata: Option<EntityMetadataVoiceResponse>,
    pub scheduled_start_time: String,
    // Optional TODO
    pub image: Option<String>,
    pub name: String,
    pub entity_type: i32,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub creator_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduledEventUserResponse {
    // Optional TODO
    pub user: Option<UserResponse>,
    // Optional TODO
    pub member: Option<GuildMemberResponse>,
    pub user_id: SnowflakeType,
    pub guild_scheduled_event_id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum WebhookTypes {
    GuildIncoming = 1,
    ChannelFollower = 2,
    ApplicationIncoming = 3,
}
impl From<WebhookTypes> for i16 {
    fn from(v: WebhookTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for WebhookTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::GuildIncoming),
            2 => Ok(Self::ChannelFollower),
            3 => Ok(Self::ApplicationIncoming),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationResponse {
    pub id: SnowflakeType,
    pub flags: i32,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub slug: Option<String>,
    pub description: String,
    // Optional TODO
    pub rpc_origins: Option<Vec<Option<String>>>,
    // Optional TODO
    pub privacy_policy_url: Option<String>,
    // Optional TODO
    pub max_participants: Option<i32>,
    pub name: String,
    // Optional TODO
    pub custom_install_url: Option<String>,
    // Optional TODO
    pub tags: Option<Vec<String>>,
    // Optional TODO
    pub bot_require_code_grant: Option<bool>,
    // Optional TODO
    pub bot: Option<UserResponse>,
    // Optional TODO
    pub cover_image: Option<String>,
    // Optional TODO
    pub install_params: Option<ApplicationOAuth2InstallParamsResponse>,
    // Optional TODO
    pub bot_public: Option<bool>,
    pub verify_key: String,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    // Optional TODO
    pub primary_sku_id: Option<SnowflakeType>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationTypes>,
    // Optional TODO
    pub terms_of_service_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectedAccountResponse {
    pub visibility: ConnectedAccountVisibility,
    // Optional TODO
    pub revoked: Option<bool>,
    pub id: String,
    pub two_way_link: bool,
    // Optional TODO
    pub integrations: Option<Vec<ConnectedAccountIntegrationResponse>>,
    pub show_activity: bool,
    #[serde(rename = "type")]
    pub kind: ConnectedAccountProviders,
    pub verified: bool,
    pub friend_sync: bool,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotAccountPatchRequest {
    // Optional TODO
    pub banner: Option<String>,
    // Optional TODO
    pub avatar: Option<String>,
    pub username: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DefaultKeywordRuleResponseActions {
    // type = 1
    BlockMessageActionResponse(BlockMessageActionResponse),
    // type = 2
    FlagToChannelActionResponse(FlagToChannelActionResponse),
    // type = 4
    QuarantineUserActionResponse(QuarantineUserActionResponse),
    // type = 3
    UserCommunicationDisabledActionResponse(UserCommunicationDisabledActionResponse),
}
impl<'de> Deserialize<'de> for DefaultKeywordRuleResponseActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(
                DefaultKeywordRuleResponseActions::BlockMessageActionResponse(
                    BlockMessageActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            2 => Ok(
                DefaultKeywordRuleResponseActions::FlagToChannelActionResponse(
                    FlagToChannelActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            4 => Ok(
                DefaultKeywordRuleResponseActions::QuarantineUserActionResponse(
                    QuarantineUserActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            3 => Ok(
                DefaultKeywordRuleResponseActions::UserCommunicationDisabledActionResponse(
                    UserCommunicationDisabledActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultKeywordRuleResponse {
    pub trigger_metadata: DefaultKeywordListTriggerMetadataResponse,
    pub event_type: AutomodEventType,
    pub trigger_type: i32,
    pub guild_id: SnowflakeType,
    pub name: String,
    pub actions: Vec<DefaultKeywordRuleResponseActions>,
    pub id: SnowflakeType,
    // Optional TODO
    pub enabled: Option<bool>,
    pub creator_id: SnowflakeType,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkBanUsersResponse {
    pub failed_users: Vec<SnowflakeType>,
    pub banned_users: Vec<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMFALevelResponse {
    pub level: GuildMFALevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildNSFWContentLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}
impl From<GuildNSFWContentLevel> for i16 {
    fn from(v: GuildNSFWContentLevel) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildNSFWContentLevel {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Default),
            1 => Ok(Self::Explicit),
            2 => Ok(Self::Safe),
            3 => Ok(Self::AgeRestricted),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPermissionOverwriteRequest {
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ChannelPermissionOverwrites>,
    pub id: SnowflakeType,
    // Optional TODO
    pub deny: Option<i32>,
    // Optional TODO
    pub allow: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildWithCountsResponse {
    pub name: String,
    // Optional TODO
    pub banner: Option<String>,
    // Optional TODO
    pub max_stage_video_channel_users: Option<i32>,
    // Optional TODO
    pub public_updates_channel_id: Option<SnowflakeType>,
    pub owner_id: SnowflakeType,
    // Optional TODO
    pub approximate_presence_count: Option<i32>,
    // Optional TODO
    pub widget_channel_id: Option<SnowflakeType>,
    pub stickers: Vec<GuildStickerResponse>,
    // Optional TODO
    pub afk_channel_id: Option<SnowflakeType>,
    pub widget_enabled: bool,
    pub premium_tier: PremiumGuildTiers,
    // Optional TODO
    pub max_presences: Option<i32>,
    pub id: SnowflakeType,
    pub premium_subscription_count: i32,
    pub nsfw: bool,
    // Optional TODO
    pub safety_alerts_channel_id: Option<SnowflakeType>,
    pub features: Vec<GuildFeatures>,
    // Optional TODO
    pub rules_channel_id: Option<SnowflakeType>,
    pub default_message_notifications: UserNotificationSettings,
    pub verification_level: VerificationLevels,
    // Optional TODO
    pub system_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub max_video_channel_users: Option<i32>,
    pub preferred_locale: AvailableLocalesEnum,
    // Optional TODO
    pub approximate_member_count: Option<i32>,
    pub region: String,
    // Optional TODO
    pub discovery_splash: Option<String>,
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    pub afk_timeout: AfkTimeouts,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub splash: Option<String>,
    pub roles: Vec<GuildRoleResponse>,
    // Optional TODO
    pub max_members: Option<i32>,
    // Optional TODO
    pub vanity_url_code: Option<String>,
    pub emojis: Vec<EmojiResponse>,
    pub premium_progress_bar_enabled: bool,
    // Optional TODO
    pub home_header: Option<String>,
    pub nsfw_level: GuildNSFWContentLevel,
    pub explicit_content_filter: GuildExplicitContentFilterTypes,
    pub system_channel_flags: i32,
    pub mfa_level: GuildMFALevel,
    // Optional TODO
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildWelcomeScreenResponse {
    // Optional TODO
    pub description: Option<String>,
    pub welcome_channels: Vec<GuildWelcomeScreenChannelResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCheckRun {
    // Optional TODO
    pub details_url: Option<String>,
    // Optional TODO
    pub conclusion: Option<String>,
    pub html_url: String,
    pub name: String,
    pub check_suite: GithubCheckSuite,
    // Optional TODO
    pub output: Option<GithubCheckRunOutput>,
    // Optional TODO
    pub pull_requests: Option<Vec<GithubCheckPullRequest>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BasicMessageResponseComponents {
    // type = 1
    MessageComponentActionRowResponse(MessageComponentActionRowResponse),
    // type = 2
    MessageComponentButtonResponse(MessageComponentButtonResponse),
    // type = 8
    MessageComponentChannelSelectResponse(MessageComponentChannelSelectResponse),
    // type = 4
    MessageComponentInputTextResponse(MessageComponentInputTextResponse),
    // type = 7
    MessageComponentMentionableSelectResponse(MessageComponentMentionableSelectResponse),
    // type = 6
    MessageComponentRoleSelectResponse(MessageComponentRoleSelectResponse),
    // type = 3
    MessageComponentStringSelectResponse(MessageComponentStringSelectResponse),
    // type = 5
    MessageComponentUserSelectResponse(MessageComponentUserSelectResponse),
}
impl<'de> Deserialize<'de> for BasicMessageResponseComponents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(
                BasicMessageResponseComponents::MessageComponentActionRowResponse(
                    MessageComponentActionRowResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            2 => Ok(
                BasicMessageResponseComponents::MessageComponentButtonResponse(
                    MessageComponentButtonResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            8 => {
                Ok(
                    BasicMessageResponseComponents::MessageComponentChannelSelectResponse(
                        MessageComponentChannelSelectResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            4 => Ok(
                BasicMessageResponseComponents::MessageComponentInputTextResponse(
                    MessageComponentInputTextResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            7 => Ok(
                BasicMessageResponseComponents::MessageComponentMentionableSelectResponse(
                    MessageComponentMentionableSelectResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            6 => Ok(
                BasicMessageResponseComponents::MessageComponentRoleSelectResponse(
                    MessageComponentRoleSelectResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),
            3 => {
                Ok(
                    BasicMessageResponseComponents::MessageComponentStringSelectResponse(
                        MessageComponentStringSelectResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            5 => Ok(
                BasicMessageResponseComponents::MessageComponentUserSelectResponse(
                    MessageComponentUserSelectResponse::deserialize(ContentRefDeserializer::<
                        D::Error,
                    >::new(
                        &content
                    ))?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "8", "4", "7", "6", "3", "5"],
            )),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BasicMessageResponseStickers {
    // type = 2
    GuildStickerResponse(GuildStickerResponse),
    // type = 1
    StandardStickerResponse(StandardStickerResponse),
}
impl<'de> Deserialize<'de> for BasicMessageResponseStickers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            2 => Ok(BasicMessageResponseStickers::GuildStickerResponse(
                GuildStickerResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            1 => Ok(BasicMessageResponseStickers::StandardStickerResponse(
                StandardStickerResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["2", "1"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BasicMessageResponseNonce {
    Integer(i64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicMessageResponse {
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    // Optional TODO
    pub webhook_id: Option<SnowflakeType>,
    pub flags: i32,
    pub attachments: Vec<MessageAttachmentResponse>,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub embeds: Vec<MessageEmbedResponse>,
    // Optional TODO
    pub interaction: Option<MessageInteractionResponse>,
    // Optional TODO
    pub resolved: Option<ResolvedObjectsResponse>,
    pub id: SnowflakeType,
    // Optional TODO
    pub mention_channels: Option<Vec<Option<MessageMentionChannelResponse>>>,
    pub channel_id: SnowflakeType,
    pub timestamp: String,
    pub components: Vec<BasicMessageResponseComponents>,
    pub mentions: Vec<UserResponse>,
    // Optional TODO
    pub purchase_notification: Option<PurchaseNotificationResponse>,
    // Optional TODO
    pub stickers: Option<Vec<BasicMessageResponseStickers>>,
    pub content: String,
    // Optional TODO
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub pinned: bool,
    // Optional TODO
    pub call: Option<MessageCallResponse>,
    pub mention_roles: Vec<SnowflakeType>,
    // Optional TODO
    pub position: Option<i32>,
    // Optional TODO
    pub application: Option<BasicApplicationResponse>,
    // Optional TODO
    pub nonce: Option<BasicMessageResponseNonce>,
    pub author: UserResponse,
    // Optional TODO
    pub activity: Option<MessageActivityResponse>,
    // Optional TODO
    pub thread: Option<ThreadResponse>,
    pub mention_everyone: bool,
    // Optional TODO
    pub sticker_items: Option<Vec<MessageStickerItemResponse>>,
    // Optional TODO
    pub message_reference: Option<MessageReferenceResponse>,
    // Optional TODO
    pub role_subscription_data: Option<MessageRoleSubscriptionDataResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentButtonResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub label: Option<String>,
    // Optional TODO
    pub disabled: Option<bool>,
    // Optional TODO
    pub emoji: Option<MessageComponentEmojiResponse>,
    // Optional TODO
    pub custom_id: Option<String>,
    pub id: SnowflakeType,
    pub style: ButtonStyleTypes,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub sku_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGuildRequestRoleItem {
    // Optional TODO
    pub permissions: Option<i32>,
    // Optional TODO
    pub hoist: Option<bool>,
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub color: Option<i32>,
    // Optional TODO
    pub mentionable: Option<bool>,
    // Optional TODO
    pub unicode_emoji: Option<String>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandUserOption {
    pub name: String,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrUpdateThreadTagRequest {
    // Optional TODO
    pub moderated: Option<bool>,
    pub name: String,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
    // Optional TODO
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReactionResponse {
    pub burst_colors: Vec<String>,
    pub count_details: MessageReactionCountDetailsResponse,
    pub emoji: MessageReactionEmojiResponse,
    pub me_burst: bool,
    pub count: i32,
    pub me: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityMetadataVoiceResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewMemberActionResponse {
    // Optional TODO
    pub emoji: Option<SettingsEmojiResponse>,
    // Optional TODO
    pub icon: Option<String>,
    pub description: String,
    pub channel_id: SnowflakeType,
    pub action_type: NewMemberActionType,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandStringOptionResponse {
    // Optional TODO
    pub max_length: Option<i32>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub min_length: Option<i32>,
    // Optional TODO
    pub name_localized: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub autocomplete: Option<bool>,
    // Optional TODO
    pub choices: Option<Vec<ApplicationCommandOptionStringChoiceResponse>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnboardingPromptOptionResponse {
    pub emoji: SettingsEmojiResponse,
    pub title: String,
    pub role_ids: Vec<SnowflakeType>,
    pub id: SnowflakeType,
    pub description: String,
    pub channel_ids: Vec<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResolvedObjectsResponseChannels {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResolvedObjectsResponse {
    pub members: HashMap<String, GuildMemberResponse>,
    pub users: HashMap<String, UserResponse>,
    pub channels: ResolvedObjectsResponseChannels,
    pub roles: HashMap<String, GuildRoleResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageScheduledEventCreateRequest {
    pub name: String,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    pub entity_type: i32,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub entity_metadata: Option<EntityMetadataStageInstance>,
    pub scheduled_start_time: String,
    // Optional TODO
    pub description: Option<String>,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamResponse {
    // Optional TODO
    pub icon: Option<String>,
    pub members: Vec<TeamMemberResponse>,
    pub id: SnowflakeType,
    pub owner_user_id: SnowflakeType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateGuildOnboardingRequest {
    // Optional TODO
    pub mode: Option<GuildOnboardingMode>,
    // Optional TODO
    pub prompts: Option<Vec<UpdateOnboardingPromptRequest>>,
    // Optional TODO
    pub default_channel_ids: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSelectDefaultValue {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGuildRequestChannelItem {
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    // Optional TODO
    pub nsfw: Option<bool>,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    // Optional TODO
    pub default_thread_rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub topic: Option<String>,
    // Optional TODO
    pub permission_overwrites: Option<Vec<ChannelPermissionOverwriteRequest>>,
    // Optional TODO
    pub user_limit: Option<i32>,
    // Optional TODO
    pub default_sort_order: Option<ThreadSortOrder>,
    // Optional TODO
    pub bitrate: Option<i32>,
    // Optional TODO
    pub rtc_region: Option<String>,
    pub name: String,
    // Optional TODO
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub default_reaction_emoji: Option<UpdateDefaultReactionEmojiRequest>,
    // Optional TODO
    pub available_tags: Option<Vec<CreateOrUpdateThreadTagRequest>>,
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    // Optional TODO
    pub position: Option<i32>,
    // Optional TODO
    pub default_forum_layout: Option<ForumLayout>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceScheduledEventCreateRequest {
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    pub name: String,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub image: Option<String>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub entity_metadata: Option<EntityMetadataVoice>,
    pub entity_type: i32,
    pub scheduled_start_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookSlackEmbedField {
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub inline: Option<bool>,
    // Optional TODO
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscordIntegrationResponse {
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub user: Option<UserResponse>,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub name: Option<String>,
    pub scopes: Vec<String>,
    // Optional TODO
    pub account: Option<AccountResponse>,
    pub application: IntegrationApplicationResponse,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbed {
    // Optional TODO
    pub thumbnail: Option<RichEmbedThumbnail>,
    // Optional TODO
    pub footer: Option<RichEmbedFooter>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<String>,
    // Optional TODO
    pub provider: Option<RichEmbedProvider>,
    // Optional TODO
    pub author: Option<RichEmbedAuthor>,
    // Optional TODO
    pub video: Option<RichEmbedVideo>,
    // Optional TODO
    pub fields: Option<Vec<RichEmbedField>>,
    // Optional TODO
    pub title: Option<String>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub color: Option<i32>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub image: Option<RichEmbedImage>,
    // Optional TODO
    pub timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum VideoQualityModes {
    Auto = 1,
    Full = 2,
}
impl From<VideoQualityModes> for i16 {
    fn from(v: VideoQualityModes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for VideoQualityModes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Auto),
            2 => Ok(Self::Full),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelFollowerWebhookResponse {
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    // Optional TODO
    pub source_guild: Option<WebhookSourceGuildResponse>,
    // Optional TODO
    pub avatar: Option<String>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    #[serde(rename = "type")]
    pub kind: i32,
    pub name: String,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub user: Option<UserResponse>,
    pub id: SnowflakeType,
    // Optional TODO
    pub source_channel: Option<WebhookSourceChannelResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildPreviewResponse {
    pub name: String,
    // Optional TODO
    pub icon: Option<String>,
    pub features: Vec<GuildFeatures>,
    pub approximate_member_count: i32,
    pub approximate_presence_count: i32,
    pub stickers: Vec<GuildStickerResponse>,
    // Optional TODO
    pub home_header: Option<String>,
    pub emojis: Vec<EmojiResponse>,
    // Optional TODO
    pub splash: Option<String>,
    // Optional TODO
    pub description: Option<String>,
    pub id: SnowflakeType,
    // Optional TODO
    pub discovery_splash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildScheduledEventStatuses {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}
impl From<GuildScheduledEventStatuses> for i16 {
    fn from(v: GuildScheduledEventStatuses) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildScheduledEventStatuses {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Scheduled),
            2 => Ok(Self::Active),
            3 => Ok(Self::Completed),
            4 => Ok(Self::Canceled),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalScheduledEventResponse {
    pub name: String,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    pub entity_metadata: EntityMetadataExternalResponse,
    pub guild_id: SnowflakeType,
    pub entity_type: i32,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub entity_id: Option<SnowflakeType>,
    pub id: SnowflakeType,
    // Optional TODO
    pub user_count: Option<i32>,
    // Optional TODO
    pub creator_id: Option<SnowflakeType>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    pub status: GuildScheduledEventStatuses,
    // Optional TODO
    pub user_rsvp: Option<ScheduledEventUserResponse>,
    // Optional TODO
    pub image: Option<String>,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub creator: Option<UserResponse>,
    pub scheduled_start_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum IntegrationExpireGracePeriodTypes {
    OneDay = 1,
    ThreeDays = 3,
    SevenDays = 7,
    FourteenDays = 14,
    ThirtyDays = 30,
}
impl From<IntegrationExpireGracePeriodTypes> for i16 {
    fn from(v: IntegrationExpireGracePeriodTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for IntegrationExpireGracePeriodTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::OneDay),
            3 => Ok(Self::ThreeDays),
            7 => Ok(Self::SevenDays),
            14 => Ok(Self::FourteenDays),
            30 => Ok(Self::ThirtyDays),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandPermissionResponse {
    pub permission: bool,
    #[serde(rename = "type")]
    pub kind: ApplicationCommandPermissionType,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityMetadataExternal {
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildPatchRequestPartial {
    // Optional TODO
    pub default_message_notifications: Option<UserNotificationSettings>,
    // Optional TODO
    pub banner: Option<String>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub discovery_splash: Option<String>,
    // Optional TODO
    pub afk_timeout: Option<AfkTimeouts>,
    // Optional TODO
    pub owner_id: SnowflakeType,
    // Optional TODO
    pub preferred_locale: Option<AvailableLocalesEnum>,
    // Optional TODO
    pub system_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub region: Option<String>,
    // Optional TODO
    pub features: Option<Vec<Option<String>>>,
    // Optional TODO
    pub splash: Option<String>,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub explicit_content_filter: Option<GuildExplicitContentFilterTypes>,
    // Optional TODO
    pub afk_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub rules_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub verification_level: Option<VerificationLevels>,
    // Optional TODO
    pub system_channel_flags: Option<i32>,
    // Optional TODO
    pub home_header: Option<String>,
    // Optional TODO
    pub safety_alerts_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub public_updates_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub premium_progress_bar_enabled: Option<bool>,
    // Optional TODO
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntegrationApplicationResponse {
    // Optional TODO
    pub icon: Option<String>,
    pub name: String,
    pub id: SnowflakeType,
    pub description: String,
    // Optional TODO
    pub primary_sku_id: Option<SnowflakeType>,
    // Optional TODO
    pub cover_image: Option<String>,
    // Optional TODO
    pub bot: Option<UserResponse>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationTypes>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTemplateRoleResponse {
    // Optional TODO
    pub unicode_emoji: Option<String>,
    pub color: i32,
    pub id: i32,
    // Optional TODO
    pub icon: Option<String>,
    pub permissions: String,
    pub name: String,
    pub mentionable: bool,
    pub hoist: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionApplicationCommandAutocompleteCallbackIntegerData {
    // Optional TODO
    pub choices: Option<Vec<Option<ApplicationCommandOptionIntegerChoice>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum VerificationLevels {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}
impl From<VerificationLevels> for i16 {
    fn from(v: VerificationLevels) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for VerificationLevels {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Low),
            2 => Ok(Self::Medium),
            3 => Ok(Self::High),
            4 => Ok(Self::VeryHigh),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetActivity {
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MLSpamUpsertRequestActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for MLSpamUpsertRequestActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(MLSpamUpsertRequestActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(MLSpamUpsertRequestActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(MLSpamUpsertRequestActions::QuarantineUserAction(
                QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            3 => Ok(MLSpamUpsertRequestActions::UserCommunicationDisabledAction(
                UserCommunicationDisabledAction::deserialize(
                    ContentRefDeserializer::<D::Error>::new(&content),
                )?,
            )),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MLSpamUpsertRequest {
    pub event_type: AutomodEventType,
    pub name: String,
    pub trigger_type: i32,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub actions: Option<Vec<MLSpamUpsertRequestActions>>,
    // Optional TODO
    pub trigger_metadata: Option<MLSpamTriggerMetadata>,
    // Optional TODO
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandMentionableOptionResponse {
    pub name: String,
    pub description: String,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub required: Option<bool>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandNumberOptionResponse {
    // Optional TODO
    pub autocomplete: Option<bool>,
    pub description: String,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub choices: Option<Vec<ApplicationCommandOptionNumberChoiceResponse>>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub max_value: Option<f64>,
    // Optional TODO
    pub min_value: Option<f64>,
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGuildChannelRequest {
    // Optional TODO
    pub topic: Option<String>,
    // Optional TODO
    pub nsfw: Option<bool>,
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    // Optional TODO
    pub default_sort_order: Option<ThreadSortOrder>,
    // Optional TODO
    pub available_tags: Option<Vec<Option<CreateOrUpdateThreadTagRequest>>>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub bitrate: Option<i32>,
    // Optional TODO
    pub position: Option<i32>,
    // Optional TODO
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub default_thread_rate_limit_per_user: Option<i32>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    // Optional TODO
    pub user_limit: Option<i32>,
    // Optional TODO
    pub default_reaction_emoji: Option<UpdateDefaultReactionEmojiRequest>,
    pub name: String,
    // Optional TODO
    pub rtc_region: Option<String>,
    // Optional TODO
    pub permission_overwrites: Option<Vec<ChannelPermissionOverwriteRequest>>,
    // Optional TODO
    pub default_forum_layout: Option<ForumLayout>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayBotSessionStartLimitResponse {
    pub max_concurrency: i32,
    pub total: i32,
    pub remaining: i32,
    pub reset_after: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuth2GetAuthorizationResponse {
    pub scopes: Vec<OAuth2Scopes>,
    pub application: ApplicationResponse,
    pub expires: String,
    // Optional TODO
    pub user: Option<UserResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendInviteResponse {
    // Optional TODO
    pub is_contact: Option<bool>,
    // Optional TODO
    pub friends_count: Option<i32>,
    // Optional TODO
    pub inviter: Option<UserResponse>,
    // Optional TODO
    pub created_at: Option<String>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub uses: Option<i32>,
    pub code: String,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    // Optional TODO
    pub expires_at: Option<String>,
    // Optional TODO
    pub channel: Option<InviteChannelResponse>,
    // Optional TODO
    pub max_uses: Option<i32>,
    // Optional TODO
    pub max_age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildStickerResponse {
    pub tags: String,
    #[serde(rename = "type")]
    pub kind: i32,
    pub name: String,
    // Optional TODO
    pub format_type: Option<StickerFormatTypes>,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub user: Option<UserResponse>,
    pub available: bool,
    pub id: SnowflakeType,
    // Optional TODO
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionSpamTriggerMetadataResponse {
    pub mention_total_limit: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReferenceRequest {
    pub message_id: SnowflakeType,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<MessageReferenceType>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub fail_if_not_exists: Option<bool>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateApplicationResponse {
    // Optional TODO
    pub interactions_endpoint_url: Option<String>,
    pub owner: UserResponse,
    // Optional TODO
    pub approximate_guild_count: Option<i32>,
    pub description: String,
    // Optional TODO
    pub slug: Option<String>,
    // Optional TODO
    pub tags: Option<Vec<String>>,
    pub flags: i32,
    // Optional TODO
    pub team: Option<TeamResponse>,
    // Optional TODO
    pub max_participants: Option<i32>,
    // Optional TODO
    pub primary_sku_id: Option<SnowflakeType>,
    // Optional TODO
    pub cover_image: Option<String>,
    // Optional TODO
    pub custom_install_url: Option<String>,
    // Optional TODO
    pub bot_require_code_grant: Option<bool>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    // Optional TODO
    pub role_connections_verification_url: Option<String>,
    // Optional TODO
    pub install_params: Option<ApplicationOAuth2InstallParamsResponse>,
    pub name: String,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<ApplicationTypes>,
    // Optional TODO
    pub bot_public: Option<bool>,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub rpc_origins: Option<Vec<Option<String>>>,
    // Optional TODO
    pub terms_of_service_url: Option<String>,
    // Optional TODO
    pub bot: Option<UserResponse>,
    pub id: SnowflakeType,
    // Optional TODO
    pub privacy_policy_url: Option<String>,
    pub verify_key: String,
    pub redirect_uris: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateChannelRequestPartial {
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDefaultReactionEmojiRequest {
    // Optional TODO
    pub emoji_name: Option<String>,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateThreadTagRequest {
    // Optional TODO
    pub moderated: Option<bool>,
    // Optional TODO
    pub emoji_name: Option<String>,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    pub name: String,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationUserRoleConnectionResponse {
    // Optional TODO
    pub platform_name: Option<String>,
    // Optional TODO
    pub platform_username: Option<String>,
    // Optional TODO
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PongInteractionCallbackRequest {
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCheckPullRequest {
    pub number: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ConnectedAccountVisibility {
    None = 0,
    Everyone = 1,
}
impl From<ConnectedAccountVisibility> for i16 {
    fn from(v: ConnectedAccountVisibility) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ConnectedAccountVisibility {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Everyone),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ApplicationTypes {
    GuildRoleSubscriptions = 4,
}
impl From<ApplicationTypes> for i16 {
    fn from(v: ApplicationTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ApplicationTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            4 => Ok(Self::GuildRoleSubscriptions),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityMetadataExternalResponse {
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum AfkTimeouts {
    OneMinute = 60,
    FiveMinutes = 300,
    FifteenMinutes = 900,
    ThirtyMinutes = 1800,
    OneHour = 3600,
}
impl From<AfkTimeouts> for i16 {
    fn from(v: AfkTimeouts) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for AfkTimeouts {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            60 => Ok(Self::OneMinute),
            300 => Ok(Self::FiveMinutes),
            900 => Ok(Self::FifteenMinutes),
            1800 => Ok(Self::ThirtyMinutes),
            3600 => Ok(Self::OneHour),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseCreateMessageCreateRequest {
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub content: Option<String>,
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub sticker_ids: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum AutomodKeywordPresetType {
    Profanity = 1,
    SexualContent = 2,
    Slurs = 3,
}
impl From<AutomodKeywordPresetType> for i16 {
    fn from(v: AutomodKeywordPresetType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for AutomodKeywordPresetType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Profanity),
            2 => Ok(Self::SexualContent),
            3 => Ok(Self::Slurs),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedFooterResponse {
    pub text: String,
    // Optional TODO
    pub icon_url: Option<String>,
    // Optional TODO
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedFooter {
    // Optional TODO
    pub text: Option<String>,
    // Optional TODO
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypingIndicatorResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum UserNotificationSettings {
    AllMessages = 0,
    OnlyMentions = 1,
}
impl From<UserNotificationSettings> for i16 {
    fn from(v: UserNotificationSettings) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for UserNotificationSettings {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::AllMessages),
            1 => Ok(Self::OnlyMentions),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetSettingsResponse {
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConnectedAccountProviders {
    #[serde(rename = "battlenet")]
    Battlenet,
    #[serde(rename = "bungie")]
    Bungie,
    #[serde(rename = "ebay")]
    Ebay,
    #[serde(rename = "epicgames")]
    EpicGames,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "leagueoflegends")]
    LeagueOfLegends,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "playstation")]
    Playstation,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "riotgames")]
    RiotGames,
    #[serde(rename = "roblox")]
    Roblox,
    #[serde(rename = "skype")]
    Skype,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "steam")]
    Steam,
    #[serde(rename = "tiktok")]
    Tiktok,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "xbox")]
    Xbox,
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "domain")]
    Domain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTextThreadWithMessageRequest {
    pub name: String,
    // Optional TODO
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlagToChannelActionMetadataResponse {
    pub channel_id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandBooleanOption {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModalInteractionCallbackRequest {
    pub data: ModalInteractionCallbackData,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTagResponse {
    pub moderated: bool,
    pub name: String,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
    // Optional TODO
    pub emoji_name: Option<String>,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCommunicationDisabledActionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub metadata: UserCommunicationDisabledActionMetadataResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationRoleConnectionsMetadataItemRequest {
    pub description: String,
    pub key: String,
    #[serde(rename = "type")]
    pub kind: MetadataItemTypes,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, Option<String>>>,
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionSpamTriggerMetadata {
    pub mention_total_limit: i32,
    // Optional TODO
    pub mention_raid_protection_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum NewMemberActionType {
    View = 0,
    Talk = 1,
}
impl From<NewMemberActionType> for i16 {
    fn from(v: NewMemberActionType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for NewMemberActionType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::View),
            1 => Ok(Self::Talk),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayBotResponse {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: GatewayBotSessionStartLimitResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildSubscriptionIntegrationResponse {
    // Optional TODO
    pub account: Option<AccountResponse>,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub enabled: Option<bool>,
    pub id: SnowflakeType,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentMentionableSelectResponse {
    pub id: SnowflakeType,
    // Optional TODO
    pub placeholder: Option<String>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub max_values: Option<i32>,
    // Optional TODO
    pub disabled: Option<bool>,
    // Optional TODO
    pub min_values: Option<i32>,
    pub custom_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockMessageActionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub metadata: BlockMessageActionMetadataResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    // Optional TODO
    pub emoji: Option<Emoji>,
    // Optional TODO
    pub default: Option<bool>,
    // Optional TODO
    pub description: Option<String>,
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerPackCollectionResponse {
    pub sticker_packs: Vec<StickerPackResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageCallResponse {
    pub participants: Vec<SnowflakeType>,
    // Optional TODO
    pub ended_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMetadataResponse {
    // Optional TODO
    pub invitable: Option<bool>,
    // Optional TODO
    pub create_timestamp: Option<String>,
    // Optional TODO
    pub archive_timestamp: Option<String>,
    pub auto_archive_duration: ThreadAutoArchiveDuration,
    pub locked: bool,
    pub archived: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorDetails {
    ErrorDetails(HashMap<String, ErrorDetails>),
    InnerErrors(InnerErrors),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceChannelResponse {
    pub channel_id: SnowflakeType,
    pub description: String,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub emoji: Option<SettingsEmojiResponse>,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityMetadataStageInstance {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildRoleResponse {
    pub id: SnowflakeType,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub description: Option<String>,
    pub managed: bool,
    pub hoist: bool,
    pub mentionable: bool,
    // Optional TODO
    pub tags: Option<GuildRoleTagsResponse>,
    pub permissions: String,
    pub name: String,
    pub position: i32,
    pub color: i32,
    // Optional TODO
    pub unicode_emoji: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteGuildResponse {
    // Optional TODO
    pub verification_level: Option<VerificationLevels>,
    pub id: SnowflakeType,
    pub name: String,
    // Optional TODO
    pub vanity_url_code: Option<String>,
    // Optional TODO
    pub nsfw: Option<bool>,
    // Optional TODO
    pub nsfw_level: Option<GuildNSFWContentLevel>,
    // Optional TODO
    pub splash: Option<String>,
    // Optional TODO
    pub banner: Option<String>,
    // Optional TODO
    pub premium_subscription_count: Option<i32>,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub description: Option<String>,
    pub features: Vec<GuildFeatures>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedField {
    pub value: String,
    pub name: String,
    // Optional TODO
    pub inline: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlagToChannelActionMetadata {
    pub channel_id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildIncomingWebhookResponse {
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub user: Option<UserResponse>,
    pub name: String,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    // Optional TODO
    pub avatar: Option<String>,
    pub id: SnowflakeType,
    // Optional TODO
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildInviteResponse {
    // Optional TODO
    pub stage_instance: Option<InviteStageInstanceResponse>,
    // Optional TODO
    pub max_uses: Option<i32>,
    pub code: String,
    // Optional TODO
    pub expires_at: Option<String>,
    // Optional TODO
    pub channel: Option<InviteChannelResponse>,
    // Optional TODO
    pub target_application: Option<InviteApplicationResponse>,
    // Optional TODO
    pub guild_scheduled_event: Option<ScheduledEventResponse>,
    // Optional TODO
    pub uses: Option<i32>,
    // Optional TODO
    pub temporary: Option<bool>,
    // Optional TODO
    pub approximate_presence_count: Option<i32>,
    // Optional TODO
    pub is_contact: Option<bool>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    // Optional TODO
    pub target_type: Option<InviteTargetTypes>,
    // Optional TODO
    pub approximate_member_count: Option<i32>,
    // Optional TODO
    pub created_at: Option<String>,
    // Optional TODO
    pub max_age: Option<i32>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub guild: Option<InviteGuildResponse>,
    // Optional TODO
    pub target_user: Option<UserResponse>,
    // Optional TODO
    pub inviter: Option<UserResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum PremiumTypes {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier0 = 3,
}
impl From<PremiumTypes> for i16 {
    fn from(v: PremiumTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for PremiumTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier0),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTextThreadWithoutMessageRequest {
    // Optional TODO
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    pub name: String,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    // Optional TODO
    pub invitable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityMetadataStageInstanceResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ApplicationCommandType {
    Chat = 1,
    User = 2,
    Message = 3,
}
impl From<ApplicationCommandType> for i16 {
    fn from(v: ApplicationCommandType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ApplicationCommandType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Chat),
            2 => Ok(Self::User),
            3 => Ok(Self::Message),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTemplateResponse {
    pub serialized_source_guild: GuildTemplateSnapshotResponse,
    pub code: String,
    pub name: String,
    pub usage_count: i32,
    // Optional TODO
    pub description: Option<String>,
    pub creator_id: SnowflakeType,
    // Optional TODO
    pub creator: Option<UserResponse>,
    pub updated_at: String,
    pub source_guild_id: SnowflakeType,
    // Optional TODO
    pub is_dirty: Option<bool>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeMessageResponse {
    pub author_ids: Vec<SnowflakeType>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildScheduledEventEntityTypes {
    None = 0,
    StageInstance = 1,
    Voice = 2,
    External = 3,
}
impl From<GuildScheduledEventEntityTypes> for i16 {
    fn from(v: GuildScheduledEventEntityTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildScheduledEventEntityTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::None),
            1 => Ok(Self::StageInstance),
            2 => Ok(Self::Voice),
            3 => Ok(Self::External),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum MessageComponentTypes {
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    InputText = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
}
impl From<MessageComponentTypes> for i16 {
    fn from(v: MessageComponentTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for MessageComponentTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::ActionRow),
            2 => Ok(Self::Button),
            3 => Ok(Self::StringSelect),
            4 => Ok(Self::InputText),
            5 => Ok(Self::UserSelect),
            6 => Ok(Self::RoleSelect),
            7 => Ok(Self::MentionableSelect),
            8 => Ok(Self::ChannelSelect),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandPermissionsResponse {
    pub id: SnowflakeType,
    pub guild_id: SnowflakeType,
    pub application_id: SnowflakeType,
    pub permissions: Vec<CommandPermissionResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingWebhookUpdateRequestPartial {
    // Optional TODO
    pub content: Option<String>,
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildChannelResponse {
    // Optional TODO
    pub last_message_id: Option<SnowflakeType>,
    // Optional TODO
    pub nsfw: Option<bool>,
    // Optional TODO
    pub permissions: Option<String>,
    // Optional TODO
    pub available_tags: Option<Vec<ForumTagResponse>>,
    // Optional TODO
    pub rtc_region: Option<String>,
    // Optional TODO
    pub topic: Option<String>,
    // Optional TODO
    pub user_limit: Option<i32>,
    pub id: SnowflakeType,
    // Optional TODO
    pub bitrate: Option<i32>,
    pub name: String,
    pub position: i32,
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub last_pin_timestamp: Option<String>,
    // Optional TODO
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub flags: i32,
    // Optional TODO
    pub default_reaction_emoji: Option<DefaultReactionEmojiResponse>,
    // Optional TODO
    pub default_sort_order: Option<ThreadSortOrder>,
    // Optional TODO
    pub default_forum_layout: Option<ForumLayout>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub permission_overwrites: Option<Vec<ChannelPermissionOverwriteResponse>>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeywordTriggerMetadata {
    // Optional TODO
    pub regex_patterns: Option<Vec<String>>,
    // Optional TODO
    pub allow_list: Option<Vec<String>>,
    // Optional TODO
    pub keyword_filter: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum AutomodTriggerType {
    Keyword = 1,
    SpamLink = 2,
    MlSpam = 3,
    DefaultKeywordList = 4,
    MentionSpam = 5,
}
impl From<AutomodTriggerType> for i16 {
    fn from(v: AutomodTriggerType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for AutomodTriggerType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Keyword),
            2 => Ok(Self::SpamLink),
            3 => Ok(Self::MlSpam),
            4 => Ok(Self::DefaultKeywordList),
            5 => Ok(Self::MentionSpam),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MentionableSelectDefaultValues {
    // type = role
    RoleSelectDefaultValue(RoleSelectDefaultValue),
    // type = user
    UserSelectDefaultValue(UserSelectDefaultValue),
}
impl<'de> Deserialize<'de> for MentionableSelectDefaultValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = StringTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            "role" => Ok(MentionableSelectDefaultValues::RoleSelectDefaultValue(
                RoleSelectDefaultValue::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            "user" => Ok(MentionableSelectDefaultValues::UserSelectDefaultValue(
                UserSelectDefaultValue::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["role", "user"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionableSelect {
    // Optional TODO
    pub min_values: Option<i32>,
    pub custom_id: String,
    // Optional TODO
    pub disabled: Option<bool>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub default_values: Option<Vec<MentionableSelectDefaultValues>>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub max_values: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRoleSubscriptionDataResponse {
    pub role_subscription_listing_id: SnowflakeType,
    pub tier_name: String,
    pub is_renewal: bool,
    pub total_months_subscribed: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageStickerItemResponse {
    pub id: SnowflakeType,
    pub format_type: StickerFormatTypes,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelFollowerResponse {
    pub webhook_id: SnowflakeType,
    pub channel_id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageAttachmentResponse {
    pub filename: String,
    // Optional TODO
    pub clip_created_at: Option<String>,
    // Optional TODO
    pub title: Option<String>,
    // Optional TODO
    pub height: Option<i32>,
    // Optional TODO
    pub duration_secs: Option<f64>,
    // Optional TODO
    pub waveform: Option<String>,
    pub id: SnowflakeType,
    pub url: String,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub clip_participants: Option<Vec<UserResponse>>,
    pub proxy_url: String,
    // Optional TODO
    pub ephemeral: Option<bool>,
    // Optional TODO
    pub width: Option<i32>,
    // Optional TODO
    pub application: Option<ApplicationResponse>,
    pub size: i32,
    // Optional TODO
    pub content_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGuildOnboardingResponse {
    pub guild_id: SnowflakeType,
    pub enabled: bool,
    pub prompts: Vec<OnboardingPromptResponse>,
    pub default_channel_ids: Vec<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceScheduledEventPatchRequestPartial {
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub image: Option<String>,
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub status: Option<GuildScheduledEventStatuses>,
    // Optional TODO
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    // Optional TODO
    pub entity_type: Option<i32>,
    // Optional TODO
    pub entity_metadata: Option<EntityMetadataVoice>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub scheduled_start_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationOAuth2InstallParamsResponse {
    pub scopes: Vec<String>,
    pub permissions: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadsResponse {
    pub members: Vec<ThreadMemberResponse>,
    // Optional TODO
    pub has_more: Option<bool>,
    pub threads: Vec<ThreadResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildOnboardingMode {
    OnboardingDefault = 0,
    OnboardingAdvanced = 1,
}
impl From<GuildOnboardingMode> for i16 {
    fn from(v: GuildOnboardingMode) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildOnboardingMode {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::OnboardingDefault),
            1 => Ok(Self::OnboardingAdvanced),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentRoleSelectResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub id: SnowflakeType,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub min_values: Option<i32>,
    // Optional TODO
    pub max_values: Option<i32>,
    // Optional TODO
    pub disabled: Option<bool>,
    pub custom_id: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MentionSpamUpsertRequestActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for MentionSpamUpsertRequestActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(MentionSpamUpsertRequestActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(MentionSpamUpsertRequestActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(MentionSpamUpsertRequestActions::QuarantineUserAction(
                QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            3 => Ok(
                MentionSpamUpsertRequestActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionSpamUpsertRequest {
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub enabled: Option<bool>,
    pub event_type: AutomodEventType,
    pub name: String,
    // Optional TODO
    pub actions: Option<Vec<MentionSpamUpsertRequestActions>>,
    pub trigger_type: i32,
    // Optional TODO
    pub trigger_metadata: Option<MentionSpamTriggerMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandPermission {
    #[serde(rename = "type")]
    pub kind: ApplicationCommandPermissionType,
    pub permission: bool,
    pub id: SnowflakeType,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeywordUpsertRequestPartialActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for KeywordUpsertRequestPartialActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(KeywordUpsertRequestPartialActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(KeywordUpsertRequestPartialActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(KeywordUpsertRequestPartialActions::QuarantineUserAction(
                QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            3 => Ok(
                KeywordUpsertRequestPartialActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeywordUpsertRequestPartial {
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub actions: Option<Vec<KeywordUpsertRequestPartialActions>>,
    // Optional TODO
    pub trigger_metadata: Option<KeywordTriggerMetadata>,
    // Optional TODO
    pub trigger_type: i32,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub event_type: AutomodEventType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectedAccountIntegrationResponse {
    #[serde(rename = "type")]
    pub kind: IntegrationTypes,
    pub account: AccountResponse,
    pub guild: ConnectedAccountGuildResponse,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GuildFeatures {
    #[serde(rename = "ANIMATED_BANNER")]
    AnimatedBanner,
    #[serde(rename = "ANIMATED_ICON")]
    AnimatedIcon,
    #[serde(rename = "APPLICATION_COMMAND_PERMISSIONS_V2")]
    ApplicationCommandPermissionsV2,
    #[serde(rename = "AUTO_MODERATION")]
    AutoModeration,
    #[serde(rename = "BANNER")]
    Banner,
    #[serde(rename = "COMMUNITY")]
    Community,
    #[serde(rename = "CREATOR_MONETIZABLE_PROVISIONAL")]
    CreatorMonetizableProvisional,
    #[serde(rename = "CREATOR_STORE_PAGE")]
    CreatorStorePage,
    #[serde(rename = "DEVELOPER_SUPPORT_SERVER")]
    DeveloperSupportServer,
    #[serde(rename = "DISCOVERABLE")]
    Discoverable,
    #[serde(rename = "FEATURABLE")]
    Featurable,
    #[serde(rename = "INVITES_DISABLED")]
    InvitesDisabled,
    #[serde(rename = "INVITE_SPLASH")]
    InviteSplash,
    #[serde(rename = "MEMBER_VERIFICATION_GATE_ENABLED")]
    MemberVerificationGateEnabled,
    #[serde(rename = "MORE_STICKERS")]
    MoreStickers,
    #[serde(rename = "NEWS")]
    News,
    #[serde(rename = "PARTNERED")]
    Partnered,
    #[serde(rename = "PREVIEW_ENABLED")]
    PreviewEnabled,
    #[serde(rename = "RAID_ALERTS_DISABLED")]
    RaidAlertsDisabled,
    #[serde(rename = "ROLE_ICONS")]
    RoleIcons,
    #[serde(rename = "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE")]
    RoleSubscriptionsAvailableForPurchase,
    #[serde(rename = "ROLE_SUBSCRIPTIONS_ENABLED")]
    RoleSubscriptionsEnabled,
    #[serde(rename = "TICKETED_EVENTS_ENABLED")]
    TicketedEventsEnabled,
    #[serde(rename = "VANITY_URL")]
    VanityUrl,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "VIP_REGIONS")]
    VipRegions,
    #[serde(rename = "WELCOME_SCREEN_ENABLED")]
    WelcomeScreenEnabled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingWebhookInteractionRequest {
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub content: Option<String>,
    // Optional TODO
    pub tts: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationRoleConnectionsMetadataItemResponse {
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub name: String,
    pub description: String,
    pub key: String,
    #[serde(rename = "type")]
    pub kind: MetadataItemTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalScheduledEventCreateRequest {
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    pub scheduled_start_time: String,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
    pub entity_metadata: EntityMetadataExternal,
    pub entity_type: i32,
    // Optional TODO
    pub image: Option<String>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MLSpamTriggerMetadataResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCheckApp {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum OnboardingPromptType {
    MultipleChoice = 0,
    Dropdown = 1,
}
impl From<OnboardingPromptType> for i16 {
    fn from(v: OnboardingPromptType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for OnboardingPromptType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::MultipleChoice),
            1 => Ok(Self::Dropdown),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInteractionResponse {
    pub id: SnowflakeType,
    pub name: String,
    // Optional TODO
    pub user: Option<UserResponse>,
    // Optional TODO
    pub name_localized: Option<String>,
    #[serde(rename = "type")]
    pub kind: InteractionTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuth2GetKeys {
    pub keys: Vec<OAuth2Key>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    // Optional TODO
    pub id: Option<SnowflakeType>,
    pub name: String,
    // Optional TODO
    pub animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCheckSuite {
    // Optional TODO
    pub pull_requests: Option<Vec<GithubCheckPullRequest>>,
    pub app: GithubCheckApp,
    // Optional TODO
    pub conclusion: Option<String>,
    // Optional TODO
    pub head_branch: Option<String>,
    pub head_sha: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildBanResponse {
    // Optional TODO
    pub reason: Option<String>,
    pub user: UserResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildOnboardingResponse {
    pub prompts: Vec<OnboardingPromptResponse>,
    pub enabled: bool,
    pub guild_id: SnowflakeType,
    pub default_channel_ids: Vec<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildWelcomeScreenChannelResponse {
    pub channel_id: SnowflakeType,
    // Optional TODO
    pub emoji_name: Option<String>,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum InviteTypes {
    Guild = 0,
    GroupDm = 1,
    Friend = 2,
}
impl From<InviteTypes> for i16 {
    fn from(v: InviteTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for InviteTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Guild),
            1 => Ok(Self::GroupDm),
            2 => Ok(Self::Friend),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum StickerFormatTypes {
    Png = 1,
    Apng = 2,
    Lottie = 3,
    Gif = 4,
}
impl From<StickerFormatTypes> for i16 {
    fn from(v: StickerFormatTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for StickerFormatTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Png),
            2 => Ok(Self::Apng),
            3 => Ok(Self::Lottie),
            4 => Ok(Self::Gif),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}
impl From<ApplicationCommandOptionType> for i16 {
    fn from(v: ApplicationCommandOptionType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ApplicationCommandOptionType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::SubCommand),
            2 => Ok(Self::SubCommandGroup),
            3 => Ok(Self::String),
            4 => Ok(Self::Integer),
            5 => Ok(Self::Boolean),
            6 => Ok(Self::User),
            7 => Ok(Self::Channel),
            8 => Ok(Self::Role),
            9 => Ok(Self::Mentionable),
            10 => Ok(Self::Number),
            11 => Ok(Self::Attachment),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMemberResponse {
    // Optional TODO
    pub avatar_decoration_data: Option<UserAvatarDecorationResponse>,
    // Optional TODO
    pub nick: Option<String>,
    pub deaf: bool,
    pub joined_at: String,
    // Optional TODO
    pub premium_since: Option<String>,
    pub roles: Vec<SnowflakeType>,
    pub flags: i32,
    pub pending: bool,
    pub user: UserResponse,
    pub mute: bool,
    // Optional TODO
    pub avatar: Option<String>,
    // Optional TODO
    pub communication_disabled_until: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildRoleTagsResponse {
    // Optional TODO
    pub premium_subscriber: (),
    // Optional TODO
    pub bot_id: Option<SnowflakeType>,
    // Optional TODO
    pub integration_id: Option<SnowflakeType>,
    // Optional TODO
    pub guild_connections: (),
    // Optional TODO
    pub available_for_purchase: (),
    // Optional TODO
    pub subscription_listing_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IntegrationTypes {
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "youtube")]
    Youtube,
    #[serde(rename = "guild_subscription")]
    GuildSubscription,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteStageInstanceResponse {
    // Optional TODO
    pub participant_count: Option<i32>,
    pub topic: String,
    // Optional TODO
    pub members: Option<Vec<GuildMemberResponse>>,
    // Optional TODO
    pub speaker_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum InteractionTypes {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}
impl From<InteractionTypes> for i16 {
    fn from(v: InteractionTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for InteractionTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Ping),
            2 => Ok(Self::ApplicationCommand),
            3 => Ok(Self::MessageComponent),
            4 => Ok(Self::ApplicationCommandAutocomplete),
            5 => Ok(Self::ModalSubmit),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageActivityResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionApplicationCommandAutocompleteCallbackNumberData {
    // Optional TODO
    pub choices: Option<Vec<Option<ApplicationCommandOptionNumberChoice>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnboardingPromptOptionRequest {
    pub title: String,
    // Optional TODO
    pub emoji_name: Option<String>,
    // Optional TODO
    pub emoji_animated: Option<bool>,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
    // Optional TODO
    pub role_ids: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub channel_ids: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionStringChoiceResponse {
    pub value: String,
    pub name: String,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedAuthorResponse {
    // Optional TODO
    pub proxy_icon_url: Option<String>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub icon_url: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedProvider {
    // Optional TODO
    pub name: Option<String>,
    // Optional TODO
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum StickerTypes {
    Standard = 1,
    Guild = 2,
}
impl From<StickerTypes> for i16 {
    fn from(v: StickerTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for StickerTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Standard),
            2 => Ok(Self::Guild),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum AutomodActionType {
    BlockMessage = 1,
    FlagToChannel = 2,
    UserCommunicationDisabled = 3,
    QuarantineUser = 4,
}
impl From<AutomodActionType> for i16 {
    fn from(v: AutomodActionType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for AutomodActionType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::BlockMessage),
            2 => Ok(Self::FlagToChannel),
            3 => Ok(Self::UserCommunicationDisabled),
            4 => Ok(Self::QuarantineUser),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UInt32Type(pub i64);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WidgetImageStyles {
    #[serde(rename = "shield")]
    Shield,
    #[serde(rename = "banner1")]
    Banner1,
    #[serde(rename = "banner2")]
    Banner2,
    #[serde(rename = "banner3")]
    Banner3,
    #[serde(rename = "banner4")]
    Banner4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandRoleOption {
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    pub name: String,
    pub description: String,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandPatchRequestPartialOptions {
    // type = 11
    ApplicationCommandAttachmentOption(ApplicationCommandAttachmentOption),
    // type = 5
    ApplicationCommandBooleanOption(ApplicationCommandBooleanOption),
    // type = 7
    ApplicationCommandChannelOption(ApplicationCommandChannelOption),
    // type = 4
    ApplicationCommandIntegerOption(ApplicationCommandIntegerOption),
    // type = 9
    ApplicationCommandMentionableOption(ApplicationCommandMentionableOption),
    // type = 10
    ApplicationCommandNumberOption(ApplicationCommandNumberOption),
    // type = 8
    ApplicationCommandRoleOption(ApplicationCommandRoleOption),
    // type = 3
    ApplicationCommandStringOption(ApplicationCommandStringOption),
    // type = 2
    ApplicationCommandSubcommandGroupOption(ApplicationCommandSubcommandGroupOption),
    // type = 1
    ApplicationCommandSubcommandOption(ApplicationCommandSubcommandOption),
    // type = 6
    ApplicationCommandUserOption(ApplicationCommandUserOption),
}
impl<'de> Deserialize<'de> for ApplicationCommandPatchRequestPartialOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            11 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandAttachmentOption(ApplicationCommandAttachmentOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            5 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandBooleanOption(ApplicationCommandBooleanOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            7 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandChannelOption(ApplicationCommandChannelOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            4 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandIntegerOption(ApplicationCommandIntegerOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            9 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandMentionableOption(ApplicationCommandMentionableOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            10 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandNumberOption(ApplicationCommandNumberOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            8 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandRoleOption(ApplicationCommandRoleOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            3 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandStringOption(ApplicationCommandStringOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            2 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandSubcommandGroupOption(ApplicationCommandSubcommandGroupOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            1 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandSubcommandOption(ApplicationCommandSubcommandOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            6 => Ok(ApplicationCommandPatchRequestPartialOptions::ApplicationCommandUserOption(ApplicationCommandUserOption::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["11", "5", "7", "4", "9", "10", "8", "3", "2", "1", "6"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandPatchRequestPartial {
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub default_member_permissions: Option<i32>,
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandPatchRequestPartialOptions>>,
    // Optional TODO
    pub dm_permission: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateForumThreadRequest {
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    pub name: String,
    // Optional TODO
    pub applied_tags: Option<Vec<SnowflakeType>>,
    pub message: BaseCreateMessageCreateRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetResponse {
    // Optional TODO
    pub instant_invite: Option<String>,
    pub name: String,
    pub channels: Vec<WidgetChannel>,
    pub id: SnowflakeType,
    pub presence_count: i32,
    pub members: Vec<WidgetMember>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum AutomodEventType {
    MessageSend = 1,
    GuildMemberJoinOrUpdate = 2,
}
impl From<AutomodEventType> for i16 {
    fn from(v: AutomodEventType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for AutomodEventType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::MessageSend),
            2 => Ok(Self::GuildMemberJoinOrUpdate),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTemplateChannelTags {
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
    pub name: String,
    // Optional TODO
    pub emoji_name: Option<String>,
    // Optional TODO
    pub moderated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum AuditLogActionTypes {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
    ApplicationCommandPermissionUpdate = 121,
    SoundboardSoundCreate = 130,
    SoundboardSoundUpdate = 131,
    SoundboardSoundDelete = 132,
    AutoModerationRuleCreate = 140,
    AutoModerationRuleUpdate = 141,
    AutoModerationRuleDelete = 142,
    AutoModerationBlockMessage = 143,
    AutoModerationFlagToChannel = 144,
    AutoModerationUserCommDisabled = 145,
    AutoModerationQuarantineUser = 146,
    CreatorMonetizationRequestCreated = 150,
    CreatorMonetizationTermsAccepted = 151,
    OnboardingPromptCreate = 163,
    OnboardingPromptUpdate = 164,
    OnboardingPromptDelete = 165,
    OnboardingCreate = 166,
    OnboardingUpdate = 167,
    GuildHomeFeatureItem = 171,
    GuildHomeRemoveItem = 172,
    HarmfulLinksBlockedMessage = 180,
    HomeSettingsCreate = 190,
    HomeSettingsUpdate = 191,
    VoiceChannelStatusCreate = 192,
    VoiceChannelStatusDelete = 193,
}
impl From<AuditLogActionTypes> for i16 {
    fn from(v: AuditLogActionTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for AuditLogActionTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::GuildUpdate),
            10 => Ok(Self::ChannelCreate),
            11 => Ok(Self::ChannelUpdate),
            12 => Ok(Self::ChannelDelete),
            13 => Ok(Self::ChannelOverwriteCreate),
            14 => Ok(Self::ChannelOverwriteUpdate),
            15 => Ok(Self::ChannelOverwriteDelete),
            20 => Ok(Self::MemberKick),
            21 => Ok(Self::MemberPrune),
            22 => Ok(Self::MemberBanAdd),
            23 => Ok(Self::MemberBanRemove),
            24 => Ok(Self::MemberUpdate),
            25 => Ok(Self::MemberRoleUpdate),
            26 => Ok(Self::MemberMove),
            27 => Ok(Self::MemberDisconnect),
            28 => Ok(Self::BotAdd),
            30 => Ok(Self::RoleCreate),
            31 => Ok(Self::RoleUpdate),
            32 => Ok(Self::RoleDelete),
            40 => Ok(Self::InviteCreate),
            41 => Ok(Self::InviteUpdate),
            42 => Ok(Self::InviteDelete),
            50 => Ok(Self::WebhookCreate),
            51 => Ok(Self::WebhookUpdate),
            52 => Ok(Self::WebhookDelete),
            60 => Ok(Self::EmojiCreate),
            61 => Ok(Self::EmojiUpdate),
            62 => Ok(Self::EmojiDelete),
            72 => Ok(Self::MessageDelete),
            73 => Ok(Self::MessageBulkDelete),
            74 => Ok(Self::MessagePin),
            75 => Ok(Self::MessageUnpin),
            80 => Ok(Self::IntegrationCreate),
            81 => Ok(Self::IntegrationUpdate),
            82 => Ok(Self::IntegrationDelete),
            83 => Ok(Self::StageInstanceCreate),
            84 => Ok(Self::StageInstanceUpdate),
            85 => Ok(Self::StageInstanceDelete),
            90 => Ok(Self::StickerCreate),
            91 => Ok(Self::StickerUpdate),
            92 => Ok(Self::StickerDelete),
            100 => Ok(Self::GuildScheduledEventCreate),
            101 => Ok(Self::GuildScheduledEventUpdate),
            102 => Ok(Self::GuildScheduledEventDelete),
            110 => Ok(Self::ThreadCreate),
            111 => Ok(Self::ThreadUpdate),
            112 => Ok(Self::ThreadDelete),
            121 => Ok(Self::ApplicationCommandPermissionUpdate),
            130 => Ok(Self::SoundboardSoundCreate),
            131 => Ok(Self::SoundboardSoundUpdate),
            132 => Ok(Self::SoundboardSoundDelete),
            140 => Ok(Self::AutoModerationRuleCreate),
            141 => Ok(Self::AutoModerationRuleUpdate),
            142 => Ok(Self::AutoModerationRuleDelete),
            143 => Ok(Self::AutoModerationBlockMessage),
            144 => Ok(Self::AutoModerationFlagToChannel),
            145 => Ok(Self::AutoModerationUserCommDisabled),
            146 => Ok(Self::AutoModerationQuarantineUser),
            150 => Ok(Self::CreatorMonetizationRequestCreated),
            151 => Ok(Self::CreatorMonetizationTermsAccepted),
            163 => Ok(Self::OnboardingPromptCreate),
            164 => Ok(Self::OnboardingPromptUpdate),
            165 => Ok(Self::OnboardingPromptDelete),
            166 => Ok(Self::OnboardingCreate),
            167 => Ok(Self::OnboardingUpdate),
            171 => Ok(Self::GuildHomeFeatureItem),
            172 => Ok(Self::GuildHomeRemoveItem),
            180 => Ok(Self::HarmfulLinksBlockedMessage),
            190 => Ok(Self::HomeSettingsCreate),
            191 => Ok(Self::HomeSettingsUpdate),
            192 => Ok(Self::VoiceChannelStatusCreate),
            193 => Ok(Self::VoiceChannelStatusDelete),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayResponse {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateGuildMemberResponse {
    pub joined_at: String,
    pub user: UserResponse,
    pub flags: i32,
    pub deaf: bool,
    // Optional TODO
    pub premium_since: Option<String>,
    // Optional TODO
    pub banner: Option<String>,
    pub roles: Vec<SnowflakeType>,
    pub mute: bool,
    // Optional TODO
    pub avatar: Option<String>,
    // Optional TODO
    pub nick: Option<String>,
    // Optional TODO
    pub communication_disabled_until: Option<String>,
    // Optional TODO
    pub avatar_decoration_data: Option<UserAvatarDecorationResponse>,
    pub pending: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OAuth2Scopes {
    #[serde(rename = "identify")]
    Identify,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "connections")]
    Connections,
    #[serde(rename = "guilds")]
    Guilds,
    #[serde(rename = "guilds.join")]
    GuildsJoin,
    #[serde(rename = "guilds.members.read")]
    GuildsMembersRead,
    #[serde(rename = "gdm.join")]
    GdmJoin,
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "rpc")]
    Rpc,
    #[serde(rename = "rpc.notifications.read")]
    RpcNotificationsRead,
    #[serde(rename = "rpc.voice.read")]
    RpcVoiceRead,
    #[serde(rename = "rpc.voice.write")]
    RpcVoiceWrite,
    #[serde(rename = "rpc.video.read")]
    RpcVideoRead,
    #[serde(rename = "rpc.video.write")]
    RpcVideoWrite,
    #[serde(rename = "rpc.screenshare.read")]
    RpcScreenshareRead,
    #[serde(rename = "rpc.screenshare.write")]
    RpcScreenshareWrite,
    #[serde(rename = "rpc.activities.write")]
    RpcActivitiesWrite,
    #[serde(rename = "webhook.incoming")]
    WebhookIncoming,
    #[serde(rename = "messages.read")]
    MessagesRead,
    #[serde(rename = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    #[serde(rename = "applications.builds.read")]
    ApplicationsBuildsRead,
    #[serde(rename = "applications.commands")]
    ApplicationsCommands,
    #[serde(rename = "applications.commands.permissions.update")]
    ApplicationsCommandsPermissionsUpdate,
    #[serde(rename = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    #[serde(rename = "applications.store.update")]
    ApplicationsStoreUpdate,
    #[serde(rename = "applications.entitlements")]
    ApplicationsEntitlements,
    #[serde(rename = "activities.read")]
    ActivitiesRead,
    #[serde(rename = "activities.write")]
    ActivitiesWrite,
    #[serde(rename = "relationships.read")]
    RelationshipsRead,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "dm_channels.read")]
    DmChannelsRead,
    #[serde(rename = "role_connections.write")]
    RoleConnectionsWrite,
    #[serde(rename = "openid")]
    Openid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetMember {
    pub status: String,
    // Optional TODO
    pub avatar: (),
    pub avatar_url: String,
    // Optional TODO
    pub activity: Option<WidgetActivity>,
    // Optional TODO
    pub deaf: Option<bool>,
    // Optional TODO
    pub suppress: Option<bool>,
    pub discriminator: WidgetUserDiscriminator,
    // Optional TODO
    pub self_mute: Option<bool>,
    pub username: String,
    pub id: String,
    // Optional TODO
    pub self_deaf: Option<bool>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub mute: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialExternalConnectionIntegrationResponse {
    // Optional TODO
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub account: Option<AccountResponse>,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApplicationCommandAutocompleteCallbackRequestData {
    InteractionApplicationCommandAutocompleteCallbackIntegerData(
        InteractionApplicationCommandAutocompleteCallbackIntegerData,
    ),
    InteractionApplicationCommandAutocompleteCallbackNumberData(
        InteractionApplicationCommandAutocompleteCallbackNumberData,
    ),
    InteractionApplicationCommandAutocompleteCallbackStringData(
        InteractionApplicationCommandAutocompleteCallbackStringData,
    ),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandAutocompleteCallbackRequest {
    #[serde(rename = "type")]
    pub kind: i32,
    pub data: ApplicationCommandAutocompleteCallbackRequestData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnboardingPromptResponse {
    pub single_select: bool,
    pub id: SnowflakeType,
    pub title: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub kind: OnboardingPromptType,
    pub in_onboarding: bool,
    pub options: Vec<OnboardingPromptOptionResponse>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DefaultKeywordListUpsertRequestActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for DefaultKeywordListUpsertRequestActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(DefaultKeywordListUpsertRequestActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(DefaultKeywordListUpsertRequestActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(
                DefaultKeywordListUpsertRequestActions::QuarantineUserAction(
                    QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            3 => Ok(
                DefaultKeywordListUpsertRequestActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultKeywordListUpsertRequest {
    pub event_type: AutomodEventType,
    // Optional TODO
    pub actions: Option<Vec<DefaultKeywordListUpsertRequestActions>>,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    pub name: String,
    // Optional TODO
    pub enabled: Option<bool>,
    pub trigger_type: i32,
    pub trigger_metadata: DefaultKeywordListTriggerMetadata,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SpamLinkRuleResponseActions {
    // type = 1
    BlockMessageActionResponse(BlockMessageActionResponse),
    // type = 2
    FlagToChannelActionResponse(FlagToChannelActionResponse),
    // type = 4
    QuarantineUserActionResponse(QuarantineUserActionResponse),
    // type = 3
    UserCommunicationDisabledActionResponse(UserCommunicationDisabledActionResponse),
}
impl<'de> Deserialize<'de> for SpamLinkRuleResponseActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(SpamLinkRuleResponseActions::BlockMessageActionResponse(
                BlockMessageActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            2 => Ok(SpamLinkRuleResponseActions::FlagToChannelActionResponse(
                FlagToChannelActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(SpamLinkRuleResponseActions::QuarantineUserActionResponse(
                QuarantineUserActionResponse::deserialize(
                    ContentRefDeserializer::<D::Error>::new(&content),
                )?,
            )),
            3 => Ok(
                SpamLinkRuleResponseActions::UserCommunicationDisabledActionResponse(
                    UserCommunicationDisabledActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpamLinkRuleResponse {
    pub trigger_metadata: SpamLinkTriggerMetadataResponse,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub enabled: Option<bool>,
    pub actions: Vec<SpamLinkRuleResponseActions>,
    pub creator_id: SnowflakeType,
    pub id: SnowflakeType,
    pub name: String,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    pub trigger_type: i32,
    pub event_type: AutomodEventType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarantineUserActionResponse {
    #[serde(rename = "type")]
    pub kind: i32,
    pub metadata: QuarantineUserActionMetadataResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageAttachmentRequest {
    // Optional TODO
    pub filename: Option<String>,
    // Optional TODO
    pub is_remix: Option<bool>,
    pub id: SnowflakeType,
    // Optional TODO
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCommunicationDisabledActionMetadata {
    // Optional TODO
    pub duration_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationIncomingWebhookResponse {
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    pub id: SnowflakeType,
    pub name: String,
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub avatar: Option<String>,
    // Optional TODO
    pub user: Option<UserResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPIIResponse {
    // Optional TODO
    pub system: Option<bool>,
    pub mfa_enabled: bool,
    // Optional TODO
    pub bot: Option<bool>,
    pub public_flags: i32,
    pub discriminator: String,
    // Optional TODO
    pub verified: Option<bool>,
    // Optional TODO
    pub global_name: Option<String>,
    pub flags: Int53Type,
    pub locale: AvailableLocalesEnum,
    // Optional TODO
    pub avatar: Option<String>,
    // Optional TODO
    pub email: Option<String>,
    // Optional TODO
    pub premium_type: Option<PremiumTypes>,
    pub id: SnowflakeType,
    // Optional TODO
    pub accent_color: Option<i32>,
    // Optional TODO
    pub banner: Option<String>,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedVideo {
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub width: Option<i32>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub height: Option<i32>,
    // Optional TODO
    pub placeholder_version: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum InteractionCallbackTypes {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}
impl From<InteractionCallbackTypes> for i16 {
    fn from(v: InteractionCallbackTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for InteractionCallbackTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Pong),
            4 => Ok(Self::ChannelMessageWithSource),
            5 => Ok(Self::DeferredChannelMessageWithSource),
            6 => Ok(Self::DeferredUpdateMessage),
            7 => Ok(Self::UpdateMessage),
            8 => Ok(Self::ApplicationCommandAutocompleteResult),
            9 => Ok(Self::Modal),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ChannelTypes {
    Dm = 1,
    GroupDm = 3,
    GuildText = 0,
    GuildVoice = 2,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}
impl From<ChannelTypes> for i16 {
    fn from(v: ChannelTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ChannelTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Dm),
            3 => Ok(Self::GroupDm),
            0 => Ok(Self::GuildText),
            2 => Ok(Self::GuildVoice),
            4 => Ok(Self::GuildCategory),
            5 => Ok(Self::GuildAnnouncement),
            10 => Ok(Self::AnnouncementThread),
            11 => Ok(Self::PublicThread),
            12 => Ok(Self::PrivateThread),
            13 => Ok(Self::GuildStageVoice),
            14 => Ok(Self::GuildDirectory),
            15 => Ok(Self::GuildForum),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubDiscussion {
    pub user: GithubUser,
    pub title: String,
    pub html_url: String,
    pub number: i32,
    // Optional TODO
    pub answer_html_url: Option<String>,
    // Optional TODO
    pub body: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MLSpamRuleResponseActions {
    // type = 1
    BlockMessageActionResponse(BlockMessageActionResponse),
    // type = 2
    FlagToChannelActionResponse(FlagToChannelActionResponse),
    // type = 4
    QuarantineUserActionResponse(QuarantineUserActionResponse),
    // type = 3
    UserCommunicationDisabledActionResponse(UserCommunicationDisabledActionResponse),
}
impl<'de> Deserialize<'de> for MLSpamRuleResponseActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(MLSpamRuleResponseActions::BlockMessageActionResponse(
                BlockMessageActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            2 => Ok(MLSpamRuleResponseActions::FlagToChannelActionResponse(
                FlagToChannelActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(MLSpamRuleResponseActions::QuarantineUserActionResponse(
                QuarantineUserActionResponse::deserialize(
                    ContentRefDeserializer::<D::Error>::new(&content),
                )?,
            )),
            3 => Ok(
                MLSpamRuleResponseActions::UserCommunicationDisabledActionResponse(
                    UserCommunicationDisabledActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MLSpamRuleResponse {
    pub guild_id: SnowflakeType,
    pub trigger_type: i32,
    // Optional TODO
    pub enabled: Option<bool>,
    pub actions: Vec<MLSpamRuleResponseActions>,
    pub id: SnowflakeType,
    pub creator_id: SnowflakeType,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    pub event_type: AutomodEventType,
    pub name: String,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    pub trigger_metadata: MLSpamTriggerMetadataResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultKeywordListTriggerMetadataResponse {
    pub allow_list: Vec<String>,
    pub presets: Vec<AutomodKeywordPresetType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleSelect {
    // Optional TODO
    pub default_values: Option<Vec<RoleSelectDefaultValue>>,
    // Optional TODO
    pub min_values: Option<i32>,
    // Optional TODO
    pub max_values: Option<i32>,
    pub custom_id: String,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReferenceResponse {
    // Optional TODO
    pub message_id: Option<SnowflakeType>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    pub channel_id: SnowflakeType,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<MessageReferenceType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarantineUserActionMetadataResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedVideoResponse {
    // Optional TODO
    pub width: Option<UInt32Type>,
    // Optional TODO
    pub proxy_url: Option<String>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub height: Option<UInt32Type>,
    // Optional TODO
    pub placeholder_version: Option<UInt32Type>,
    // Optional TODO
    pub placeholder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum TeamMembershipStates {
    Invited = 1,
    Accepted = 2,
}
impl From<TeamMembershipStates> for i16 {
    fn from(v: TeamMembershipStates) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for TeamMembershipStates {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Invited),
            2 => Ok(Self::Accepted),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandChannelOption {
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
    pub name: String,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub channel_types: Option<Vec<ChannelTypes>>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedImageResponse {
    // Optional TODO
    pub proxy_url: Option<String>,
    // Optional TODO
    pub height: Option<UInt32Type>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub width: Option<UInt32Type>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub placeholder_version: Option<UInt32Type>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntitlementOwnerTypes {}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageComponentActionRowResponseComponents {
    // type = 2
    MessageComponentButtonResponse(MessageComponentButtonResponse),
    // type = 8
    MessageComponentChannelSelectResponse(MessageComponentChannelSelectResponse),
    // type = 4
    MessageComponentInputTextResponse(MessageComponentInputTextResponse),
    // type = 7
    MessageComponentMentionableSelectResponse(MessageComponentMentionableSelectResponse),
    // type = 6
    MessageComponentRoleSelectResponse(MessageComponentRoleSelectResponse),
    // type = 3
    MessageComponentStringSelectResponse(MessageComponentStringSelectResponse),
    // type = 5
    MessageComponentUserSelectResponse(MessageComponentUserSelectResponse),
}
impl<'de> Deserialize<'de> for MessageComponentActionRowResponseComponents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            2 => Ok(MessageComponentActionRowResponseComponents::MessageComponentButtonResponse(MessageComponentButtonResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            8 => Ok(MessageComponentActionRowResponseComponents::MessageComponentChannelSelectResponse(MessageComponentChannelSelectResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            4 => Ok(MessageComponentActionRowResponseComponents::MessageComponentInputTextResponse(MessageComponentInputTextResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            7 => Ok(MessageComponentActionRowResponseComponents::MessageComponentMentionableSelectResponse(MessageComponentMentionableSelectResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            6 => Ok(MessageComponentActionRowResponseComponents::MessageComponentRoleSelectResponse(MessageComponentRoleSelectResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            3 => Ok(MessageComponentActionRowResponseComponents::MessageComponentStringSelectResponse(MessageComponentStringSelectResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            5 => Ok(MessageComponentActionRowResponseComponents::MessageComponentUserSelectResponse(MessageComponentUserSelectResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["2", "8", "4", "7", "6", "3", "5"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageComponentActionRowResponse {
    // Optional TODO
    pub components: Option<Vec<MessageComponentActionRowResponseComponents>>,
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildCreateRequest {
    // Optional TODO
    pub system_channel_flags: Option<i32>,
    pub name: String,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub verification_level: Option<VerificationLevels>,
    // Optional TODO
    pub default_message_notifications: Option<UserNotificationSettings>,
    // Optional TODO
    pub explicit_content_filter: Option<GuildExplicitContentFilterTypes>,
    // Optional TODO
    pub system_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub channels: Option<Vec<CreateGuildRequestChannelItem>>,
    // Optional TODO
    pub region: Option<String>,
    // Optional TODO
    pub roles: Option<Vec<CreateGuildRequestRoleItem>>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub preferred_locale: Option<AvailableLocalesEnum>,
    // Optional TODO
    pub afk_timeout: Option<AfkTimeouts>,
    // Optional TODO
    pub afk_channel_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SnowflakeSelectDefaultValueTypes {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "channel")]
    Channel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IconEmojiResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandMentionableOption {
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub kind: i32,
    pub description: String,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildPruneResponse {
    // Optional TODO
    pub pruned: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncomingWebhookUpdateForInteractionCallbackRequestPartial {
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
    // Optional TODO
    pub content: Option<String>,
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
    // Optional TODO
    pub flags: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteChannelRecipientResponse {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageCreateRequestNonce {
    Integer(i64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageCreateRequest {
    // Optional TODO
    pub sticker_ids: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub allowed_mentions: Option<MessageAllowedMentionsRequest>,
    // Optional TODO
    pub message_reference: Option<MessageReferenceRequest>,
    // Optional TODO
    pub embeds: Option<Vec<RichEmbed>>,
    // Optional TODO
    pub nonce: Option<MessageCreateRequestNonce>,
    // Optional TODO
    pub components: Option<Vec<ActionRow>>,
    // Optional TODO
    pub attachments: Option<Vec<MessageAttachmentRequest>>,
    // Optional TODO
    pub content: Option<String>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub tts: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputText {
    #[serde(rename = "type")]
    pub kind: i32,
    pub custom_id: String,
    pub style: TextStyleTypes,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub max_length: Option<i32>,
    // Optional TODO
    pub value: Option<String>,
    // Optional TODO
    pub min_length: Option<i32>,
    pub label: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeywordRuleResponseActions {
    // type = 1
    BlockMessageActionResponse(BlockMessageActionResponse),
    // type = 2
    FlagToChannelActionResponse(FlagToChannelActionResponse),
    // type = 4
    QuarantineUserActionResponse(QuarantineUserActionResponse),
    // type = 3
    UserCommunicationDisabledActionResponse(UserCommunicationDisabledActionResponse),
}
impl<'de> Deserialize<'de> for KeywordRuleResponseActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(KeywordRuleResponseActions::BlockMessageActionResponse(
                BlockMessageActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            2 => Ok(KeywordRuleResponseActions::FlagToChannelActionResponse(
                FlagToChannelActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(KeywordRuleResponseActions::QuarantineUserActionResponse(
                QuarantineUserActionResponse::deserialize(
                    ContentRefDeserializer::<D::Error>::new(&content),
                )?,
            )),
            3 => Ok(
                KeywordRuleResponseActions::UserCommunicationDisabledActionResponse(
                    UserCommunicationDisabledActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeywordRuleResponse {
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    pub actions: Vec<KeywordRuleResponseActions>,
    pub trigger_metadata: KeywordTriggerMetadataResponse,
    // Optional TODO
    pub enabled: Option<bool>,
    pub name: String,
    pub trigger_type: i32,
    pub creator_id: SnowflakeType,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    pub guild_id: SnowflakeType,
    pub event_type: AutomodEventType,
    pub id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionNumberChoice {
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountResponse {
    pub id: String,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedFieldResponse {
    pub value: String,
    pub inline: bool,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarantineUserAction {
    // Optional TODO
    pub metadata: Option<QuarantineUserActionMetadata>,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedThumbnail {
    // Optional TODO
    pub width: Option<i32>,
    // Optional TODO
    pub height: Option<i32>,
    // Optional TODO
    pub placeholder_version: Option<i32>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub placeholder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduledEventResponse {
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub creator: Option<UserResponse>,
    pub id: SnowflakeType,
    pub name: String,
    pub guild_id: SnowflakeType,
    // Optional TODO
    pub scheduled_end_time: Option<String>,
    // Optional TODO
    pub creator_id: Option<SnowflakeType>,
    pub scheduled_start_time: String,
    pub entity_type: GuildScheduledEventEntityTypes,
    // Optional TODO
    pub entity_id: Option<SnowflakeType>,
    // Optional TODO
    pub user_count: Option<i32>,
    // Optional TODO
    pub image: Option<String>,
    // Optional TODO
    pub user_rsvp: Option<ScheduledEventUserResponse>,
    // Optional TODO
    pub channel_id: Option<SnowflakeType>,
    pub status: GuildScheduledEventStatuses,
    pub privacy_level: GuildScheduledEventPrivacyLevels,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultReactionEmojiResponse {
    // Optional TODO
    pub emoji_id: Option<SnowflakeType>,
    // Optional TODO
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum TextStyleTypes {
    Short = 1,
    Paragraph = 2,
}
impl From<TextStyleTypes> for i16 {
    fn from(v: TextStyleTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for TextStyleTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Short),
            2 => Ok(Self::Paragraph),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ThreadAutoArchiveDuration {
    OneHour = 60,
    OneDay = 1440,
    ThreeDay = 4320,
    SevenDay = 10080,
}
impl From<ThreadAutoArchiveDuration> for i16 {
    fn from(v: ThreadAutoArchiveDuration) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ThreadAutoArchiveDuration {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            60 => Ok(Self::OneHour),
            1440 => Ok(Self::OneDay),
            4320 => Ok(Self::ThreeDay),
            10080 => Ok(Self::SevenDay),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum ChannelPermissionOverwrites {
    Role = 0,
    Member = 1,
}
impl From<ChannelPermissionOverwrites> for i16 {
    fn from(v: ChannelPermissionOverwrites) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for ChannelPermissionOverwrites {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Role),
            1 => Ok(Self::Member),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMemberResponse {
    pub flags: i32,
    // Optional TODO
    pub member: Option<GuildMemberResponse>,
    pub id: SnowflakeType,
    pub user_id: SnowflakeType,
    pub join_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateOnboardingPromptRequest {
    pub id: SnowflakeType,
    // Optional TODO
    pub single_select: Option<bool>,
    pub title: String,
    // Optional TODO
    pub in_onboarding: Option<bool>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<OnboardingPromptType>,
    pub options: Vec<OnboardingPromptOptionRequest>,
    // Optional TODO
    pub required: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeScreenPatchRequestPartial {
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub welcome_channels: Option<Vec<GuildWelcomeChannel>>,
    // Optional TODO
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichEmbedImage {
    // Optional TODO
    pub height: Option<i32>,
    // Optional TODO
    pub width: Option<i32>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub placeholder: Option<String>,
    // Optional TODO
    pub placeholder_version: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockMessageActionMetadata {
    // Optional TODO
    pub custom_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum EntitlementTypes {
    ApplicationSubscription = 8,
    QuestReward = 10,
}
impl From<EntitlementTypes> for i16 {
    fn from(v: EntitlementTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for EntitlementTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            8 => Ok(Self::ApplicationSubscription),
            10 => Ok(Self::QuestReward),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum MessageReferenceType {
    Default = 0,
}
impl From<MessageReferenceType> for i16 {
    fn from(v: MessageReferenceType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for MessageReferenceType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Default),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMemberResponse {
    pub user: UserResponse,
    pub membership_state: TeamMembershipStates,
    pub team_id: SnowflakeType,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MLSpamUpsertRequestPartialActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for MLSpamUpsertRequestPartialActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(MLSpamUpsertRequestPartialActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(MLSpamUpsertRequestPartialActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(MLSpamUpsertRequestPartialActions::QuarantineUserAction(
                QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            3 => Ok(
                MLSpamUpsertRequestPartialActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MLSpamUpsertRequestPartial {
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub trigger_type: i32,
    // Optional TODO
    pub trigger_metadata: Option<MLSpamTriggerMetadata>,
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub event_type: AutomodEventType,
    // Optional TODO
    pub actions: Option<Vec<MLSpamUpsertRequestPartialActions>>,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateGroupDMInviteRequest {
    // Optional TODO
    pub max_age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalConnectionIntegrationResponse {
    // Optional TODO
    pub synced_at: Option<String>,
    // Optional TODO
    pub enabled: Option<bool>,
    pub id: String,
    // Optional TODO
    pub expire_behavior: Option<IntegrationExpireBehaviorTypes>,
    // Optional TODO
    pub expire_grace_period: Option<IntegrationExpireGracePeriodTypes>,
    // Optional TODO
    pub subscriber_count: Option<i32>,
    // Optional TODO
    pub role_id: Option<SnowflakeType>,
    // Optional TODO
    pub syncing: Option<bool>,
    // Optional TODO
    pub account: Option<AccountResponse>,
    // Optional TODO
    pub enable_emoticons: Option<bool>,
    // Optional TODO
    pub name: Option<String>,
    pub user: UserResponse,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub revoked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum MetadataItemTypes {
    IntegerLessThanEqual = 1,
    IntegerGreaterThanEqual = 2,
    IntegerEqual = 3,
    IntegerNotEqual = 4,
    DatetimeLessThanEqual = 5,
    DatetimeGreaterThanEqual = 6,
    BooleanEqual = 7,
    BooleanNotEqual = 8,
}
impl From<MetadataItemTypes> for i16 {
    fn from(v: MetadataItemTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for MetadataItemTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::IntegerLessThanEqual),
            2 => Ok(Self::IntegerGreaterThanEqual),
            3 => Ok(Self::IntegerEqual),
            4 => Ok(Self::IntegerNotEqual),
            5 => Ok(Self::DatetimeLessThanEqual),
            6 => Ok(Self::DatetimeGreaterThanEqual),
            7 => Ok(Self::BooleanEqual),
            8 => Ok(Self::BooleanNotEqual),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRelease {
    pub id: i32,
    pub tag_name: String,
    pub html_url: String,
    pub author: GithubUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubCommit {
    pub url: String,
    pub id: String,
    pub message: String,
    pub author: GithubAuthor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityMetadataVoice {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReactionEmojiResponse {
    // Optional TODO
    pub animated: Option<bool>,
    // Optional TODO
    pub id: Option<SnowflakeType>,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageAllowedMentionsRequest {
    // Optional TODO
    pub roles: Option<Vec<Option<SnowflakeType>>>,
    // Optional TODO
    pub parse: Option<Vec<Option<AllowedMentionTypes>>>,
    // Optional TODO
    pub users: Option<Vec<Option<SnowflakeType>>>,
    // Optional TODO
    pub replied_user: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandSubcommandOptionResponseOptions {
    // type = 11
    ApplicationCommandAttachmentOptionResponse(ApplicationCommandAttachmentOptionResponse),
    // type = 5
    ApplicationCommandBooleanOptionResponse(ApplicationCommandBooleanOptionResponse),
    // type = 7
    ApplicationCommandChannelOptionResponse(ApplicationCommandChannelOptionResponse),
    // type = 4
    ApplicationCommandIntegerOptionResponse(ApplicationCommandIntegerOptionResponse),
    // type = 9
    ApplicationCommandMentionableOptionResponse(ApplicationCommandMentionableOptionResponse),
    // type = 10
    ApplicationCommandNumberOptionResponse(ApplicationCommandNumberOptionResponse),
    // type = 8
    ApplicationCommandRoleOptionResponse(ApplicationCommandRoleOptionResponse),
    // type = 3
    ApplicationCommandStringOptionResponse(ApplicationCommandStringOptionResponse),
    // type = 6
    ApplicationCommandUserOptionResponse(ApplicationCommandUserOptionResponse),
}
impl<'de> Deserialize<'de> for ApplicationCommandSubcommandOptionResponseOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            11 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandAttachmentOptionResponse(ApplicationCommandAttachmentOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            5 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandBooleanOptionResponse(ApplicationCommandBooleanOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            7 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandChannelOptionResponse(ApplicationCommandChannelOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            4 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandIntegerOptionResponse(ApplicationCommandIntegerOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            9 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandMentionableOptionResponse(ApplicationCommandMentionableOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            10 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandNumberOptionResponse(ApplicationCommandNumberOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            8 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandRoleOptionResponse(ApplicationCommandRoleOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            3 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandStringOptionResponse(ApplicationCommandStringOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),
            6 => Ok(ApplicationCommandSubcommandOptionResponseOptions::ApplicationCommandUserOptionResponse(ApplicationCommandUserOptionResponse::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            )?)),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["11", "5", "7", "4", "9", "10", "8", "3", "6"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandSubcommandOptionResponse {
    // Optional TODO
    pub required: Option<bool>,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description_localized: Option<String>,
    #[serde(rename = "type")]
    pub kind: i32,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandSubcommandOptionResponseOptions>>,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildProductPurchaseResponse {
    pub listing_id: SnowflakeType,
    pub product_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLogEntryResponse {
    pub id: SnowflakeType,
    // Optional TODO
    pub target_id: Option<SnowflakeType>,
    pub action_type: AuditLogActionTypes,
    // Optional TODO
    pub changes: Option<Vec<AuditLogObjectChangeResponse>>,
    // Optional TODO
    pub options: Option<HashMap<String, String>>,
    // Optional TODO
    pub reason: Option<String>,
    // Optional TODO
    pub user_id: Option<SnowflakeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageEmbedResponse {
    // Optional TODO
    pub author: Option<MessageEmbedAuthorResponse>,
    // Optional TODO
    pub timestamp: Option<String>,
    // Optional TODO
    pub provider: Option<MessageEmbedProviderResponse>,
    // Optional TODO
    pub image: Option<MessageEmbedImageResponse>,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub title: Option<String>,
    // Optional TODO
    pub footer: Option<MessageEmbedFooterResponse>,
    // Optional TODO
    pub url: Option<String>,
    // Optional TODO
    pub fields: Option<Vec<MessageEmbedFieldResponse>>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub thumbnail: Option<MessageEmbedImageResponse>,
    // Optional TODO
    pub video: Option<MessageEmbedVideoResponse>,
    // Optional TODO
    pub color: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandOptionStringChoice {
    pub value: String,
    pub name: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlagToChannelAction {
    pub metadata: FlagToChannelActionMetadata,
    #[serde(rename = "type")]
    pub kind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildResponse {
    pub features: Vec<GuildFeatures>,
    pub roles: Vec<GuildRoleResponse>,
    // Optional TODO
    pub max_members: Option<i32>,
    pub verification_level: VerificationLevels,
    pub owner_id: SnowflakeType,
    // Optional TODO
    pub application_id: Option<SnowflakeType>,
    pub default_message_notifications: UserNotificationSettings,
    pub nsfw_level: GuildNSFWContentLevel,
    pub premium_tier: PremiumGuildTiers,
    pub afk_timeout: AfkTimeouts,
    pub widget_enabled: bool,
    pub system_channel_flags: i32,
    // Optional TODO
    pub vanity_url_code: Option<String>,
    pub preferred_locale: AvailableLocalesEnum,
    pub name: String,
    // Optional TODO
    pub system_channel_id: Option<SnowflakeType>,
    pub nsfw: bool,
    // Optional TODO
    pub max_stage_video_channel_users: Option<i32>,
    pub mfa_level: GuildMFALevel,
    // Optional TODO
    pub widget_channel_id: Option<SnowflakeType>,
    pub region: String,
    // Optional TODO
    pub public_updates_channel_id: Option<SnowflakeType>,
    pub premium_subscription_count: i32,
    pub id: SnowflakeType,
    // Optional TODO
    pub home_header: Option<String>,
    // Optional TODO
    pub max_video_channel_users: Option<i32>,
    // Optional TODO
    pub max_presences: Option<i32>,
    // Optional TODO
    pub banner: Option<String>,
    // Optional TODO
    pub afk_channel_id: Option<SnowflakeType>,
    // Optional TODO
    pub safety_alerts_channel_id: Option<SnowflakeType>,
    pub stickers: Vec<GuildStickerResponse>,
    // Optional TODO
    pub description: Option<String>,
    // Optional TODO
    pub discovery_splash: Option<String>,
    pub explicit_content_filter: GuildExplicitContentFilterTypes,
    // Optional TODO
    pub rules_channel_id: Option<SnowflakeType>,
    pub premium_progress_bar_enabled: bool,
    pub emojis: Vec<EmojiResponse>,
    // Optional TODO
    pub icon: Option<String>,
    // Optional TODO
    pub splash: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MentionSpamUpsertRequestPartialActions {
    // type = 1
    BlockMessageAction(BlockMessageAction),
    // type = 2
    FlagToChannelAction(FlagToChannelAction),
    // type = 4
    QuarantineUserAction(QuarantineUserAction),
    // type = 3
    UserCommunicationDisabledAction(UserCommunicationDisabledAction),
}
impl<'de> Deserialize<'de> for MentionSpamUpsertRequestPartialActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(MentionSpamUpsertRequestPartialActions::BlockMessageAction(
                BlockMessageAction::deserialize(ContentRefDeserializer::<D::Error>::new(&content))?,
            )),
            2 => Ok(MentionSpamUpsertRequestPartialActions::FlagToChannelAction(
                FlagToChannelAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(
                MentionSpamUpsertRequestPartialActions::QuarantineUserAction(
                    QuarantineUserAction::deserialize(ContentRefDeserializer::<D::Error>::new(
                        &content,
                    ))?,
                ),
            ),
            3 => Ok(
                MentionSpamUpsertRequestPartialActions::UserCommunicationDisabledAction(
                    UserCommunicationDisabledAction::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionSpamUpsertRequestPartial {
    // Optional TODO
    pub event_type: AutomodEventType,
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub trigger_type: i32,
    // Optional TODO
    pub trigger_metadata: Option<MentionSpamTriggerMetadata>,
    // Optional TODO
    pub actions: Option<Vec<MentionSpamUpsertRequestPartialActions>>,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpamLinkTriggerMetadataResponse {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialDiscordIntegrationResponse {
    // Optional TODO
    pub account: Option<AccountResponse>,
    pub application_id: SnowflakeType,
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: String,
    // Optional TODO
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum IntegrationExpireBehaviorTypes {
    RemoveRole = 0,
    Kick = 1,
}
impl From<IntegrationExpireBehaviorTypes> for i16 {
    fn from(v: IntegrationExpireBehaviorTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for IntegrationExpireBehaviorTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::RemoveRole),
            1 => Ok(Self::Kick),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateGuildChannelRequestPartial {
    // Optional TODO
    pub video_quality_mode: Option<VideoQualityModes>,
    // Optional TODO
    pub rtc_region: Option<String>,
    // Optional TODO
    pub default_reaction_emoji: Option<UpdateDefaultReactionEmojiRequest>,
    // Optional TODO
    pub available_tags: Option<Vec<UpdateThreadTagRequest>>,
    // Optional TODO
    pub parent_id: Option<SnowflakeType>,
    // Optional TODO
    pub default_auto_archive_duration: Option<ThreadAutoArchiveDuration>,
    // Optional TODO
    pub default_forum_layout: Option<ForumLayout>,
    // Optional TODO
    #[serde(rename = "type")]
    pub kind: Option<i32>,
    // Optional TODO
    pub position: Option<i32>,
    // Optional TODO
    pub bitrate: Option<i32>,
    // Optional TODO
    pub rate_limit_per_user: Option<i32>,
    // Optional TODO
    pub nsfw: Option<bool>,
    // Optional TODO
    pub user_limit: Option<i32>,
    // Optional TODO
    pub flags: Option<i32>,
    // Optional TODO
    pub topic: Option<String>,
    // Optional TODO
    pub default_sort_order: Option<ThreadSortOrder>,
    // Optional TODO
    pub name: String,
    // Optional TODO
    pub permission_overwrites: Option<Vec<ChannelPermissionOverwriteRequest>>,
    // Optional TODO
    pub default_thread_rate_limit_per_user: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum GuildExplicitContentFilterTypes {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}
impl From<GuildExplicitContentFilterTypes> for i16 {
    fn from(v: GuildExplicitContentFilterTypes) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for GuildExplicitContentFilterTypes {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::MembersWithoutRoles),
            2 => Ok(Self::AllMembers),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCommunicationDisabledAction {
    #[serde(rename = "type")]
    pub kind: i32,
    pub metadata: UserCommunicationDisabledActionMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockMessageActionMetadataResponse {
    // Optional TODO
    pub custom_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnowflakeType(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRepository {
    pub html_url: String,
    pub id: i32,
    pub name: String,
    pub full_name: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MentionSpamRuleResponseActions {
    // type = 1
    BlockMessageActionResponse(BlockMessageActionResponse),
    // type = 2
    FlagToChannelActionResponse(FlagToChannelActionResponse),
    // type = 4
    QuarantineUserActionResponse(QuarantineUserActionResponse),
    // type = 3
    UserCommunicationDisabledActionResponse(UserCommunicationDisabledActionResponse),
}
impl<'de> Deserialize<'de> for MentionSpamRuleResponseActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            1 => Ok(MentionSpamRuleResponseActions::BlockMessageActionResponse(
                BlockMessageActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            2 => Ok(MentionSpamRuleResponseActions::FlagToChannelActionResponse(
                FlagToChannelActionResponse::deserialize(ContentRefDeserializer::<D::Error>::new(
                    &content,
                ))?,
            )),
            4 => Ok(
                MentionSpamRuleResponseActions::QuarantineUserActionResponse(
                    QuarantineUserActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            3 => Ok(
                MentionSpamRuleResponseActions::UserCommunicationDisabledActionResponse(
                    UserCommunicationDisabledActionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["1", "2", "4", "3"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MentionSpamRuleResponse {
    pub guild_id: SnowflakeType,
    pub event_type: AutomodEventType,
    pub actions: Vec<MentionSpamRuleResponseActions>,
    pub id: SnowflakeType,
    // Optional TODO
    pub enabled: Option<bool>,
    // Optional TODO
    pub exempt_roles: Option<Vec<SnowflakeType>>,
    // Optional TODO
    pub exempt_channels: Option<Vec<SnowflakeType>>,
    pub trigger_type: i32,
    pub name: String,
    pub creator_id: SnowflakeType,
    pub trigger_metadata: MentionSpamTriggerMetadataResponse,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandResponseOptions {
    // type = 11
    ApplicationCommandAttachmentOptionResponse(ApplicationCommandAttachmentOptionResponse),
    // type = 5
    ApplicationCommandBooleanOptionResponse(ApplicationCommandBooleanOptionResponse),
    // type = 7
    ApplicationCommandChannelOptionResponse(ApplicationCommandChannelOptionResponse),
    // type = 4
    ApplicationCommandIntegerOptionResponse(ApplicationCommandIntegerOptionResponse),
    // type = 9
    ApplicationCommandMentionableOptionResponse(ApplicationCommandMentionableOptionResponse),
    // type = 10
    ApplicationCommandNumberOptionResponse(ApplicationCommandNumberOptionResponse),
    // type = 8
    ApplicationCommandRoleOptionResponse(ApplicationCommandRoleOptionResponse),
    // type = 3
    ApplicationCommandStringOptionResponse(ApplicationCommandStringOptionResponse),
    // type = 2
    ApplicationCommandSubcommandGroupOptionResponse(
        ApplicationCommandSubcommandGroupOptionResponse,
    ),
    // type = 1
    ApplicationCommandSubcommandOptionResponse(ApplicationCommandSubcommandOptionResponse),
    // type = 6
    ApplicationCommandUserOptionResponse(ApplicationCommandUserOptionResponse),
}
impl<'de> Deserialize<'de> for ApplicationCommandResponseOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Were not really supposed to use these "private" items but the alternative is copying massive amounts of code for this
        // i'd rather just use this for now
        use serde::__private::de::{Content, ContentRefDeserializer};

        let content = Content::deserialize(deserializer)?;

        let deserializer_ref = ContentRefDeserializer::<D::Error>::new(&content);

        let visitor = IntTagVisitor {
            field_name: "type".to_string(),
        };
        let tag = deserializer_ref.deserialize_any(visitor)?;

        match tag {
            11 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandAttachmentOptionResponse(
                    ApplicationCommandAttachmentOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            5 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandBooleanOptionResponse(
                    ApplicationCommandBooleanOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            7 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandChannelOptionResponse(
                    ApplicationCommandChannelOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            4 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandIntegerOptionResponse(
                    ApplicationCommandIntegerOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            9 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandMentionableOptionResponse(
                    ApplicationCommandMentionableOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            10 => {
                Ok(
                    ApplicationCommandResponseOptions::ApplicationCommandNumberOptionResponse(
                        ApplicationCommandNumberOptionResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            8 => {
                Ok(
                    ApplicationCommandResponseOptions::ApplicationCommandRoleOptionResponse(
                        ApplicationCommandRoleOptionResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            3 => {
                Ok(
                    ApplicationCommandResponseOptions::ApplicationCommandStringOptionResponse(
                        ApplicationCommandStringOptionResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }
            2 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandSubcommandGroupOptionResponse(
                    ApplicationCommandSubcommandGroupOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            1 => Ok(
                ApplicationCommandResponseOptions::ApplicationCommandSubcommandOptionResponse(
                    ApplicationCommandSubcommandOptionResponse::deserialize(
                        ContentRefDeserializer::<D::Error>::new(&content),
                    )?,
                ),
            ),
            6 => {
                Ok(
                    ApplicationCommandResponseOptions::ApplicationCommandUserOptionResponse(
                        ApplicationCommandUserOptionResponse::deserialize(
                            ContentRefDeserializer::<D::Error>::new(&content),
                        )?,
                    ),
                )
            }

            other => Err(serde::de::Error::unknown_variant(
                &other.to_string(),
                &["11", "5", "7", "4", "9", "10", "8", "3", "2", "1", "6"],
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationCommandResponse {
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,
    pub name: String,
    // Optional TODO
    pub default_member_permissions: Option<String>,
    pub version: SnowflakeType,
    // Optional TODO
    pub nsfw: Option<bool>,
    // Optional TODO
    pub name_localized: Option<String>,
    // Optional TODO
    pub description_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub options: Option<Vec<ApplicationCommandResponseOptions>>,
    // Optional TODO
    pub guild_id: Option<SnowflakeType>,
    pub id: SnowflakeType,
    pub description: String,
    // Optional TODO
    pub name_localizations: Option<HashMap<String, String>>,
    // Optional TODO
    pub description_localized: Option<String>,
    // Optional TODO
    pub dm_permission: Option<bool>,
    pub application_id: SnowflakeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageMentionChannelResponse {
    pub name: String,
    pub guild_id: SnowflakeType,
    pub id: SnowflakeType,
    #[serde(rename = "type")]
    pub kind: ChannelTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
}
impl From<MessageType> for i16 {
    fn from(v: MessageType) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for MessageType {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Default),
            1 => Ok(Self::RecipientAdd),
            2 => Ok(Self::RecipientRemove),
            3 => Ok(Self::Call),
            4 => Ok(Self::ChannelNameChange),
            5 => Ok(Self::ChannelIconChange),
            6 => Ok(Self::ChannelPinnedMessage),
            7 => Ok(Self::UserJoin),
            8 => Ok(Self::GuildBoost),
            9 => Ok(Self::GuildBoostTier1),
            10 => Ok(Self::GuildBoostTier2),
            11 => Ok(Self::GuildBoostTier3),
            12 => Ok(Self::ChannelFollowAdd),
            14 => Ok(Self::GuildDiscoveryDisqualified),
            15 => Ok(Self::GuildDiscoveryRequalified),
            16 => Ok(Self::GuildDiscoveryGracePeriodInitialWarning),
            17 => Ok(Self::GuildDiscoveryGracePeriodFinalWarning),
            18 => Ok(Self::ThreadCreated),
            19 => Ok(Self::Reply),
            20 => Ok(Self::ChatInputCommand),
            21 => Ok(Self::ThreadStarterMessage),
            22 => Ok(Self::GuildInviteReminder),
            23 => Ok(Self::ContextMenuCommand),
            24 => Ok(Self::AutoModerationAction),
            25 => Ok(Self::RoleSubscriptionPurchase),
            26 => Ok(Self::InteractionPremiumUpsell),
            27 => Ok(Self::StageStart),
            28 => Ok(Self::StageEnd),
            29 => Ok(Self::StageSpeaker),
            31 => Ok(Self::StageTopic),
            32 => Ok(Self::GuildApplicationPremiumSubscription),
            36 => Ok(Self::GuildIncidentAlertModeEnabled),
            37 => Ok(Self::GuildIncidentAlertModeDisabled),
            38 => Ok(Self::GuildIncidentReportRaid),
            39 => Ok(Self::GuildIncidentReportFalseAlarm),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "i16")]
#[serde(into = "i16")]
pub enum StageInstancesPrivacyLevels {
    Public = 1,
    GuildOnly = 2,
}
impl From<StageInstancesPrivacyLevels> for i16 {
    fn from(v: StageInstancesPrivacyLevels) -> Self {
        v as i16
    }
}
impl TryFrom<i16> for StageInstancesPrivacyLevels {
    type Error = String;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Public),
            2 => Ok(Self::GuildOnly),
            other => Err(format!("Unimplemented variant {}", other)),
        }
    }
}
