-- This file should undo anything in `up.sql`
alter table measurements 
  modify column
    comment tinytext not null;
