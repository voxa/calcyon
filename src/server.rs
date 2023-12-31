use chrono::{prelude::DateTime, Duration, Utc};
use ical::generator::{Emitter, IcalCalendarBuilder, IcalEventBuilder, Property};
use serenity::model::guild::ScheduledEvent;
use warp::{http::Response, reject, Filter};

use serenity::http::Http as DiscordHttp;

fn generate_ical(events: Vec<ScheduledEvent>) -> String {
    dbg!("Started parsing");
    let fmt = "%Y%m%dT%H%M%S";

    let now: String = Utc::now().format(fmt).to_string();

    let mut calendar = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();

    for scheduled_event in events {
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

    dbg!("Finished parsing");

    calendar.generate()
}

async fn generate_guild_calendar(
    guild_id: u64,
    token: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = DiscordHttp::new(&token);

    let events = client.get_scheduled_events(guild_id, false).await;

    match events {
        Ok(events) => Ok(Response::builder()
            .header("content-type", "text/calendar")
            .body(generate_ical(events))),
        // TODO - implement custom error handling
        Err(_error) => Err(reject()),
    }
}

fn with_token(
    token: String,
) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || token.clone())
}

pub async fn start(token: String) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("calendar" / "guild" / u64)
        .and(with_token(token))
        .and_then(generate_guild_calendar);

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
