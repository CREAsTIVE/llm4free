pub trait ProxyFetcher {
  /// Returns list of fetched proxies from some source (could be constant)
  fn fetch_proxies(&self) -> Vec<String>;
}