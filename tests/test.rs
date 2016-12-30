extern crate teleborg;

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Read;

    use teleborg::updater::*;

    #[test]
    fn create_updater() {
        let updater = Updater::new(None);
        println!("{:?}", updater);
        assert!(updater.is_ok());
    }
}
