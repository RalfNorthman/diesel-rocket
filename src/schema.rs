table! {
    authors (author_id) {
        author_id -> Integer,
        author_canonical -> Nullable<Mediumtext>,
        author_legalname -> Nullable<Mediumtext>,
        author_birthplace -> Nullable<Mediumtext>,
        author_birthdate -> Nullable<Date>,
        author_deathdate -> Nullable<Date>,
        note_id -> Nullable<Integer>,
        author_wikipedia -> Nullable<Mediumtext>,
        author_views -> Nullable<Integer>,
        author_imdb -> Nullable<Mediumtext>,
        author_marque -> Integer,
        author_image -> Nullable<Mediumtext>,
        author_annualviews -> Integer,
        author_lastname -> Nullable<Mediumtext>,
        author_language -> Nullable<Integer>,
        author_note -> Nullable<Mediumtext>,
    }
}

table! {
    authors_by_debut_date (row_id) {
        row_id -> Integer,
        debut_year -> Nullable<Integer>,
        author_id -> Nullable<Integer>,
        title_count -> Nullable<Integer>,
    }
}

table! {
    awards (award_id) {
        award_id -> Integer,
        award_title -> Nullable<Mediumtext>,
        award_author -> Nullable<Mediumtext>,
        award_year -> Nullable<Date>,
        award_ttype -> Nullable<Varchar>,
        award_atype -> Nullable<Mediumtext>,
        award_level -> Nullable<Mediumtext>,
        award_movie -> Nullable<Mediumtext>,
        award_type_id -> Nullable<Integer>,
        award_cat_id -> Nullable<Integer>,
        award_note_id -> Nullable<Integer>,
    }
}

table! {
    award_cats (award_cat_id) {
        award_cat_id -> Integer,
        award_cat_name -> Nullable<Mediumtext>,
        award_cat_type_id -> Nullable<Integer>,
        award_cat_order -> Nullable<Integer>,
        award_cat_note_id -> Nullable<Integer>,
    }
}

table! {
    award_titles_report (award_title_id) {
        award_title_id -> Integer,
        title_id -> Nullable<Integer>,
        score -> Nullable<Integer>,
        year -> Nullable<Integer>,
        decade -> Nullable<Varchar>,
        title_type -> Nullable<Enum>,
    }
}

table! {
    award_types (award_type_id) {
        award_type_id -> Integer,
        award_type_code -> Nullable<Varchar>,
        award_type_name -> Nullable<Mediumtext>,
        award_type_wikipedia -> Nullable<Mediumtext>,
        award_type_note_id -> Nullable<Integer>,
        award_type_by -> Nullable<Mediumtext>,
        award_type_for -> Nullable<Mediumtext>,
        award_type_short_name -> Nullable<Mediumtext>,
        award_type_poll -> Nullable<Enum>,
        award_type_non_genre -> Nullable<Enum>,
    }
}

table! {
    bad_images (pub_id) {
        pub_id -> Integer,
        image_url -> Nullable<Mediumtext>,
    }
}

table! {
    canonical_author (ca_id) {
        ca_id -> Integer,
        title_id -> Nullable<Integer>,
        author_id -> Nullable<Integer>,
        ca_status -> Nullable<Integer>,
    }
}

table! {
    changed_verified_pubs (change_id) {
        change_id -> Integer,
        pub_id -> Nullable<Integer>,
        sub_id -> Nullable<Integer>,
        verifier_id -> Nullable<Integer>,
        change_time -> Nullable<Datetime>,
    }
}

table! {
    cleanup (cleanup_id) {
        cleanup_id -> Integer,
        record_id -> Nullable<Integer>,
        report_type -> Nullable<Integer>,
        resolved -> Nullable<Bool>,
        record_id_2 -> Nullable<Integer>,
    }
}

table! {
    directory (directory_id) {
        directory_id -> Integer,
        directory_mask -> Nullable<Integer>,
        directory_index -> Nullable<Mediumtext>,
    }
}

table! {
    emails (email_id) {
        email_id -> Integer,
        author_id -> Nullable<Integer>,
        email_address -> Nullable<Mediumtext>,
    }
}

table! {
    history (history_id) {
        history_id -> Integer,
        history_time -> Nullable<Datetime>,
        history_table -> Nullable<Integer>,
        history_record -> Nullable<Integer>,
        history_field -> Nullable<Integer>,
        history_submission -> Nullable<Integer>,
        history_submitter -> Integer,
        history_reviewer -> Integer,
        history_from -> Nullable<Mediumtext>,
        history_to -> Nullable<Mediumtext>,
    }
}

