CREATE TABLE raw_txn
(
    signature varchar(64) PRIMARY KEY,
    slot      bigint not null,
    processed bool   not null
);

CREATE INDEX raw_slot on raw_txn (slot);

CREATE TABLE cl_items
(
    id       bigserial PRIMARY KEY,
    tree     BYTEA  NOT NULL,
    node_idx BIGINT NOT NULL,
    leaf_idx BIGINT,
    seq      BIGINT NOT NULL,
    level    BIGINT NOT NULL,
    hash     BYTEA  NOT NULL
);
-- Index All the things space is cheap
CREATE INDEX cl_items_tree_idx on cl_items (tree);
CREATE INDEX cl_items_hash_idx on cl_items (hash);
CREATE INDEX cl_items_level on cl_items (level);
CREATE INDEX cl_items_node_idx on cl_items (node_idx);
CREATE INDEX cl_items_leaf_idx on cl_items (leaf_idx);
CREATE UNIQUE INDEX cl_items__tree_node on cl_items (tree, node_idx);

CREATE TABLE backfill_items
(
    id          bigserial PRIMARY KEY,
    tree        BYTEA  NOT NULL,
    seq         BIGINT NOT NULL,
    slot        BIGINT NOT NULL,
    force_chk   bool,
    backfilled  bool
);

CREATE INDEX backfill_items_tree_idx on backfill_items (tree);
CREATE INDEX backfill_items_seq_idx on backfill_items (seq);
CREATE INDEX backfill_items_slot_idx on backfill_items (slot);
CREATE INDEX backfill_items_force_chk_idx on backfill_items (force_chk);
CREATE INDEX backfill_items_backfilled_idx on backfill_items (backfilled);
CREATE INDEX backfill_items_tree_seq_idx on backfill_items (tree, seq);
CREATE INDEX backfill_items_tree_slot_idx on backfill_items (tree, slot);
CREATE INDEX backfill_items_tree_force_chk_idx on backfill_items (tree, force_chk);

CREATE or REPLACE FUNCTION notify_new_backfill_item()
    RETURNS trigger
     LANGUAGE 'plpgsql'
as $BODY$
declare
begin
    if (tg_op = 'INSERT') then
        perform pg_notify('backfill_item_added', 'hello');
    end if;

    return null;
end
$BODY$;

CREATE TRIGGER after_insert_item
    AFTER INSERT
    ON backfill_items
    FOR EACH ROW
    EXECUTE PROCEDURE notify_new_backfill_item();

-- START NFT METADATA
CREATE TYPE owner_type AS ENUM ('unknown', 'token', 'single');
CREATE TYPE royalty_target_type AS ENUM ('unknown', 'creators', 'fanout', 'single');
CREATE TYPE chain_mutability AS ENUM ('unknown', 'mutable', 'immutable');
CREATE TYPE mutability AS ENUM ('unknown', 'mutable', 'immutable');

create table asset_data
(
    id                    bigserial PRIMARY KEY,
    chain_data_mutability chain_mutability not null default 'mutable',
    schema_version        int              not null default 1,
    chain_data            jsonb            not null,
    metadata_url          varchar(200)     not null,
    metadata_mutability   mutability       not null default 'mutable',
    metadata              jsonb            not null
);

create table asset
(
    id                    bytea PRIMARY KEY,
    specification_version int                 not null default 1,
    owner                 bytea               not null,
    owner_type            owner_type          not null default 'single',
    -- delegation
    delegate              bytea,
    -- freeze
    frozen                bool                not null default false,
    -- supply
    supply                bigint              not null default 1,
    supply_mint           bytea,
    -- compression
    compressed            bool                not null default false,
    seq                   bigint              not null,
    -- -- Can this asset be compressed
    compressible          bool                not null default false,
    tree_id               bytea,
    leaf                  bytea,
    nonce                 bigint              not null,
    -- royalty
    royalty_target_type   royalty_target_type not null default 'creators',
    royalty_target        bytea,
    royalty_amount        int                 not null default 0,
    -- data
    chain_data_id         bigint references asset_data (id),
    -- visibility
    created_at            timestamp with time zone     default (now() at time zone 'utc'),
    burnt                 bool                not null default false
);

create index asset_tree on asset (tree_id);
create index asset_leaf on asset (leaf);
create index asset_tree_leaf on asset (tree_id, leaf);
create index asset_revision on asset (tree_id, leaf, nonce);
create index asset_owner on asset (owner);
create index asset_delegate on asset (delegate);

