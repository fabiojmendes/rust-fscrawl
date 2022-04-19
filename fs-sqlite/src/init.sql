pragma journal_mode = wal;
pragma synchronous = normal;

create table if not exists visited (
     id integer primary key,
     path text not null unique
 );
