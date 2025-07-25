use leptos::prelude::*;
use leptos_router::params::Params;

#[derive(Debug, Params, PartialEq, Clone)]
pub struct Pagination {
    tag: Option<String>,
    my_feed: Option<bool>,
    page: Option<u32>,
    amount: Option<u32>,
}

impl Pagination {
    #[inline]
    pub fn get_tag(&self) -> &str {
        self.tag.as_deref().unwrap_or_default()
    }
    #[inline]
    pub fn get_my_feed(&self) -> bool {
        self.my_feed.unwrap_or_default()
    }
    #[inline]
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or_default()
    }
    #[inline]
    pub fn get_amount(&self) -> u32 {
        let per_page = expect_context::<RwSignal<Option<u32>>>();
        self.amount.unwrap_or(per_page.get_untracked().unwrap())
    }

    #[inline]
    pub fn set_tag<T: ToString + ?Sized>(mut self, tag: &T) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    #[inline]
    pub fn set_amount(mut self, amount: u32) -> Self {
        self.amount = Some(amount);
        self
    }

    #[inline]
    pub fn set_my_feed(mut self, feed: bool) -> Self {
        self.my_feed = Some(feed);
        self
    }

    #[inline]
    pub fn reset_page(mut self) -> Self {
        self.page = Some(0);
        self
    }

    #[inline]
    pub fn next_page(mut self) -> Self {
        self.page = Some(self.page.unwrap_or_default().saturating_add(1));
        self
    }

    #[inline]
    pub fn previous_page(mut self) -> Self {
        self.page = Some(self.page.unwrap_or_default().saturating_sub(1));
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        let per_page = expect_context::<RwSignal<Option<u32>>>();
        Self {
            tag: Some(String::new()),
            my_feed: Some(false),
            page: Some(0),
            amount: Some(per_page.get_untracked().unwrap_or(10)),
        }
    }
}

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/?tag={}&my_feed={}&page={}&amount={}",
            self.get_tag(),
            self.get_my_feed(),
            self.get_page(),
            self.get_amount(),
        )
    }
}
