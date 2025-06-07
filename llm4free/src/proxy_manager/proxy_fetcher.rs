use std::collections::VecDeque;

use crate::proxy_manager::proxy_data::ProxyData;

pub trait ProxyFetcher {
  fn fetch_proxies(&self) -> VecDeque<ProxyData>;
}