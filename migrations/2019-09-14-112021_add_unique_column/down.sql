-- This file should undo anything in `up.sql`
alter table measurements
  drop column my_ref;
