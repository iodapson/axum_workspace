create table some_table (
  column_1 varchar not null,
  column_2 varchar not null
);

create unique index column_2_idx on some_table(column_2);