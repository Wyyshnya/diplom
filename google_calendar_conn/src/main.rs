use std::thread::scope;
use chrono::Utc;
use google_calendar::Client;
use google_calendar::types::{Event, EventDateTime};


#[tokio::main]
async fn main() {
    // TODO нужно авторизоваться и получить Oauth token
    // Создание календаря
    let mut client = Client::new(
        String::from("1032867852781-vk3sth9p1c0fi2fvl0jddhu2c0uk6hj6.apps.googleusercontent.com"),
        String::from("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"),
        String::from("https://oauth2.googleapis.com/token"),
        String::from("ya29.a0AWY7CknVWDZfqYOWgYogGp9nrcJh-_7ezNBBY0OdZQxxDKt4EB4r4ucMEguR4Htrf6bMLOZrHyTLGMo5leAH8w8GlVLfIugYoLb7WsY3xPglr8IuMGLD1xXYOAHW_5BvcjP5wIIOQpCowlFLXeSKBz39759baCgYKAYISARASFQG1tDrpO6RbBigtXW-P8PmJZQFzWQ0163"),
        String::from("1//0cRtbEVhW9YzgCgYIARAAGAwSNwF-L9Irmf3eh7nUOnhfCNPX4UgcIllRnIx1taVo2guWO1ZChzebRKN0tOxJXPFhUG82HM3Sg7Q"),
    );

    let user_consent_url = client.user_consent_url(&["https://www.googleapis.com/auth/calendar".to_string()]);

    let s = client.events().list_all("jefersonhrm1@gmail.com", "", 0, Default::default(), &[], "", &[], false, false, false, "", "", "", "").await;
    println!("{:?}", s);
    let event = Event {
        anyone_can_add_self: true,
        attachments: vec![],
        attendees: vec![],
        attendees_omitted: false,
        color_id: "".to_string(),
        conference_data: None,
        created: None,
        summary: String::from("My Event"),
        transparency: "".to_string(),
        updated: None,
        description: String::from("Description of my event"),
        end: Option::from(EventDateTime {
            date: None,
            date_time: Option::from(chrono::DateTime::<Utc>::from_local(chrono::Local::now().naive_local(), Utc)),
            time_zone: "".to_string()
        }),
        end_time_unspecified: false,
        etag: "".to_string(),
        event_type: "".to_string(),
        extended_properties: None,
        gadget: None,
        guests_can_invite_others: false,
        guests_can_modify: false,
        guests_can_see_other_guests: false,
        hangout_link: "".to_string(),
        html_link: "".to_string(),
        i_cal_uid: "".to_string(),
        id: "".to_string(),
        kind: "".to_string(),
        location: "".to_string(),
        locked: false,
        organizer: None,
        original_start_time: None,
        private_copy: false,
        recurrence: vec![],
        recurring_event_id: "".to_string(),
        reminders: None,
        sequence: 0,
        source: None,
        start: Option::from(EventDateTime {
            date: None,
            date_time: Some(chrono::DateTime::<Utc>::from_local(chrono::Local::now().naive_local(), Utc)),
            time_zone: "".to_string()
        }),
        creator: None,
        status: "".to_string(),
        visibility: "".to_string(),
    };
    // let p = client.events().insert("jefersonhrm1@gmail.com",
    //              0,
    //              0,
    //              false,
    //              Default::default(),
    //              false,
    //              &event).await;
    let p = client.events().insert("2c3b15364f6c792022d82abd9329dbde69a04092187e9d05f4655f5170efb872@group.calendar.google.com",
                                   0,
                                   0,
                                   false,
                                   Default::default(),
                                   false,
                                   &Default::default()).await;
    // let p = client.acl().insert("2c3b15364f6c792022d82abd9329dbde69a04092187e9d05f4655f5170efb872@group.calendar.google.com", false,
    //                             &google_calendar::types::AclRule {
    //                                 etag: "".to_string(),
    //                                 id: "".to_string(),
    //                                 kind: "calendar#aclRule".to_string(),
    //                                 role: "writer".to_string(),
    //                                 scope: Option::from(google_calendar::types::Scope {
    //                                     type_: "user".to_string(),
    //                                     value: "jefersonhrm1@gmail.com".to_string(),
    //                                 }),
    //                             }).await;
    println!("{:?}", p);
}