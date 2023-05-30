#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod events;
pub mod mailer;

use hub_core::{anyhow::Result, clap, consumer::RecvError, prelude::*};

#[allow(clippy::pedantic)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/organization.proto.rs"));
}

#[derive(Debug)]
pub enum Services {
    Organizations(proto::OrganizationEventKey, proto::OrganizationEvents),
}

impl hub_core::consumer::MessageGroup for Services {
    const REQUESTED_TOPICS: &'static [&'static str] = &["hub-orgs"];

    fn from_message<M: hub_core::consumer::Message>(msg: &M) -> Result<Self, RecvError> {
        let topic = msg.topic();
        let key = msg.key().ok_or(RecvError::MissingKey)?;
        let val = msg.payload().ok_or(RecvError::MissingPayload)?;
        info!(topic, ?key, ?val);

        match topic {
            "hub-orgs" => {
                let key = proto::OrganizationEventKey::decode(key)?;
                let val = proto::OrganizationEvents::decode(val)?;

                Ok(Services::Organizations(key, val))
            },

            t => Err(RecvError::BadTopic(t.into())),
        }
    }
}

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env)]
    pub domain: String,

    #[arg(short, long, env)]
    pub source_email: String,

    #[command(flatten)]
    pub smtp: mailer::SmtpServerArgs,
}
