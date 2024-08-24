// @generated automatically by Diesel CLI.

diesel::table! {
    cats (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        image_path -> Varchar,
    }
}