table! {
    identifiers (identifier_id) {
        identifier_id -> Integer,
        identifier_type_id -> Nullable<Integer>,
        identifier_value -> Nullable<Text>,
        pub_id -> Nullable<Integer>,
    }
}

table! {
    identifier_sites (identifier_site_id) {
        identifier_site_id -> Integer,
        identifier_type_id -> Nullable<Integer>,
        site_position -> Nullable<Tinyint>,
        site_url -> Nullable<Text>,
        site_name -> Nullable<Text>,
    }
}

table! {
    identifier_types (identifier_type_id) {
        identifier_type_id -> Integer,
        identifier_type_name -> Nullable<Tinytext>,
        identifier_type_full_name -> Nullable<Text>,
    }
}

table! {
    languages (lang_id) {
        lang_id -> Integer,
        lang_name -> Nullable<Varchar>,
        lang_code -> Nullable<Varchar>,
        latin_script -> Nullable<Enum>,
    }
}

table! {
    license_keys (key_id) {
        key_id -> Integer,
        user_id -> Nullable<Integer>,
        license_key -> Nullable<Mediumtext>,
    }
}

table! {
    magazine (Mag_Code) {
        Mag_Code -> Varchar,
        Mag_Name -> Varchar,
        Mag_Desc -> Nullable<Varchar>,
    }
}

table! {
    most_reviewed (most_reviewed_id) {
        most_reviewed_id -> Integer,
        title_id -> Nullable<Integer>,
        year -> Nullable<Integer>,
        decade -> Nullable<Varchar>,
        reviews -> Nullable<Integer>,
    }
}

table! {
    mw_user (user_id) {
        user_id -> Unsigned<Integer>,
        user_name -> Varchar,
        user_real_name -> Varchar,
        user_password -> Tinyblob,
        user_newpassword -> Tinyblob,
        user_email -> Tinytext,
        user_options -> Blob,
        user_touched -> Varchar,
        user_token -> Varchar,
        user_email_authenticated -> Nullable<Binary>,
        user_email_token -> Nullable<Binary>,
        user_email_token_expires -> Nullable<Binary>,
        user_registration -> Nullable<Binary>,
        user_newpass_time -> Nullable<Binary>,
        user_editcount -> Nullable<Integer>,
    }
}

table! {
    mw_user_groups (ug_user, ug_group) {
        ug_user -> Unsigned<Integer>,
        ug_group -> Varbinary,
    }
}

table! {
    notes (note_id) {
        note_id -> Integer,
        note_note -> Nullable<Mediumtext>,
    }
}

table! {
    primary_verifications (verification_id) {
        verification_id -> Integer,
        pub_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        ver_time -> Nullable<Datetime>,
        ver_transient -> Nullable<Tinyint>,
    }
}

table! {
    pseudonyms (pseudo_id) {
        pseudo_id -> Integer,
        author_id -> Nullable<Integer>,
        pseudonym -> Nullable<Integer>,
    }
}

table! {
    publishers (publisher_id) {
        publisher_id -> Integer,
        publisher_name -> Nullable<Mediumtext>,
        publisher_wikipedia -> Nullable<Mediumtext>,
        note_id -> Nullable<Integer>,
    }
}

table! {
    pubs (pub_id) {
        pub_id -> Integer,
        pub_title -> Nullable<Mediumtext>,
        pub_tag -> Nullable<Varchar>,
        pub_year -> Nullable<Date>,
        publisher_id -> Nullable<Integer>,
        pub_pages -> Nullable<Varchar>,
        pub_ptype -> Nullable<Varchar>,
        pub_ctype -> Nullable<Enum>,
        pub_isbn -> Nullable<Varchar>,
        pub_frontimage -> Nullable<Mediumtext>,
        pub_price -> Nullable<Varchar>,
        note_id -> Nullable<Integer>,
        pub_series_id -> Nullable<Integer>,
        pub_series_num -> Nullable<Varchar>,
        pub_catalog -> Nullable<Mediumtext>,
    }
}

table! {
    pub_authors (pa_id) {
        pa_id -> Integer,
        pub_id -> Nullable<Integer>,
        author_id -> Nullable<Integer>,
    }
}

