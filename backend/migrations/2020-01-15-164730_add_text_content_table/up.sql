CREATE TABLE text_content (
    tag TEXT NOT NULL,
    lang TEXT NOT NULL,
    text TEXT NOT NULL,
    PRIMARY KEY (tag, lang)
);
