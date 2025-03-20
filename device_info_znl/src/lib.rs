pub mod modules;

#[cfg(test)]
mod tests {
    use crate::modules::hostname::get_hostname;

    #[test]
    fn test_get_hostname() {
        let hostname = get_hostname();
        println!("Hostname: {}", hostname);
        assert!(!hostname.is_empty(), "Hostname should not be empty");
    }
}