table! {
    pub_content (pubc_id) {
        pubc_id -> Integer,
        title_id -> Nullable<Integer>,
        pub_id -> Nullable<Integer>,
        pubc_page -> Nullable<Varchar>,
    }
}

table! {
    pub_series (pub_series_id) {
        pub_series_id -> Integer,
        pub_series_name -> Nullable<Mediumtext>,
        pub_series_wikipedia -> Nullable<Mediumtext>,
        pub_series_note_id -> Nullable<Integer>,
    }
}

table! {
    reference (reference_id) {
        reference_id -> Integer,
        reference_label -> Nullable<Mediumtext>,
        reference_fullname -> Nullable<Mediumtext>,
        pub_id -> Nullable<Integer>,
        reference_url -> Nullable<Mediumtext>,
    }
}

table! {
    reports (row_id) {
        row_id -> Integer,
        report_id -> Nullable<Integer>,
        report_param -> Nullable<Integer>,
        report_data -> Nullable<Mediumtext>,
    }
}

table! {
    series (series_id) {
        series_id -> Integer,
        series_title -> Nullable<Mediumtext>,
        series_parent -> Nullable<Integer>,
        series_type -> Nullable<Integer>,
        series_parent_position -> Nullable<Integer>,
        series_note_id -> Nullable<Integer>,
    }
}

table! {
    sfe3_authors (sfe3_authors_id) {
        sfe3_authors_id -> Integer,
        url -> Nullable<Text>,
        author_name -> Nullable<Text>,
        resolved -> Nullable<Integer>,
    }
}

table! {
    submissions (sub_id) {
        sub_id -> Integer,
        sub_state -> Nullable<Enum>,
        sub_type -> Nullable<Integer>,
        sub_data -> Nullable<Mediumtext>,
        sub_time -> Nullable<Datetime>,
        sub_reviewed -> Nullable<Datetime>,
        sub_submitter -> Integer,
        sub_reviewer -> Integer,
        sub_reason -> Nullable<Mediumtext>,
        sub_holdid -> Integer,
        affected_record_id -> Nullable<Integer>,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Integer,
        tag_name -> Nullable<Tinytext>,
        tag_status -> Nullable<Bool>,
    }
}

table! {
    tag_mapping (tagmap_id) {
        tagmap_id -> Integer,
        tag_id -> Nullable<Integer>,
        title_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
    }
}

table! {
    titles (title_id) {
        title_id -> Integer,
        title_title -> Nullable<Mediumtext>,
        title_translator -> Nullable<Mediumtext>,
        title_synopsis -> Nullable<Integer>,
        note_id -> Nullable<Integer>,
        series_id -> Nullable<Integer>,
        title_seriesnum -> Nullable<Integer>,
        title_copyright -> Nullable<Date>,
        title_storylen -> Nullable<Mediumtext>,
        title_ttype -> Nullable<Enum>,
        title_wikipedia -> Nullable<Mediumtext>,
        title_views -> Integer,
        title_parent -> Integer,
        title_rating -> Nullable<Float>,
        title_annualviews -> Integer,
        title_ctl -> Unsigned<Integer>,
        title_language -> Nullable<Integer>,
        title_seriesnum_2 -> Nullable<Varchar>,
        title_non_genre -> Nullable<Enum>,
        title_graphic -> Nullable<Enum>,
        title_nvz -> Nullable<Enum>,
        title_jvn -> Nullable<Enum>,
        title_content -> Nullable<Tinytext>,
    }
}

table! {
    title_awards (taw_id) {
        taw_id -> Integer,
        award_id -> Nullable<Integer>,
        title_id -> Nullable<Integer>,
    }
}

table! {
    title_relationships (tr_id) {
        tr_id -> Integer,
        title_id -> Nullable<Integer>,
        review_id -> Nullable<Integer>,
        series_id -> Nullable<Integer>,
        translation_id -> Nullable<Integer>,
    }
}

table! {
    trans_authors (trans_author_id) {
        trans_author_id -> Integer,
        trans_author_name -> Nullable<Mediumtext>,
        author_id -> Nullable<Integer>,
    }
}

table! {
    trans_legal_names (trans_legal_name_id) {
        trans_legal_name_id -> Integer,
        trans_legal_name -> Nullable<Mediumtext>,
        author_id -> Nullable<Integer>,
    }
}

table! {
    trans_publisher (trans_publisher_id) {
        trans_publisher_id -> Integer,
        trans_publisher_name -> Nullable<Mediumtext>,
        publisher_id -> Nullable<Integer>,
    }
}

