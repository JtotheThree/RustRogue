macro_rules! add_sub_i32 {
    ($a:expr, $b:expr) => {
        if $b < 0 {
            if $a > 0 {
                $a - (-$b) as u32
            } else {
                $a
            }
        } else {
            if $a == <u32>::max_value() {
                $a
            } else {
                $a + $b as u32
            }
        }
    };
}