-- Your SQL goes here
alter table measurements
  add column my_ref bigint unsigned unique;
