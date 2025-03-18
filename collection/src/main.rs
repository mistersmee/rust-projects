mod collection;
mod exercises;

use crate::collection::vectors::vectors;
use crate::collection::hashmaps::hashmaps;
use crate::collection::strings::strings;
use crate::exercises::medianmode;

fn main() {
    vectors();
    strings();
    hashmaps();

    medianmode::median();
    medianmode::mode();
}
