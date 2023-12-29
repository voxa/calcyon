use std::{net::SocketAddr, sync::Arc};

use http_body_util::Full;
use hyper_util::rt::TokioIo;
use ical::generator::{Emitter, IcalCalendarBuilder, IcalEventBuilder, Property};
use serenity::model::guild::ScheduledEvent;
use tokio::net::TcpListener;

use hyper::{body::Bytes, server::conn::http1, service::service_fn, Error, Response};

use chrono::{prelude::DateTime, Duration, Utc};

fn generate_ical(events: Vec<ScheduledEvent>) -> String {
    let fmt = "%Y%m%dT%H%M%S";

    let now: String = Utc::now().format(fmt).to_string();

    let mut calendar = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();

    for scheduled_event in events {
        dbg!(&scheduled_event);

        let ScheduledEvent {
            id,
            guild_id,
            name,
            description,
            start_time,
            end_time,
            ..
        } = scheduled_event;

        let base_event = IcalEventBuilder::tzid("Etc/UTC")
            .uid(format!("{id}/{guild_id}"))
            .changed(&now);

        let mut ical_event = match end_time {
            Some(end_time) => base_event
                .start(start_time.format(fmt).to_string())
                .end(end_time.format(fmt).to_string()),
            None => {
                let end_time = DateTime::parse_from_rfc3339(&start_time.to_rfc3339()).unwrap()
                    + Duration::minutes(30);

                base_event
                    .start(start_time.format(fmt).to_string())
                    .end(end_time.format(fmt).to_string())
            }
        };

        ical_event = ical_event.set(ical::ical_property!("SUMMARY", name));

        if let Some(description) = description {
            if !description.is_empty() {
                ical_event = ical_event.set(ical::ical_property!("DESCRIPTION", description));
            }
        }

        calendar.events.push(ical_event.build());
    }

    calendar.generate()
}

pub async fn start(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let client = Arc::new(serenity::http::Http::new(token));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let client = client.clone();

        let service = service_fn(move |_req| {
            let client = client.clone();

            async move {
                dbg!("Hello!");

                Ok::<_, Error>(Response::new(Full::new(Bytes::from(generate_ical(
                    client
                        .get_scheduled_events(0, false)
                        .await
                        .unwrap(),
                )))))
            }
        });

        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}
