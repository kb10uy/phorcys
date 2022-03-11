mod app;

use time::OffsetDateTime;

fn main() {
    println!("{:?}", OffsetDateTime::now_local());
}
