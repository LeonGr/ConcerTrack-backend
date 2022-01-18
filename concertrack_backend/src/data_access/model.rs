use super::schema::tracked;

#[derive(Queryable, Debug)]
pub struct TrackEntry {
    pub id: i32,
    pub code: String,
    pub artist: String,
}


#[derive(Insertable)]
#[table_name="tracked"]
pub struct NewEntry {
    pub code: String,
    pub artist: String,
}
