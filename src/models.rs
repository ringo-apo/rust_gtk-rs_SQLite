use super::schema::memos;

#[derive(Queryable)]
pub struct Memo {
    pub id: i32,
    pub comment: String,
}

#[derive(Insertable)]
#[table_name = "memos"]
pub struct NewMemo<'a> {
    pub comment: &'a str,
}
