CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    salt TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL
);

CREATE TABLE friends (
    user_one_id UUID NOT NULL,
    user_two_id UUID NOT NULL,
    CONSTRAINT friend_order CHECK (user_one_id < user_two_id), -- Also makes sure that a player is not befriend with himself
    CONSTRAINT unique_friend UNIQUE (user_one_id, user_two_id)
);

CREATE TABLE friend_requests (
    user_one_id UUID NOT NULL,
    user_two_id UUID NOT NULL,
    request_sender_id UUID NOT NULL,
    request_created_time TIMESTAMPTZ NOT NULL,
    CONSTRAINT user_order CHECK (user_one_id < user_two_id), -- Also makes sure that a player is not requesting to befriend himself
    CONSTRAINT sender_in_pair CHECK (request_sender_id = user_one_id OR request_sender_id = user_two_id),
    CONSTRAINT unique_friend_request UNIQUE (user_one_id, user_two_id)
);

CREATE TABLE stats (
    user_id UUID PRIMARY KEY REFERENCES users(user_id),
    selected_rod UUID,
    selected_bait UUID,
    xp INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    bucks INTEGER NOT NULL,
    total_playtime BIGINT NOT NULL
);

CREATE TABLE fish_caught (
    user_id UUID NOT NULL REFERENCES stats(user_id),
    fish_id INT NOT NULL,
    amount INT NOT NULL,
    max_length INTEGER NOT NULL,
    first_caught DATE NOT NULL,
    PRIMARY KEY (user_id, fish_id)
);

CREATE TABLE fish_caught_area (
    user_id UUID NOT null,
    fish_id INTEGER NOT NULL,
    area_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, fish_id, area_id) ,
    FOREIGN KEY (user_id, fish_id) REFERENCES fish_caught(user_id, fish_id)
);

CREATE TABLE fish_caught_bait (
    user_id UUID NOT null,
    fish_id INTEGER NOT NULL,
    bait_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, fish_id, bait_id),
    FOREIGN KEY (user_id, fish_id) REFERENCES fish_caught(user_id, fish_id)
);

-- Inventory item start
CREATE TABLE inventory_item (
    user_id UUID NOT NULL REFERENCES users(user_id),
    item_uuid UUID UNIQUE,
    definition_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, item_uuid)
);

CREATE TABLE inventory_item_stack_codec (
    item_uuid UUID PRIMARY KEY REFERENCES inventory_item(item_uuid) ON DELETE CASCADE,
    current_stack INT NOT NULL
);

CREATE TABLE inventory_item_durability_codec (
    item_uuid UUID PRIMARY KEY REFERENCES inventory_item(item_uuid) ON DELETE CASCADE,
    current_durability INT NOT NULL
);
-- Inventory item end

CREATE TABLE mail (
    mail_id UUID PRIMARY KEY,
    sender_id UUID NOT NULL REFERENCES users(user_id),
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    send_time TIMESTAMPTZ NOT NULL
);

CREATE TABLE mailbox (
    user_id UUID NOT NULL,
    mail_id UUID NOT NULL,
    read BOOLEAN NOT NULL DEFAULT FALSE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (user_id, mail_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (mail_id) REFERENCES mail(mail_id)
);

CREATE TABLE player_effects (
    user_id UUID NOT NULL REFERENCES users(user_id),
    item_id INTEGER NOT NULL,
    expiry_time TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id, item_id),
    CONSTRAINT valid_expiry_time CHECK (expiry_time > NOW())
);

CREATE INDEX idx_player_effects_expiry ON player_effects(expiry_time);
CREATE INDEX idx_player_effects_user_id ON player_effects(user_id);

-- The single, globally shared daily quest of Herb. The database is the source of truth:
-- it is generated/rolled over here (see the scheduler in main.rs) and only read by the game.
-- Only ever the current quest is kept; a rollover replaces the row (and its fishes).
CREATE TABLE herb_quest (
    quest_id UUID PRIMARY KEY,
    -- Area enum value (see Area.cs in the game) Herb stands in / the fishes are caught in.
    area_id INTEGER NOT NULL,
    -- Fishcoins paid for handing the quest in (1 coin per requested fish).
    reward_coins INTEGER NOT NULL,
    -- When the quest rolls over to the next one (Herb's next move), in UTC.
    next_move_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

-- The fishes (species + amount) the current Herb quest asks for.
CREATE TABLE herb_quest_fish (
    quest_id UUID NOT NULL REFERENCES herb_quest(quest_id) ON DELETE CASCADE,
    fish_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY (quest_id, fish_id)
);

-- Per-player Herb quest progress. Tracks which quest the player last accepted (saw) and
-- last completed, both by quest UUID. Used to skip Herb's intro and to block double hand-ins.
CREATE TABLE herb_quest_player_state (
    user_id UUID PRIMARY KEY REFERENCES users(user_id),
    last_completed_quest_id UUID,
    last_accepted_quest_id UUID
);

CREATE TABLE missions_completed (
    user_id UUID NOT NULL,
    mission_id SMALLINT NOT NULL,
    PRIMARY KEY (user_id, mission_id)
);

CREATE TABLE missions_started (
    user_id UUID NOT NULL,
    mission_id SMALLINT NOT NULL,
    mission_progress INTEGER NOT NULL,
    PRIMARY KEY (user_id, mission_id)
);
