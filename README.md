# rust_gtk-rs_SQLite

# Create SQLite table

```
sqlite3 sample.db

CREATE TABLE memos (id INTEGER PRIMARY KEY AUTOINCREMENT, comment);
insert into memos (comment) values ('comment1');
insert into memos (comment) values ('banana');
insert into memos (comment) values ('ichigo');
select * from memos;
.exit
```

# build & run

```
cargo build
cargo run
```
