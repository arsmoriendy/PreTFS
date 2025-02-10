#[cfg(test)]
mod test {
    struct Setup<'a> {
        monut_path: &'a str,
    }

    impl Default for Setup<'_> {
        fn default() -> Self {
            let ret = Setup {
                monut_path: "mountpoint",
            };
            if let Err(e) = std::fs::create_dir(ret.monut_path) {
                panic!("{e}");
            }
            return ret;
        }
    }

    impl Drop for Setup<'_> {
        fn drop(&mut self) {
            if let Err(e) = std::fs::remove_dir_all(self.monut_path) {
                panic!("{e}");
            }
        }
    }

    #[test]
    fn mount() {
        Setup::default();
    }
}
