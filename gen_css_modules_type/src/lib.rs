use regex::Regex;

pub const TEST_CSS: &str = "
    .sup { \
    } \
    .dude { \
    } \
";

pub fn parse_rules(css: &str) -> Vec<&str> {
  let rule_pattern = Regex::new(r"\.\w+").unwrap();
  rule_pattern
    .find_iter(&css)
    .map(|match_| match_.as_str())
    .collect()
}
