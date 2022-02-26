use base64::encode as base64_encode;
use serde::{Deserialize, Serialize};

/// Raw payload object to XSOverlay.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawNotification {
    message_type: i32,
    index: i32,
    timeout: f32,
    height: f32,
    opacity: f32,
    volume: f32,
    audio_path: String,
    title: String,
    content: String,
    use_base64_icon: bool,
    icon: String,
    source_app: String,
}

#[derive(Debug, Clone)]
pub struct NotificationBuilder {
    description: NotificationDescription,
    app_name: String,
    title: String,
    content: Option<String>,
    timeout: Option<f32>,
    opacity: Option<f32>,
    height: Option<f32>,
    volume: Option<f32>,
    icon: Option<NotificationIcon>,
    audio: Option<NotificationAudio>,
}

impl NotificationBuilder {
    /// Creates a new builder with application name and title.
    pub fn new(
        app_name: &str,
        title: &str,
        description: NotificationDescription,
    ) -> NotificationBuilder {
        NotificationBuilder {
            description,
            app_name: app_name.into(),
            title: title.into(),
            content: None,
            timeout: None,
            opacity: None,
            height: None,
            volume: None,
            icon: None,
            audio: None,
        }
    }

    pub fn build(self) -> RawNotification {
        RawNotification {
            message_type: match &self.description {
                NotificationDescription::Popup => 1,
                NotificationDescription::MediaPlayer { .. } => 2,
            },
            index: match &self.description {
                NotificationDescription::Popup => 0,
                NotificationDescription::MediaPlayer { icon_index } => *icon_index as i32,
            },
            timeout: self.timeout.unwrap_or(0.5),
            height: self.height.unwrap_or(175.0),
            opacity: self.opacity.unwrap_or(1.0),
            volume: self.volume.unwrap_or(0.7),
            audio_path: match self.audio {
                Some(NotificationAudio::Default) => "default".into(),
                Some(NotificationAudio::Error) => "error".into(),
                Some(NotificationAudio::Warning) => "warning".into(),
                Some(NotificationAudio::File(filename)) => filename,
                None => "".into(),
            },
            title: self.title,
            content: self.content.unwrap_or_default(),
            use_base64_icon: matches!(
                self.icon.as_ref().unwrap_or(&NotificationIcon::Default),
                NotificationIcon::Base64Binary(_)
            ),
            icon: match self.icon {
                Some(NotificationIcon::Default) => "default".into(),
                Some(NotificationIcon::Error) => "default".into(),
                Some(NotificationIcon::Warning) => "default".into(),
                Some(NotificationIcon::File(filename)) => filename,
                Some(NotificationIcon::Base64Binary(buffer)) => base64_encode(&buffer),
                None => "".into(),
            },
            source_app: self.app_name,
        }
    }

    /// Sets content.
    pub fn content(mut self, content: &str) -> NotificationBuilder {
        self.content = Some(content.into());
        self
    }

    /// Sets timeout in seconds.
    pub fn timeout(mut self, secs: f32) -> NotificationBuilder {
        self.timeout = Some(secs);
        self
    }

    /// Sets opacity.
    pub fn opacity(mut self, opacity: f32) -> NotificationBuilder {
        self.opacity = Some(opacity.max(0.0).min(1.0));
        self
    }

    /// Sets height.
    pub fn height(mut self, height: f32) -> NotificationBuilder {
        self.height = Some(height.max(0.0));
        self
    }

    /// Sets volume.
    pub fn volume(mut self, volume: f32) -> NotificationBuilder {
        self.volume = Some(volume);
        self
    }

    /// Sets notification icon.
    pub fn icon(mut self, icon: NotificationIcon) -> NotificationBuilder {
        self.icon = Some(icon);
        self
    }

    /// Sets notification audio.
    pub fn audio(mut self, audio: NotificationAudio) -> NotificationBuilder {
        self.audio = Some(audio);
        self
    }
}

/// Contains derived notification data.
#[derive(Debug, Clone)]
pub enum NotificationDescription {
    /// Popup.
    Popup,

    /// Media player information.
    MediaPlayer { icon_index: usize },
}

/// Represents an audio information for notification.
#[derive(Debug, Clone)]
pub enum NotificationAudio {
    /// Default audio.
    Default,

    /// Error audio.
    Error,

    /// Warning audio.
    Warning,

    /// User-specified filename.
    File(String),
}

/// Represents an icon information for notification.
#[derive(Debug, Clone)]
pub enum NotificationIcon {
    /// Default icon.
    Default,

    /// Error icon.
    Error,

    /// Warning icon.
    Warning,

    /// User-specified filename.
    File(String),

    /// Image binary sent in Base64.
    Base64Binary(Vec<u8>),
}
