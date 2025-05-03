CREATE TABLE fishes (
    fish_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    min_length INTEGER NOT NULL,
    max_length INTEGER NOT NULL,
    rarety_catagory INTEGER NOT NULL CHECK (rarety_catagory BETWEEN 1 AND 5),
    rarety_factor FLOAT NOT NULL,
    sprite TEXT NOT NULL
);

CREATE TABLE rot (
    rot_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    distance INTEGER NOT NULL,
    durability INTEGER NOT NULL,
    sprite TEXT NOT NULL,
    price_coins INTEGER,
    price_bucks INTEGER
);

CREATE TABLE bait (
    bait_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    durability INTEGER NOT NULL,
    sprite TEXT NOT NULL,
    price_coins INTEGER,
    price_bucks INTEGER,
    bait_type UUID NOT NULL REFERENCES bait_types(id)
);

CREATE TABLE bait_types (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE fish_preferences (
    fish_id UUID NOT NULL REFERENCES fishes(fish_id),
    bait_id UUID NOT NULL REFERENCES bait(bait_id),
    likes BOOLEAN NOT NULL,
    PRIMARY KEY (fish_id, bait_id)
);

CREATE TABLE fish_caught (
    user_id UUID NOT NULL REFERENCES users(user_id),
    fish_id UUID NOT NULL REFERENCES fishes(fish_id),
    max_length UUID NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY (user_id, fish_id)
);

CREATE TABLE fish_places_caught (
    user_id UUID NOT NULL REFERENCES users(user_id),
    fish_id UUID NOT NULL REFERENCES fishes(fish_id),
    place_id UUID NOT NULL REFERENCES places(place_id),
    PRIMARY KEY (user_id, fish_id, place_id)
);

CREATE TABLE places (
    place_id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE inventory (
    user_id UUID NOT NULL REFERENCES users(user_id),
    item_id UUID NOT NULL,
    amount INTEGER NOT NULL,
    cell_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, item_id)
);

CREATE TABLE mail (
    sender_id UUID NOT NULL REFERENCES users(user_id),
    receiver_id UUID NOT NULL REFERENCES users(user_id),
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    send_time DATE NOT NULL,
    read_time DATE
);

CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    sault TEXT NOT NULL,
    created DATE NOT NULL
);