-- grouping
create table asset_grouping
(
    id          bigserial PRIMARY KEY,
    asset_id    bytea references asset (id) not null,
    group_key   text                        not null,
    group_value text                        not null,
    seq         bigint                      not null
);
-- Limit indexable grouping keys, meaning only create on specific keys, but index the ones we allow
create unique index asset_grouping_asset_id on asset_grouping (asset_id);
create index asset_grouping_key on asset_grouping (group_key, group_value);
create index asset_grouping_value on asset_grouping (group_key, asset_id);

-- authority
create table asset_authority
(
    id        bigserial PRIMARY KEY,
    asset_id  bytea references asset (id) not null,
    scopes    text[],
    authority bytea                       not null,
    seq       bigint                      not null
);
create unique index asset_authority_asset_id on asset_authority (asset_id);
create index asset_authority_idx on asset_authority (asset_id, authority);

-- creators
create table asset_creators
(
    id       bigserial PRIMARY KEY,
    asset_id bytea references asset (id) not null,
    creator  bytea                       not null,
    share    int                         not null default 0,
    verified bool                        not null default false,
    seq      bigint                      not null
);
create unique index asset_creators_asset_id on asset_creators (asset_id);
create index asset_creator on asset_creators (asset_id, creator);
create index asset_verified_creator on asset_creators (asset_id, verified);

create type whitelist_mint_mode AS ENUM ('burn_every_time', 'never_burn');
create type end_setting_type AS ENUM ('date', 'amount');

create table candy_machine_data
(
    id                         bigserial        PRIMARY KEY,
    uuid                       varchar(6)       not null,
    price                      int              not null,
    symbol                     varchar(5)       not null,
    seller_fee_basis_points    int              not null,
    max_supply                 int              not null,
    is_mutable                 bool             not null,
    retain_authority           bool             not null,
    go_live_date               int,
    items_available            int              not null,
);

create table candy_machine_state
(
    id                       bigserial           PRIMARY KEY,
    candy_machine_data_id    bigint references candy_machine_data (id),
    authority                bytea               not null,
    wallet                   bytea               not null,
    token_mint               bytea,
    items_redeemdd           int                 not null               
);

create table candy_machine_creators
(
    id                    bigserial                                PRIMARY KEY,
    candy_machine_data_id bigint references candy_machine_data (id) not null,
    creator               bytea                                    not null,
    share                 int                                      not null default 0,
    verified              bool                                     not null default false
);
create unique index candy_machine_creators_candy_machine_data_id on candy_machine_creators (candy_machine_data_id);
create index candy_machine_creator on candy_machine_creators (candy_machine_data_id, creator);
create index candy_machine_verified_creator on candy_machine_creators (candy_machine_data_id, verified);

create table candy_machine_whitelist_mint_settings
(
    id                    bigserial                                PRIMARY KEY,
    candy_machine_data_id bigint references candy_machine_data (id) not null,
    mode                  whitelist_mint_mode                      not null,
    mint                  bytea                                    not null,
    presale               bool                                     not null,
    discount_price        int
) 
create unique index candy_machine_whitelist_mint_settings_candy_machine_data_id on candy_machine_whitelist_mint_settings (candy_machine_data_id);


create table candy_machine_hidden_settings
(
    id                    bigserial                                PRIMARY KEY,
    candy_machine_data_id bigint references candy_machine_data (id) not null,
    name                  varchar(50)                              not null,
    uri                   varchar(200)                             not null,
    hash                  bytea                                    not null
) 
create unique index candy_machine_hidden_settings_candy_machine_data_id on candy_machine_hidden_settings (candy_machine_data_id);


create table candy_machine_end_settings
(
    id                    bigserial                                PRIMARY KEY,
    candy_machine_data_id bigint references candy_machine_data (id) not null,
    number                int                                      not null,
    end_setting_type      end_setting_type                         not null
    
) 
create unique index candy_machine_end_settings_candy_machine_data_id on candy_machine_end_settings (candy_machine_data_id);

create table candy_machine_gatekeeper
(
    id                    bigserial                                PRIMARY KEY,
    candy_machine_data_id bigint references candy_machine_data (id) not null,
    gatekeeper_network    bytea                                    not null,
    expire_on_use         bool                                     not null
    
) 
create unique index candy_machine_end_settings_candy_machine_data_id on candy_machine_end_settings (candy_machine_data_id);


