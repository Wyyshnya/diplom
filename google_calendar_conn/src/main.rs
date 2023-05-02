use std::fmt::Debug;
use chrono::{Duration, Utc};
use google_calendar::Client;
use google_calendar::types::{Event, EventDateTime};
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};


#[tokio::main]
async fn main() {
    // TODO сделать через фронт, как написано на листочке
    // Создание календаря
    let mut client = Client::new(
        String::from("1032867852781-vk3sth9p1c0fi2fvl0jddhu2c0uk6hj6.apps.googleusercontent.com"),
        String::from("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"),
        String::from("http://localhost"),
        String::from(""),
        String::from(""),
    );

    let user_consent_url = client.user_consent_url(&["https://www.googleapis.com/auth/calendar".to_string()]);

    webbrowser::open(&*user_consent_url).unwrap();


    let code = url::Url::parse(&*user_consent_url).unwrap().query_pairs().find(|(key, _)| key == "code").unwrap().1.to_string();
    let state = url::Url::parse(&*user_consent_url).unwrap().query_pairs().find(|(key, _)| key == "state").unwrap().1.to_string();

    let mut access_token = client.get_access_token(&*code, &*state).await.unwrap();

    let s = client.events().list_all("jefersonhrm1@gmail.com", "", 0, Default::default(), &[], "", &[], false, false, false, "", "", "", "").await;
    println!("{:?}", user_consent_url);
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
            date_time: Option::from(chrono::DateTime::<Utc>::from_local(chrono::Local::now().naive_local() + Duration::hours(1), Utc)),
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

    let p = client.events().insert("jefersonhrm1@gmail.com",
                                   0,
                                   0,
                                   false,
                                   Default::default(),
                                   false,
                                   &event).await;
    // // let p = client.acl().insert("2c3b15364f6c792022d82abd9329dbde69a04092187e9d05f4655f5170efb872@group.calendar.google.com", false,
    // //                             &google_calendar::types::AclRule {
    // //                                 etag: "".to_string(),
    // //                                 id: "".to_string(),
    // //                                 kind: "calendar#aclRule".to_string(),
    // //                                 role: "writer".to_string(),
    // //                                 scope: Option::from(google_calendar::types::Scope {
    // //                                     type_: "user".to_string(),
    // //                                     value: "jefersonhrm1@gmail.com".to_string(),
    // //                                 }),
    // //                             }).await;
    println!("{:?}", p);
}

