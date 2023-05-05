use chrono::{Duration, Utc};
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use google_calendar::Client;
use google_calendar::types::Event;


#[tokio::main]
pub async fn auth() -> String {
    let secret = yup_oauth2::read_application_secret("/home/wyyshnya/RustProjects/aci_diplom/conn_google_calendar/src/creds.json")
        .await
        .expect("creds.json");

    let mut auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .build()
        .await
        .unwrap(); // .persist_tokens_to_disk("/home/wyyshnya/RustProjects/aci_diplom/conn_google_calendar/src/tokencache.json")

    let scopes = &["https://www.googleapis.com/auth/calendar"];
    let mut token_str = String::from("");
    match auth.token(scopes).await {
        Ok(token) => {
            token_str = token.token().unwrap().to_owned();
        }
        Err(e) => println!("error: {:?}", e),
    }
    token_str
}

async fn client_new(token_str: &String) -> Client{
    // Создание клиента пользователя
    let client = Client::new(
        String::from("1032867852781-vk3sth9p1c0fi2fvl0jddhu2c0uk6hj6.apps.googleusercontent.com"),
        String::from("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"),
        String::from("http://localhost"),
        token_str,
        String::from(""),
    );
    client
}


#[tokio::main]
pub async fn insert_event_lib(token_str: &String, event: &Event) {
    let client = client_new(token_str).await;

    // Событие в календаре
    // let event = Event {
    //     anyone_can_add_self: true,
    //     attachments: vec![],
    //     attendees: vec![],
    //     attendees_omitted: false,
    //     color_id: "".to_string(),
    //     conference_data: None,
    //     created: None,
    //     summary: String::from("My Event"),
    //     transparency: "".to_string(),
    //     updated: None,
    //     description: String::from("Description of my event"),
    //     end: Option::from(EventDateTime {
    //         date: None,
    //         date_time: Option::from(chrono::DateTime::<Utc>::from_local(chrono::Local::now().naive_local() + Duration::hours(1), Utc)),
    //         time_zone: "".to_string(),
    //     }),
    //     end_time_unspecified: false,
    //     etag: "".to_string(),
    //     event_type: "".to_string(),
    //     extended_properties: None,
    //     gadget: None,
    //     guests_can_invite_others: false,
    //     guests_can_modify: false,
    //     guests_can_see_other_guests: false,
    //     hangout_link: "".to_string(),
    //     html_link: "".to_string(),
    //     i_cal_uid: "".to_string(),
    //     id: "".to_string(),
    //     kind: "".to_string(),
    //     location: "".to_string(),
    //     locked: false,
    //     organizer: None,
    //     original_start_time: None,
    //     private_copy: false,
    //     recurrence: vec![],
    //     recurring_event_id: "".to_string(),
    //     reminders: None,
    //     sequence: 0,
    //     source: None,
    //     start: Option::from(EventDateTime {
    //         date: None,
    //         date_time: Some(chrono::DateTime::<Utc>::from_local(chrono::Local::now().naive_local(), Utc)),
    //         time_zone: "".to_string(),
    //     }),
    //     creator: None,
    //     status: "".to_string(),
    //     visibility: "".to_string(),
    // };

    let p = client.events().insert("jefersonhrm1@gmail.com",
                                   0,
                                   0,
                                   false,
                                   Default::default(),
                                   false,
                                   event).await;
    println!("{:?}", p);
}

#[tokio::main]
pub async fn get_list_of_events(token: &String) -> Vec<Event>{
    let client = client_new(token).await;

    // Вывод событий из календаря
    let s = client.events().list_all("jefersonhrm1@gmail.com", "", 0, Default::default(), &[], "", &[], false, false, false, "", "", "", "").await;

    s.unwrap()
}