use std::collections::VecDeque;

use crate::proxy_manager::{proxy_data::ProxyData, proxy_fetcher::ProxyFetcher};

pub mod shader_proxy_fetcher;
pub mod proxy_data;
pub mod proxy_fetcher;
pub mod fetchers_implementations;

pub struct ProxyManager<TProxyFetcher : ProxyFetcher> {
  pub max_errors: u32,
  pub stored_proxies: VecDeque<ProxyData>,
  pub proxy_fetcher: TProxyFetcher
}

pub struct TooManyAttemptsError {}

impl<TProxyFetcher : ProxyFetcher> ProxyManager<TProxyFetcher> {
  pub fn get_next(&mut self) -> Option<&ProxyData> {
    while let Some(proxy) = self.stored_proxies.pop_back() {
      if proxy.errors >= self.max_errors {continue;}
      self.stored_proxies.push_front(proxy);
      return self.stored_proxies.front()
    }
    None
  }

  pub fn request_proxy(&mut self, max_attempts: u32) -> Result<&ProxyData, TooManyAttemptsError> {
    let mut attempts: u32 = 0;
    loop {
      if let Some(proxy) = self.get_next() {
        return Ok(proxy);
      }
      self.refetch_proxies();
      attempts += 1;
      if attempts >= max_attempts {
        return Err(TooManyAttemptsError {})
      }
    }
  }

  pub fn refetch_proxies(&mut self) {
    self.stored_proxies = self.proxy_fetcher.fetch_proxies().into_iter()
      .map(|proxy| ProxyData::new(proxy))
      .collect();
  }
}