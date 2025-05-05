CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    salt TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL
);

CREATE TABLE stats (
    user_id UUID PRIMARY KEY REFERENCES users(user_id),
    xp INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    bucks INTEGER NOT NULL,
    total_playtime INTEGER NOT NULL
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

CREATE TABLE inventory (
    user_id UUID NOT NULL REFERENCES users(user_id),
    item_id INTEGER NOT NULL,
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
