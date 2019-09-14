-- Your SQL goes here
alter table measurements
  modify column
    comment tinytext not null default '';
