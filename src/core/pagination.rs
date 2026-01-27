use serde::{Deserialize, Serialize};

use super::ResponseMeta;

#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page.min(100) as i64
    }
}

#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        Self {
            items,
            total,
            page: params.page,
            per_page: params.per_page,
        }
    }

    pub fn total_pages(&self) -> u32 {
        if self.total == 0 {
            1
        } else {
            ((self.total as f64) / (self.per_page as f64)).ceil() as u32
        }
    }

    pub fn meta(&self) -> ResponseMeta {
        ResponseMeta {
            page: Some(self.page),
            per_page: Some(self.per_page),
            total: Some(self.total),
            total_pages: Some(self.total_pages()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let params = PaginationParams {
            page: 1,
            per_page: 20,
        };
        assert_eq!(params.offset(), 0);

        let params = PaginationParams {
            page: 2,
            per_page: 20,
        };
        assert_eq!(params.offset(), 20);

        let params = PaginationParams {
            page: 3,
            per_page: 10,
        };
        assert_eq!(params.offset(), 20);
    }

    #[test]
    fn test_pagination_limit() {
        let params = PaginationParams {
            page: 1,
            per_page: 20,
        };
        assert_eq!(params.limit(), 20);

        let params = PaginationParams {
            page: 1,
            per_page: 150,
        };
        assert_eq!(params.limit(), 100); // capped at 100
    }

    #[test]
    fn test_total_pages() {
        let paginated: Paginated<i32> = Paginated {
            items: vec![],
            total: 100,
            page: 1,
            per_page: 20,
        };
        assert_eq!(paginated.total_pages(), 5);

        let paginated: Paginated<i32> = Paginated {
            items: vec![],
            total: 101,
            page: 1,
            per_page: 20,
        };
        assert_eq!(paginated.total_pages(), 6);
    }
}
