pub struct Pagination {
    page: u64,
    per_page: u64,
}

impl Pagination {
    pub fn new(page: u64, per_page: u64) -> Self {
        Self { page, per_page }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 25,
        }
    }
}

impl Pagination {
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.per_page
    }

    pub fn limit(&self) -> u64 {
        self.per_page
    }

    pub fn page(&self) -> u64 {
        self.page
    }
}

pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub per_page: u64,
    pub total_items: u64,
}

impl<T> PaginationResult<T> {
    pub fn total_pages(&self) -> u64 {
        (self.total_items + self.per_page - 1) / self.per_page
    }

    pub fn has_next_page(&self) -> bool {
        self.page < self.total_pages()
    }

    pub fn has_prev_page(&self) -> bool {
        self.page > 1
    }

    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> PaginationResult<U> {
        PaginationResult {
            items: self.items.into_iter().map(f).collect::<Vec<U>>(),
            total_items: self.total_items,
            per_page: self.per_page,
            page: self.page,
        }
    }
}
