// @generated automatically by Diesel CLI.

diesel::table! {
    tb_batch (batch_number, batch_year) {
        batch_number -> Integer,
        batch_year -> Integer,
        batch_month -> Nullable<Integer>,
        seed -> Text,
        coating -> Text,
        brand -> Text,
        sack_weight -> Nullable<Integer>,
        sack_amount -> Nullable<Integer>,
        total_weight -> Nullable<Integer>,
        pureness_score -> Nullable<Float>,
        total_pureness_score -> Nullable<Float>,
        origin -> Nullable<Text>,
    }
}

diesel::table! {
    tb_brand (brand_name) {
        brand_name -> Text,
    }
}

diesel::table! {
    tb_brand_weights (brand, sack_weight) {
        brand -> Text,
        sack_weight -> Integer,
    }
}

diesel::table! {
    tb_coating (coating_name) {
        coating_name -> Text,
    }
}

diesel::table! {
    tb_outflow (id) {
        id -> Nullable<Integer>,
        sack_amount -> Nullable<Integer>,
        total_weight -> Nullable<Integer>,
        total_pureness_score -> Nullable<Float>,
        usage -> Nullable<Text>,
    }
}

diesel::table! {
    tb_seed (scientific_name) {
        scientific_name -> Text,
        popular_name -> Nullable<Text>,
    }
}

diesel::table! {
    tb_weight (sack_weight) {
        sack_weight -> Integer,
    }
}

diesel::joinable!(tb_batch -> tb_seed (seed));
diesel::joinable!(tb_batch -> tb_weight (sack_weight));
diesel::joinable!(tb_brand_weights -> tb_brand (brand));
diesel::joinable!(tb_brand_weights -> tb_weight (sack_weight));

diesel::allow_tables_to_appear_in_same_query!(
    tb_batch,
    tb_brand,
    tb_brand_weights,
    tb_coating,
    tb_outflow,
    tb_seed,
    tb_weight,
);
