-- Add down migration script here

ALTER TABLE public_item
    DROP COLUMN purchase_request_id;

DROP TABLE purchase_request;

DROP TABLE purchase_request_status;

DROP TABLE main_user_public_item;

DROP TABLE private_item;

DROP TABLE member;

DROP TABLE post_grad_treat;



