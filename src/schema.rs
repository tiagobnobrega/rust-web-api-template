table! {
    roles (role_id) {
        role_id -> Int4,
        role_name -> Varchar,
    }
}

table! {
    roles_system_actions (roles_system_actions_id) {
        roles_system_actions_id -> Int4,
        role_id -> Int4,
        action_id -> Int4,
    }
}

table! {
    system_actions (action_id) {
        action_id -> Int4,
        action_name -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        primary_email -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

table! {
    users_roles (users_roles_id) {
        users_roles_id -> Int4,
        username -> Varchar,
        role_id -> Int4,
    }
}

joinable!(roles_system_actions -> roles (role_id));
joinable!(roles_system_actions -> system_actions (role_id));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (username));

allow_tables_to_appear_in_same_query!(
    roles,
    roles_system_actions,
    system_actions,
    users,
    users_roles,
);
