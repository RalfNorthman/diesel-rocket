alter table measurements 
  change column temp_celsius 
    temperature double not null 
      comment 'Unit: degrees centigrade (Celsius).',
  modify column
    humidity double not null 
      comment 'Unit: percent (relative humidity).',
  add column
    pressure double not null 
      comment 'Unit: Pascal (Pa, SI-unit).'
      after humidity
