use twilight_http::Client;
use twilight_model::channel::{
    message::{MessageFlags, MessageType},
    Message,
};

use crate::{
    attachment_sticker, avatar, component, error::Error, later_messages, reaction, reference,
    thread, MessageSource,
};

impl<'a> MessageSource<'a> {
    /// Create [`MessageSource`] from a [`Message`]
    ///
    /// # Warnings
    ///
    /// `message.guild_id` is usually `None` even if the message is in a guild,
    /// make sure this field is actually passed
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotInGuild`] if the message is not in a guild,
    ///
    /// Returns [`Error::RichPresence`] if the message is related
    /// to rich presence, which can't be recreated by bots
    ///
    /// Returns [`Error::Voice`] if the message is a voice message, which
    /// bots currently can't create
    ///
    /// Returns [`Error::System`] if the message's type isn't
    /// [`MessageType::Regular`] or [`MessageType::Reply`] or has role
    /// subscription data, which are edge-cases that can't be replicated
    /// correctly
    ///
    /// Returns [`Error::ContentInvalid`] if the message's content is
    /// invalid, this may happen when the author has used Nitro perks to send a
    /// message with over 2000 characters
    pub fn from_message(message: &'a Message, http: &'a Client) -> Result<Self, Error> {
        if message.activity.is_some() || message.application.is_some() {
            return Err(Error::RichPresence);
        }
        if message
            .flags
            .is_some_and(|flags| flags.contains(MessageFlags::IS_VOICE_MESSAGE))
        {
            return Err(Error::Voice);
        }
        if !matches!(message.kind, MessageType::Regular | MessageType::Reply)
            || message.role_subscription_data.is_some()
        {
            return Err(Error::System);
        }
        twilight_validate::message::content(&message.content).map_err(|_| Error::ContentInvalid)?;

        let guild_id = message.guild_id.ok_or(Error::NotInGuild)?;

        let url_components = component::filter_valid(&message.components);
        let has_invalid_components = message.components != url_components;

        let reference_info = message.referenced_message.as_ref().map_or_else(
            || {
                if message.kind == MessageType::Reply {
                    reference::Info::UnknownOrDeleted
                } else {
                    reference::Info::None
                }
            },
            |referenced_message| reference::Info::Reference(referenced_message),
        );

        let thread_info = message
            .thread
            .as_ref()
            .map_or(thread::Info::Unknown, |thread| {
                thread::Info::CreatedUnknown(Box::new(thread.clone()))
            });

        Ok(MessageSource {
            source_id: message.id,
            source_channel_id: message.channel_id,
            source_thread_id: thread_info.id(),
            content: message.content.clone(),
            embeds: message.embeds.clone(),
            tts: message.tts,
            flags: message.flags,
            channel_id: message.channel_id,
            guild_id,
            guild_emoji_ids: None,
            username: message
                .member
                .as_ref()
                .and_then(|member| member.nick.as_ref())
                .unwrap_or(&message.author.name)
                .clone(),
            reference_info,
            avatar_info: avatar::Info {
                url: None,
                user_id: message.author.id,
                guild_id,
                user_discriminator: message.author.discriminator,
                user_avatar: message.author.avatar,
                member_avatar: message.member.as_ref().and_then(|member| member.avatar),
            },
            webhook_name: "Message Cloner".to_owned(),
            reaction_info: reaction::Info {
                reactions: &message.reactions,
            },
            attachment_sticker_info: attachment_sticker::Info {
                stickers: &message.sticker_items,
                attachments: &message.attachments,
                #[cfg(feature = "upload")]
                attachments_upload: vec![],
            },
            component_info: component::Info {
                url_components,
                has_invalid_components,
            },
            thread_info,
            webhook: None,
            later_messages: later_messages::Info {
                messages: vec![],
                is_complete: false,
                is_source_created: false,
                is_later_message_sources_created: false,
            },
            response: None,
            http,
        })
    }
}
