mod filter;
mod pagination;
mod sort;

pub use filter::*;
pub use pagination::*;
pub use sort::*;

pub struct QuerySpec {
    pub filters: Vec<Filter>,
    pub sorting: Vec<SortFiled>,
    pub pagination: Option<Pagination>,
}

impl QuerySpec {
    pub fn new() -> Self {
        Self {
            filters: vec![],
            sorting: vec![],
            pagination: None,
        }
    }
}

impl QuerySpec {
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn with_sort(mut self, field: impl Into<String>, direction: SortDirection) -> Self {
        self.sorting.push(SortFiled {
            field: field.into(),
            direction,
        });
        self
    }

    pub fn with_pagination(mut self, page: u64, per_page: u64) -> Self {
        self.pagination = Some(Pagination::new(page, per_page));
        self
    }
}
