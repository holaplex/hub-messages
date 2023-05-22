//!

use async_std::stream::StreamExt;
use holaplex_hub_messages::{events, Args, Services};
use hub_core::{
    tokio::{self, task},
    tracing::{info, warn},
};
use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        client::{Tls, TlsParameters},
        PoolConfig,
    },
    SmtpTransport,
};
use tera::Tera;

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-messages",
    };

    hub_core::run(opts, |common, args| {
        let Args {
            domain,
            source_email,
            smtp,
        } = args;

        let tera = Tera::new("./templates/*.html")?;

        let creds = Credentials::new(smtp.username.to_owned(), smtp.password.to_owned());

        let tls = TlsParameters::builder(smtp.server.to_owned())
            .dangerous_accept_invalid_certs(true)
            .build()?;

        let mailer = SmtpTransport::relay(&smtp.server)?
            .credentials(creds)
            .authentication(vec![Mechanism::Login])
            .pool_config(PoolConfig::new().max_size(smtp.pool_size.into()))
            .tls(Tls::Required(tls))
            .port(smtp.plaintext_port)
            .build();

        common.rt.block_on(async move {
            let cons = common.consumer_cfg.build::<Services>().await?;

            let mut stream = cons.stream();
            loop {
                let domain = domain.clone();
                let source_email = source_email.clone();
                let mailer = mailer.clone();
                let tera = tera.clone();

                match stream.next().await {
                    Some(Ok(msg)) => {
                        info!(?msg, "message received");

                        tokio::spawn(async move {
                            events::process(msg, &mailer, &tera, &domain, &source_email)
                        });
                        task::yield_now().await;
                    },
                    None => (),
                    Some(Err(e)) => {
                        warn!("failed to get message {:?}", e);
                    },
                }
            }
        })
    });
}
