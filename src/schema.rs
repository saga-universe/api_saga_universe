// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    countries (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        code -> Char,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    episodes (id) {
        id -> Unsigned<Integer>,
        number -> Unsigned<Integer>,
        name -> Varchar,
        description -> Nullable<Text>,
        duration_time -> Unsigned<Integer>,
        episode_url -> Varchar,
        saga_id -> Unsigned<Integer>,
        season_id -> Nullable<Unsigned<Integer>>,
        special_type_id -> Nullable<Unsigned<Integer>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    group_types (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        website_url -> Nullable<Varchar>,
        netophonix_url -> Nullable<Varchar>,
        group_types_id -> Nullable<Unsigned<Integer>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups_episodes (id) {
        id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        episode_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups_peoples (id) {
        id -> Unsigned<Integer>,
        people_id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    groups_sagas (id) {
        id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        saga_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    licences (id) {
        id -> Unsigned<Integer>,
        code -> Varchar,
        name -> Varchar,
        detail_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    people_roles (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    peoples (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        website_url -> Nullable<Varchar>,
        netophonix_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    peoples_episodes (id) {
        id -> Unsigned<Integer>,
        people_id -> Unsigned<Integer>,
        episode_id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    peoples_sagas (id) {
        id -> Unsigned<Integer>,
        people_id -> Unsigned<Integer>,
        saga_id -> Unsigned<Integer>,
        people_role_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sagas (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        description -> Text,
        author -> Varchar,
        music -> Varchar,
        season -> Unsigned<Smallint>,
        creation_date -> Varchar,
        netophonix_url -> Nullable<Varchar>,
        website_url -> Nullable<Varchar>,
        picture_url -> Nullable<Varchar>,
        licence_id -> Nullable<Unsigned<Integer>>,
        country_id -> Unsigned<Integer>,
        status_id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sagas_subcategories (id) {
        id -> Unsigned<Integer>,
        saga_id -> Unsigned<Integer>,
        subcategory_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    seasons (id) {
        id -> Unsigned<Integer>,
        number -> Unsigned<Integer>,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    special_types (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    status (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    subcategories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(episodes -> sagas (saga_id));
diesel::joinable!(episodes -> seasons (season_id));
diesel::joinable!(episodes -> special_types (special_type_id));
diesel::joinable!(groups -> group_types (group_types_id));
diesel::joinable!(groups_episodes -> episodes (episode_id));
diesel::joinable!(groups_episodes -> groups (group_id));
diesel::joinable!(groups_peoples -> groups (group_id));
diesel::joinable!(groups_peoples -> peoples (people_id));
diesel::joinable!(groups_sagas -> groups (group_id));
diesel::joinable!(groups_sagas -> sagas (saga_id));
diesel::joinable!(peoples_episodes -> episodes (episode_id));
diesel::joinable!(peoples_episodes -> people_roles (role_id));
diesel::joinable!(peoples_episodes -> peoples (people_id));
diesel::joinable!(peoples_sagas -> people_roles (people_role_id));
diesel::joinable!(peoples_sagas -> peoples (people_id));
diesel::joinable!(peoples_sagas -> sagas (saga_id));
diesel::joinable!(sagas -> categories (category_id));
diesel::joinable!(sagas -> countries (country_id));
diesel::joinable!(sagas -> licences (licence_id));
diesel::joinable!(sagas -> status (status_id));
diesel::joinable!(sagas_subcategories -> sagas (saga_id));
diesel::joinable!(sagas_subcategories -> subcategories (subcategory_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    countries,
    episodes,
    group_types,
    groups,
    groups_episodes,
    groups_peoples,
    groups_sagas,
    licences,
    people_roles,
    peoples,
    peoples_episodes,
    peoples_sagas,
    sagas,
    sagas_subcategories,
    seasons,
    special_types,
    status,
    subcategories,
);
