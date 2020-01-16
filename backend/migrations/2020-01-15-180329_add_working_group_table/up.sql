CREATE TABLE working_group (
    year INTEGER NOT NULL PRIMARY KEY
);

CREATE TABLE working_group_members (
    id SERIAL NOT NULL PRIMARY KEY,
    member_id INTEGER NOT NULL REFERENCES members(id)
    -- We might add additional fields to this in the future,
    -- things that shouldn't be in the member table
);

CREATE TABLE working_group_membership (
    year INTEGER NOT NULL REFERENCES working_group(year),
    working_group_member_id INTEGER NOT NULL REFERENCES working_group_members(id),
    PRIMARY KEY (year, working_group_member_id)

);
COMMENT ON TABLE working_group_membership IS
'Relational table specifying relationships between working_group and working_group_members.
Necessary since members may be part of multiple working groups';
