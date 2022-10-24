use std::path::Path;

use cucumber::{given, then, when, World as _};

#[derive(Clone, Debug, Default, cucumber::World)]
pub struct World(pub Vec<usize>);

#[given(regex = "^I stored (\\d+)$")]
#[when(regex = "^I store (\\d+)$")]
pub fn store(w: &mut World, value: usize) {
    w.0.push(value)
}

#[then(regex = "^I see (\\d+)(?:, (\\d+))?(?: and (\\d+))?$")]
pub fn assert(w: &mut World, value_1: String, value_2: String, value_3: String) {
    assert!([value_1, value_2, value_3]
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .eq(w.0.iter().copied()))
}

pub async fn run(path: impl AsRef<Path>) {
    World::run(path).await;
}
