use std::collections::VecDeque;

pub trait ProxyFetcher {
  fn fetch_proxies(&self) -> Vec<String>;
}