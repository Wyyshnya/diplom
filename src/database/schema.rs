// @generated automatically by Diesel CLI.

diesel::table! {
    administration (id) {
        id -> Int4,
        fio -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        position_office -> Varchar,
    }
}

diesel::table! {
    chats (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
    }
}

diesel::table! {
    content (id) {
        id -> Int4,
        content -> Nullable<Text>,
        type_content -> Nullable<Text>,
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
    departments (id) {
        id -> Int4,
        number -> Varchar,
        name -> Varchar,
        deanery_id -> Nullable<Int4>,
    }
}

diesel::table! {
    group_subject (id) {
        id -> Int4,
        group_id -> Nullable<Int4>,
        subject_id -> Nullable<Int4>,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        number -> Varchar,
        department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        chat_id -> Nullable<Int4>,
        sender_type -> Varchar,
        sender_id -> Nullable<Int4>,
        date_send -> Nullable<Timestamp>,
        content_id -> Nullable<Int4>,
    }
}

diesel::table! {
    p_to_p (id) {
        id -> Int4,
        chat_id -> Nullable<Int4>,
        user_id -> Int4,
        type_user -> Varchar,
    }
}

diesel::table! {
    sections (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        fio -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    subjects (id) {
        id -> Int4,
        name -> Varchar,
        department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    t_to_g (id) {
        id -> Int4,
        chat_id -> Nullable<Int4>,
        admin -> Nullable<Int4>,
        group_id -> Nullable<Int4>,
    }
}

diesel::table! {
    teacher_department (id) {
        id -> Int4,
        teacher_id -> Nullable<Int4>,
        department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    teacher_subject (id) {
        id -> Int4,
        teacher_id -> Nullable<Int4>,
        subject_id -> Nullable<Int4>,
    }
}

diesel::table! {
    teachers (id) {
        id -> Int4,
        fio -> Varchar,
        email -> Varchar,
        phone -> Varchar,
    }
}

diesel::table! {
    users_many (id) {
        id -> Int4,
        chat_id -> Nullable<Int4>,
        user_id -> Int4,
        type_user -> Varchar,
    }
}

diesel::table! {
    workers (id) {
        id -> Int4,
        fio -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        position_office -> Varchar,
        section_id -> Nullable<Int4>,
    }
}

diesel::joinable!(departments -> deaneries (deanery_id));
diesel::joinable!(group_subject -> groups (group_id));
diesel::joinable!(group_subject -> subjects (subject_id));
diesel::joinable!(groups -> departments (department_id));
diesel::joinable!(messages -> chats (chat_id));
diesel::joinable!(messages -> content (content_id));
diesel::joinable!(p_to_p -> chats (chat_id));
diesel::joinable!(students -> groups (group_id));
diesel::joinable!(subjects -> departments (department_id));
diesel::joinable!(t_to_g -> chats (chat_id));
diesel::joinable!(t_to_g -> groups (group_id));
diesel::joinable!(t_to_g -> teachers (admin));
diesel::joinable!(teacher_department -> departments (department_id));
diesel::joinable!(teacher_department -> teachers (teacher_id));
diesel::joinable!(teacher_subject -> subjects (subject_id));
diesel::joinable!(teacher_subject -> teachers (teacher_id));
diesel::joinable!(users_many -> chats (chat_id));
diesel::joinable!(workers -> sections (section_id));

diesel::allow_tables_to_appear_in_same_query!(
    administration,
    chats,
    content,
    deaneries,
    departments,
    group_subject,
    groups,
    messages,
    p_to_p,
    sections,
    students,
    subjects,
    t_to_g,
    teacher_department,
    teacher_subject,
    teachers,
    users_many,
    workers,
);
