alter table measurements 
  change column temperature
    temp_celsius double not null,
  modify column
    humidity double not null, 
  drop column pressure 
