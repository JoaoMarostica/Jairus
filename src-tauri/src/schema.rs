// @generated automatically by Diesel CLI.

diesel::table! {
    tb_batch (batch_number, batch_year) {
        batch_number -> Integer,
        batch_year -> Integer,
        batch_month -> Integer,
        seed -> Text,
        coating -> Text,
        brand -> Text,
        sack_weight -> Integer,
        sack_amount -> Integer,
        total_weight -> Integer,
        pureness_score -> Float,
        total_pureness_score -> Float,
        origin -> Text,
    }
}

diesel::table! {
    tb_brand (brand_name, sack_weight) {
        brand_name -> Text,
        sack_weight -> Integer,
    }
}

diesel::table! {
    tb_coating (coating_name) {
        coating_name -> Text,
    }
}

diesel::table! {
    tb_outflow (outflow_id) {
        outflow_id -> Integer,
        batch_number -> Integer,
        batch_year -> Integer,
        sack_amount -> Integer,
        total_weight -> Integer,
        total_pureness_score -> Float,
        usage -> Text,
    }
}

diesel::table! {
    tb_seed (popular_name) {
        popular_name -> Text,
        scientific_name -> Text,
    }
}

diesel::joinable!(tb_batch -> tb_coating (coating));
diesel::joinable!(tb_batch -> tb_seed (seed));

diesel::allow_tables_to_appear_in_same_query!(
    tb_batch,
    tb_brand,
    tb_coating,
    tb_outflow,
    tb_seed,
);
