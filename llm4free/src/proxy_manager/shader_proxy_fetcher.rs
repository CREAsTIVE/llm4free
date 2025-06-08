use crate::proxy_manager::proxy_fetcher::ProxyFetcher;

pub struct SharedProxyFetcher {
  pub proxy_fetchers: Vec<Box<dyn ProxyFetcher>>
}

impl ProxyFetcher for SharedProxyFetcher {
  fn fetch_proxies(&self) -> Vec<String> {
    self.proxy_fetchers.iter()
      .map(|fetcher| fetcher.fetch_proxies())
      .flatten()
      .collect()
    }
}