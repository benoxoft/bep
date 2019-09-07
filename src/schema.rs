table! {
    building_managers (id) {
        id -> Uuid,
        full_name -> Varchar,
        profile_picture -> Bytea,
        coordinates_id -> Nullable<Uuid>,
        linked_user_id -> Nullable<Uuid>,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

table! {
    building_owners (id) {
        id -> Uuid,
        full_name -> Varchar,
        is_manager -> Bool,
        manager_id -> Nullable<Uuid>,
        linked_user_id -> Nullable<Uuid>,
        coordinates_id -> Nullable<Uuid>,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

table! {
    buildings (id) {
        id -> Uuid,
        owner_id -> Uuid,
        manager_id -> Uuid,
        respondant_id -> Uuid,
        name -> Varchar,
        address -> Varchar,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

table! {
    coordinates (id) {
        id -> Uuid,
        address -> Varchar,
        telephone_no -> Varchar,
        fax -> Varchar,
        cellphone_no -> Varchar,
        email -> Varchar,
        company_name -> Varchar,
        company_number -> Varchar,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

table! {
    entities_files (id) {
        id -> Uuid,
        entity_id -> Uuid,
        filename -> Varchar,
        url -> Varchar,
        content -> Nullable<Varchar>,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    entities_history (id) {
        id -> Uuid,
        entity_id -> Uuid,
        action_id -> Int2,
        file_id -> Uuid,
        user_id -> Uuid,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    entities_notes (id) {
        id -> Uuid,
        entity_id -> Uuid,
        user_id -> Uuid,
        note -> Varchar,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    registers (id) {
        id -> Uuid,
        name -> Varchar,
        building_id -> Uuid,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        permission -> Int2,
        full_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        job_title -> Varchar,
        profile_picture -> Varchar,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

joinable!(building_managers -> coordinates (coordinates_id));
joinable!(building_managers -> users (linked_user_id));
joinable!(building_owners -> building_managers (manager_id));
joinable!(building_owners -> coordinates (coordinates_id));
joinable!(building_owners -> users (linked_user_id));
joinable!(buildings -> building_managers (manager_id));
joinable!(buildings -> building_owners (owner_id));
joinable!(buildings -> users (respondant_id));
joinable!(entities_history -> entities_files (file_id));
joinable!(entities_history -> users (user_id));
joinable!(entities_notes -> users (user_id));
joinable!(registers -> buildings (building_id));

allow_tables_to_appear_in_same_query!(
    building_managers,
    building_owners,
    buildings,
    coordinates,
    entities_files,
    entities_history,
    entities_notes,
    registers,
    users,
);
