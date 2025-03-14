use crate::Notification;

pub type ChannelConnection = entropyx_notify::connection::ChannelConnection<Notification>;
pub use entropyx_notify::connection::ChannelType;
