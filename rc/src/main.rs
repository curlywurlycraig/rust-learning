mod lifetime;
mod rc;

use lifetime::{demo as lt_demo};
use rc::{demo as rc_demo};

fn main() {
    lt_demo();
    rc_demo();
}