CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    salt TEXT NOT NULL,
    created DATE NOT NULL
);

CREATE TABLE bait_types (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE fish_caught (
    user_id UUID NOT NULL REFERENCES users(user_id),
    fish_id UUID NOT NULL,
    max_length INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY (user_id, fish_id)
);

CREATE TABLE first_fish_caught (
    user_id UUID NOT NULL REFERENCES users(user_id),
    fish_id UUID NOT NULL,
    place_id UUID NOT NULL,
    bait_id UUID NOT NULL,
    PRIMARY KEY (user_id, fish_id, place_id)
);

CREATE TABLE inventory (
    user_id UUID NOT NULL REFERENCES users(user_id),
    item_id UUID NOT NULL,
    amount INTEGER NOT NULL,
    cell_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, item_id)
);

CREATE TABLE mail (
    mail_id UUID PRIMARY KEY,
    sender_id UUID NOT NULL REFERENCES users(user_id),
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    send_time DATE NOT NULL
);

CREATE TABLE mailbox (
    user_id UUID NOT NULL,
    mail_id UUID NOT NULL,
    read BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (user_id, mail_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (mail_id) REFERENCES mail(mail_id)
);

