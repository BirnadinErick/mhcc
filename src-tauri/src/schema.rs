table! {
    grns (grn_id) {
        grn_id -> Nullable<Integer>,
        date_returned -> Integer,
        staff_returned -> Integer,
    }
}

table! {
    sales (sales_id) {
        sales_id -> Nullable<Integer>,
        v_id -> Integer,
        date_sold -> Integer,
        staff_id -> Integer,
    }
}

table! {
    sales_item (sales_id, stock_id) {
        sales_id -> Integer,
        stock_id -> Integer,
        quantity -> Integer,
        uprice -> Float,
    }
}

table! {
    staffs (staff_id) {
        staff_id -> Nullable<Integer>,
        name -> Text,
        uname -> Text,
        passwd -> Text,
        role -> Integer,
        date_enrolled -> Integer,
    }
}

table! {
    stocks (stock_id) {
        stock_id -> Nullable<Integer>,
        name -> Text,
        dispenser -> Text,
        uprice -> Float,
        quantity -> Integer,
        date_expiry -> Integer,
        staff_stocked -> Integer,
    }
}

table! {
    visitors (v_id) {
        v_id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
        tpno -> Text,
        dob -> Integer,
        nic -> Text,
    }
}

joinable!(grns -> staffs (staff_returned));
joinable!(sales -> staffs (staff_id));
joinable!(sales_item -> sales (sales_id));
joinable!(sales_item -> stocks (stock_id));
joinable!(stocks -> staffs (staff_stocked));

allow_tables_to_appear_in_same_query!(
    grns,
    sales,
    sales_item,
    staffs,
    stocks,
    visitors,
);
