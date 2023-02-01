pub trait PaginatedApi {
    fn get_cursor(&self) -> String;

    fn has_results(&self) -> bool;
}
