mod fs_tests;

use fs_tests::test_fs;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn main() {
        test_fs();
    }
}
