use hub_core::prelude::*;
use lettre::{self, message::header::ContentType, Message, SmtpTransport, Transport};
use tera::{Context, Tera};

use crate::{
    proto::{organization_events::Event as OrganizationEvent, Invite, OrganizationEventKey},
    Services,
};
/// This function matches each event type and processes it.
///
/// # Errors
/// This function fails if it is unable to process any event
pub fn process(
    msg: Services,
    mailer: &SmtpTransport,
    tera: &Tera,
    domain: &str,
    source_email: &str,
) -> Result<()> {
    // match topics
    match msg {
        Services::Organizations(key, e) => match e.event {
            Some(OrganizationEvent::InviteCreated(invite)) => {
                send_invite_email(mailer, tera, domain, source_email, key, invite)
            },
            Some(_) | None => Ok(()),
        },
    }
}

fn send_invite_email(
    mailer: &SmtpTransport,
    tera: &Tera,
    domain: &str,
    source_email: &str,
    key: OrganizationEventKey,
    invite: Invite,
) -> Result<()> {
    let Invite {
        email,
        organization,
    } = invite;

    let OrganizationEventKey { id, .. } = key;

    let mut context = Context::new();

    let url = Url::from_str(&format!("https://{domain}/invites/{id}"))?;

    context.insert("action_url", url.as_str());
    context.insert("org_name", &organization);

    let html: String = tera.render("send_invite.html", &context)?;

    let email = Message::builder()
        .from(format!("Holaplex Support <{source_email}>").parse()?)
        .to(email.parse().unwrap())
        .subject(format!("You have been invited to join {organization}"))
        .header(ContentType::TEXT_HTML)
        .body(html)?;

    mailer.send(&email)?;

    Ok(())
}
