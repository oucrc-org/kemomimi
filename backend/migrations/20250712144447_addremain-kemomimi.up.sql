-- Add up migration script here
CREATE TABLE
    post_grad_treat (
        treat_id TEXT PRIMARY KEY, -- 処理のユニークID
        treat_name TEXT NOT NULL -- 処理名（未定や回収、寄付等）
    );

CREATE TABLE
    member (
        user_id UUID PRIMARY KEY, -- ユーザーのユニークID
        handle_name TEXT NOT NULL, -- ユーザーのハンドルネーム
        screen_name TEXT UNIQUE NOT NULL, -- ユーザーのスクリーンネーム
        slack_id TEXT UNIQUE, -- ユーザーのSlack ID(通知等に使用)
        is_admin BOOLEAN DEFAULT FALSE NOT NULL, -- 管理者フラグ
        is_member BOOLEAN DEFAULT TRUE NOT NULL, -- 在籍状況
        graduation_date DATE, -- 卒業日
        remarks TEXT -- 備考欄
    );

CREATE TABLE
    private_item (
        private_item_id UUID PRIMARY KEY, -- 私物のユニークID
        name TEXT NOT NULL, -- 製品名
        owner_id UUID REFERENCES member (user_id), -- 所有者（Userへの外部キー）
        post_grad_treat_id TEXT REFERENCES post_grad_treat (treat_id), -- 卒業後の処理（PostGradTreatへの外部キー）
        model_number TEXT, -- 型番
        is_remaining BOOLEAN DEFAULT TRUE NOT NULL, -- 存しているか(廃棄済みや失効済みならFALSE)
        remarks TEXT -- 備考
    );



CREATE TABLE
    main_user_public_item (
        public_item_id UUID REFERENCES public_item (public_item_id), -- 備品のID (PublicItem への外部キー)
        user_id UUID REFERENCES member (user_id), -- メインユーザーのID (User への外部キー)
        PRIMARY KEY (public_item_id, user_id)
    );

CREATE TABLE
    purchase_request_status (
        purchase_request_status_id UUID PRIMARY KEY, -- 状態のユニークID
        status_name TEXT UNIQUE NOT NULL, -- 状態名（例: Pendingなど）
        remarks TEXT -- 備考欄               
    );

CREATE TABLE
    purchase_request (
        purchase_request_id TEXT PRIMARY KEY, -- 申請のユニークID                                                            
        applicant_id UUID REFERENCES member (user_id), -- 申請者のID（Userへの外部キー）                                              
        product_id UUID REFERENCES product (product_id), -- 申請する製品のID                                                            
        cost INT CHECK (cost >= 0), -- 申請時の想定費用                                                            
        status_id UUID REFERENCES purchase_request_status (purchase_request_status_id), -- 購入申請状態ID（Statusへの外部キー、pending(保留)やapproved(承認済み)など） 
        request_date DATE DEFAULT CURRENT_DATE, -- 申請作成日                                                                  
        approval_date DATE, -- 承認日                                                                      
        remarks TEXT -- 備考欄                                                                      
    );


ALTER TABLE public_item
    ADD COLUMN purchase_request_id TEXT REFERENCES purchase_request (purchase_request_id);