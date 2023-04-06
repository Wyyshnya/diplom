// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Int4,
        title -> Varchar,
        is_dialog -> Bool,
    }
}

diesel::table! {
    content_message (id) {
        id -> Int4,
        content -> Text,
        type_content -> Text,
    }
}

diesel::table! {
    deaneries (id) {
        id -> Int4,
        number -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    group_subject (id) {
        id -> Int4,
        group_id -> Int4,
        subject_id -> Int4,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        number -> Varchar,
        deanery_id -> Int4,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        chat_id -> Int4,
        sender_id -> Int4,
        date_send -> Timestamp,
        content_id -> Int4,
    }
}

diesel::table! {
    subjects (id) {
        id -> Int4,
        name -> Varchar,
        deanery_id -> Int4,
    }
}

diesel::table! {
    teacher_deanery (id) {
        id -> Int4,
        teacher_id -> Int4,
        deanery_id -> Int4,
    }
}

diesel::table! {
    teacher_subject (id) {
        id -> Int4,
        teacher_id -> Int4,
        subject_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        fio -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        position_office -> Nullable<Varchar>,
        group_id -> Nullable<Int4>,
        is_teacher -> Bool,
    }
}

diesel::table! {
    users_chats (user_id, chat_id) {
        user_id -> Int4,
        chat_id -> Int4,
    }
}

diesel::joinable!(group_subject -> groups (group_id));
diesel::joinable!(group_subject -> subjects (subject_id));
diesel::joinable!(groups -> deaneries (deanery_id));
diesel::joinable!(messages -> chats (chat_id));
diesel::joinable!(messages -> content_message (content_id));
diesel::joinable!(messages -> users (sender_id));
diesel::joinable!(subjects -> deaneries (deanery_id));
diesel::joinable!(teacher_subject -> subjects (subject_id));
diesel::joinable!(teacher_subject -> users (teacher_id));
diesel::joinable!(users -> groups (group_id));
diesel::joinable!(users_chats -> chats (chat_id));
diesel::joinable!(users_chats -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    content_message,
    deaneries,
    group_subject,
    groups,
    messages,
    subjects,
    teacher_deanery,
    teacher_subject,
    users,
    users_chats,
);
