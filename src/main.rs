use std::error::Error;
use pollster::FutureExt;

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    app::run().block_on()
}
