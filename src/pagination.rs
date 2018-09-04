#[derive(FromForm, Serialize)]
pub struct PaginationParams {
    pub page: u32,
    pub items_per_page: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> PaginationParams {
        PaginationParams {
            page: 1,
            items_per_page: Some(5),
        }
    }
}

#[derive(Serialize)]
pub struct PaginationContext {
    current: u32,
    pages: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<u32>,
}

impl PaginationParams {
    pub fn context(self, total_items: u32) -> PaginationContext {
        let mut total_pages = total_items / self.items_per_page.unwrap_or(5);
        if total_items % self.items_per_page.unwrap_or(5) > 0 {
            total_pages += 1;
        } else if total_pages == 0 {
            total_pages = 1;
        }

        PaginationContext {
            current: if self.page < 1 { 1 }
                     else if self.page >= total_pages { total_pages }
                     else { self.page },
            pages: (1..total_pages + 1).collect::<Vec<u32>>(),
            prev: if self.page <= 1 {
                None
            } else {
                Some(self.page - 1)
            },
            next: if self.page < total_pages {
                Some(self.page + 1)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check_context(page: u32, item_per_page: u32, total_items: u32) -> PaginationContext {
        let pagination = PaginationParams {
            page: page,
            items_per_page: item_per_page,
        };
        pagination.context(total_items)
    }
    
    #[test]
    fn one_page_has_no_next_no_prev() {
        let context = check_context(1, 5, 4);
        assert!(context.next.is_none());
        assert!(context.prev.is_none());
        let context = check_context(1, 5, 5);
        assert!(context.next.is_none());
        assert!(context.prev.is_none());
    }

    #[test]
    fn pages_first_has_next_but_no_prev() {
        let context = check_context(1, 5, 6);
        assert_eq!(context.next, Some(2));
        assert!(context.prev.is_none());
    }

    #[test]
    fn pages_last_has_prev_but_no_next() {
        let context = check_context(2, 5, 6);
        assert_eq!(context.prev, Some(1));
        assert!(context.next.is_none());
        let context = check_context(4, 5, 18);
        assert_eq!(context.prev, Some(3));
        assert!(context.next.is_none());
    }

    #[test]
    fn pages_middle_has_prev_and_next() {
        let context = check_context(3, 5, 18);
        assert_eq!(context.next, Some(4));
        assert_eq!(context.prev, Some(2));
        let context = check_context(2, 5, 15);
        assert_eq!(context.next, Some(3));
        assert_eq!(context.prev, Some(1));
    }

    #[test]
    fn current_stays_unchanged_inside_bounds() {
        assert_eq!(check_context(3, 5, 18).current, 3);
        assert_eq!(check_context(1, 5, 18).current, 1);
        assert_eq!(check_context(1, 5, 5).current, 1);
        assert_eq!(check_context(1, 5, 0).current, 1);
    }

    #[test]
    fn current_set_to_first_when_outside_bounds_before() {
        assert_eq!(check_context(0, 5, 18).current, 1);
        assert_eq!(check_context(0, 5, 5).current, 1);
        assert_eq!(check_context(0, 5, 0).current, 1);
    }
    
    #[test]
    fn current_set_to_last_when_outside_bounds_after() {
        assert_eq!(check_context(6, 5, 18).current, 4);
        assert_eq!(check_context(2, 5, 5).current, 1);
        assert_eq!(check_context(2, 5, 0).current, 1);
    }

    #[test]
    fn page_list_has_all_values() {
        assert_eq!(check_context(2, 5, 18).pages, [1, 2, 3, 4]);
        assert_eq!(check_context(1, 5, 6).pages, [1, 2]);
        assert_eq!(check_context(1, 5, 1).pages, [1]);
        assert_eq!(check_context(1, 5, 5).pages, [1]);
        assert_eq!(check_context(1, 5, 0).pages, [1]);
    }
}
