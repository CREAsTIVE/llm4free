pub struct ProxyData {
  pub link: String,
  pub errors: u32
}

impl ProxyData {
  pub fn new(link: impl Into<String>) -> Self {
    Self {
      link: link.into(),
      errors: 0
    }
  }
}