create table measurements (
  id bigint unsigned not null auto_increment primary key,
  temp_celsius double not null,
  humidity double not null,
  comment tinytext
)
