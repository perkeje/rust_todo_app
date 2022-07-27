#[derive(serde::Serialize)]
pub struct Paginated<T> {
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
    pub last_page: u32,
    pub data: Vec<T>,
}