table! {
    trans_pubs (trans_pub_id) {
        trans_pub_id -> Integer,
        trans_pub_title -> Nullable<Mediumtext>,
        pub_id -> Nullable<Integer>,
    }
}

table! {
    trans_pub_series (trans_pub_series_id) {
        trans_pub_series_id -> Integer,
        trans_pub_series_name -> Nullable<Mediumtext>,
        pub_series_id -> Nullable<Integer>,
    }
}

table! {
    trans_series (trans_series_id) {
        trans_series_id -> Integer,
        trans_series_name -> Nullable<Mediumtext>,
        series_id -> Nullable<Integer>,
    }
}

table! {
    trans_titles (trans_title_id) {
        trans_title_id -> Integer,
        trans_title_title -> Nullable<Mediumtext>,
        title_id -> Nullable<Integer>,
    }
}

table! {
    user_languages (user_lang_id) {
        user_lang_id -> Integer,
        user_id -> Nullable<Integer>,
        lang_id -> Nullable<Integer>,
        user_choice -> Nullable<Integer>,
    }
}

table! {
    user_preferences (user_pref_id) {
        user_pref_id -> Integer,
        user_id -> Nullable<Integer>,
        concise_disp -> Nullable<Integer>,
        display_all_languages -> Nullable<Enum>,
        default_language -> Nullable<Integer>,
        covers_display -> Nullable<Bool>,
        suppress_translation_warnings -> Nullable<Bool>,
        suppress_bibliographic_warnings -> Nullable<Bool>,
        cover_links_display -> Nullable<Bool>,
        keep_spaces_in_searches -> Nullable<Bool>,
        suppress_help_bubbles -> Nullable<Bool>,
        suppress_awards -> Nullable<Bool>,
        suppress_reviews -> Nullable<Bool>,
        display_post_submission -> Nullable<Bool>,
    }
}

table! {
    user_sites (user_site_id) {
        user_site_id -> Integer,
        site_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        user_choice -> Nullable<Integer>,
    }
}

table! {
    user_status (user_status_id) {
        user_status_id -> Integer,
        user_id -> Nullable<Integer>,
        last_changed_ver_pubs -> Nullable<Datetime>,
        last_viewed_ver_pubs -> Nullable<Datetime>,
    }
}

table! {
    verification (verification_id) {
        verification_id -> Integer,
        pub_id -> Nullable<Integer>,
        reference_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        ver_time -> Nullable<Datetime>,
        ver_status -> Nullable<Integer>,
    }
}

table! {
    votes (vote_id) {
        vote_id -> Integer,
        title_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        rating -> Nullable<Integer>,
    }
}

table! {
    webpages (webpage_id) {
        webpage_id -> Integer,
        author_id -> Nullable<Integer>,
        publisher_id -> Nullable<Integer>,
        url -> Nullable<Mediumtext>,
        pub_series_id -> Nullable<Integer>,
        title_id -> Nullable<Integer>,
        award_type_id -> Nullable<Integer>,
        series_id -> Nullable<Integer>,
        award_cat_id -> Nullable<Integer>,
        pub_id -> Nullable<Integer>,
    }
}

table! {
    websites (site_id) {
        site_id -> Integer,
        site_name -> Nullable<Tinytext>,
        site_url -> Nullable<Mediumtext>,
        site_isbn13 -> Nullable<Tinyint>,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    authors_by_debut_date,
    awards,
    award_cats,
    award_titles_report,
    award_types,
    bad_images,
    canonical_author,
    changed_verified_pubs,
    cleanup,
    directory,
    emails,
    history,
    identifiers,
    identifier_sites,
    identifier_types,
    languages,
    license_keys,
    magazine,
    most_reviewed,
    mw_user,
    mw_user_groups,
    notes,
    primary_verifications,
    pseudonyms,
    publishers,
    pubs,
    pub_authors,
    pub_content,
    pub_series,
    reference,
    reports,
    series,
    sfe3_authors,
    submissions,
    tags,
    tag_mapping,
    titles,
    title_awards,
    title_relationships,
    trans_authors,
    trans_legal_names,
    trans_publisher,
    trans_pubs,
    trans_pub_series,
    trans_series,
    trans_titles,
    user_languages,
    user_preferences,
    user_sites,
    user_status,
    verification,
    votes,
    webpages,
    websites,
);
