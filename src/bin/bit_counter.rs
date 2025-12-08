// conta os bits
fn count_bits(n: i64) -> u32 {
    let binary_string: String = format!("{:b}", n);

    let total_de_uns: usize = binary_string.chars().filter(|c| *c == '1').count();
    return total_de_uns.try_into().unwrap();
}

fn count_bits_2(n: u64) -> u32 {
    let binary_string: String = format!("{:b}", n);

    let total_de_uns: usize = binary_string.chars().filter(|c| *c == '1').count();
    return total_de_uns.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use std::u64;

    use super::*;

    fn do_test(n: i64, expected: u32) {
        let actual = count_bits(n);
        assert_eq!(
            actual, expected,
            "expected {} but got {} for n = {}",
            expected, actual, n
        );
    }

    #[test]
    fn sample_tests() {
        do_test(0, 0);
        do_test(2, 1);
        do_test(4, 1);
        do_test(7, 3);
        do_test(77231418, 14);
        do_test(12525589, 11);
        do_test(31, 5);
        do_test(417862, 7);
        do_test(89, 4);
        do_test(674259, 10);
        do_test(i64::MAX, 63); //011111111...11111111.

        assert_eq!(count_bits_2(u64::MAX), 64, "",); // 11111111.......
    }
}

fn main() {}
