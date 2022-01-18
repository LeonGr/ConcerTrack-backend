#[derive(Queryable)]
pub struct TrackEntry {
    pub id: i32,
    pub code: String,
    pub artist: String,
}
