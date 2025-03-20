mod collection;
mod exercises;

use crate::collection::vectors::vectors;
use crate::collection::hashmaps::hashmaps;
use crate::collection::strings::strings;
use crate::exercises::medianmode;
use crate::exercises::piglatin;

fn main() {
    vectors();
    strings();
    hashmaps();

    medianmode::median();
    medianmode::mode();
    piglatin::piglatin();
}
