use cucumber::when;

#[when(regex = "^I add (\\d+) and (\\d+)$")]
fn add(w: &mut steps::world::World, l: usize, r: usize) {
    w.0.push(cucumber_steps_as_library::add(l, r));
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    steps::run("tests/features").await;
}
