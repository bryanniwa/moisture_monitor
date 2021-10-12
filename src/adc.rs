use std::fs;

pub fn read_adc(path: &str) -> u64 {
    let adc_value = fs::read_to_string(path).expect("Unable to read ADC file");
    adc_value
        .trim()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::panic;

    #[test]
    fn successful_read() {
        use std::io::Write;

        let filename = "test.txt";

        let mut file = fs::File::create(filename).unwrap();
        file.write(b"1234").unwrap();

        let res = read_adc(filename);
        
        fs::remove_file(filename).unwrap();

        assert_eq!(1234, res)
    }

    #[test]
    #[should_panic]
    fn file_does_not_exist() {
        read_adc("test2.txt");
    }

    #[test]
    fn wrong_file_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let filename = "test3.txt";

        fs::File::create(filename).unwrap();
        let mut perms = fs::metadata(filename).unwrap().permissions();
        perms.set_mode(0o000);

        let res = panic::catch_unwind(|| {
            read_adc(filename)
        });

        perms.set_mode(00644);
        fs::remove_file(filename).unwrap();

        assert!(res.is_err())
    }
}