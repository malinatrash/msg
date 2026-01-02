// @generated automatically by Diesel CLI.

diesel::table! {
    auth_users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        role_id -> Int4,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chat_members (id) {
        id -> Uuid,
        chat_id -> Uuid,
        user_id -> Uuid,
        invited_by -> Nullable<Uuid>,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        chat_id -> Uuid,
        sender_id -> Uuid,
        encrypted_content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(auth_users -> roles (role_id));
diesel::joinable!(chats -> auth_users (created_by));
diesel::joinable!(chat_members -> chats (chat_id));
diesel::joinable!(messages -> chats (chat_id));

diesel::allow_tables_to_appear_in_same_query!(auth_users, chats, chat_members, messages, roles,);
