CREATE TABLE
    app_user (
        user_id TEXT PRIMARY KEY,
        handle_name TEXT NOT NULL,
        screen_name TEXT NOT NULL,
        slack_id TEXT,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        is_member BOOLEAN NOT NULL DEFAULT TRUE,
        graduation_date DATE,
        remarks TEXT
    );

CREATE TABLE
    product_user (
        product_id UUID NOT NULL REFERENCES product (product_id) ON DELETE CASCADE,
        user_id TEXT NOT NULL REFERENCES app_user (user_id) ON DELETE CASCADE,
        PRIMARY KEY (product_id, user_id)
    );

CREATE TABLE
    private_item (
        private_item_id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        owner_id TEXT REFERENCES app_user (user_id),
        post_grad_treat_id TEXT,
        model_number TEXT,
        is_remaining BOOLEAN NOT NULL DEFAULT TRUE,
        remarks TEXT
    );
