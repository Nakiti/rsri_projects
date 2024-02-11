// @generated automatically by Diesel CLI.

diesel::table! {
    assignment_reviews (assignment_review_id) {
        assignment_review_id -> Int4,
        paperid -> Int4,
        userid -> Int4,
        assign_type -> Varchar,
    }
}

diesel::table! {
    paper_coauthors (paper_coauthor_id) {
        paper_coauthor_id -> Int4,
        paperid -> Int4,
        author -> Varchar,
    }
}

diesel::table! {
    papers (paperid) {
        paperid -> Int4,
        author -> Int4,
        title -> Varchar,
        #[sql_name = "abstract"]
        abstract_ -> Varchar,
        accepted -> Bool,
    }
}

diesel::table! {
    reviews (reviewid) {
        reviewid -> Int4,
        paperid -> Int4,
        userid -> Int4,
        content -> Varchar,
        score -> Int4,
    }
}

diesel::table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        name -> Varchar,
        email -> Varchar,
        affiliation -> Varchar,
        level -> Varchar,
    }
}

diesel::joinable!(assignment_reviews -> papers (paperid));
diesel::joinable!(assignment_reviews -> users (userid));
diesel::joinable!(papers -> users (author));
diesel::joinable!(reviews -> papers (paperid));
diesel::joinable!(reviews -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    assignment_reviews,
    paper_coauthors,
    papers,
    reviews,
    users,
);
