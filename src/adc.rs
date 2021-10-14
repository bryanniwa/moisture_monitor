use std::fs;
use std::error::Error;

pub fn read_adc(path: &str) -> Result<u64, Box<dyn Error>> {
    let adc_value = fs::read_to_string(path)?;
    let res = adc_value.trim().parse::<u64>()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn successful_read() {
        use std::io::Write;

        let filename = "test.txt";

        let mut file = fs::File::create(filename).unwrap();
        file.write(b"1234").unwrap();

        let res = read_adc(filename).unwrap();
        
        fs::remove_file(filename).unwrap();

        assert_eq!(1234, res, "adc value should read 1234")
    }

    #[test]
    fn file_does_not_exist() {
        assert!(read_adc("test2.txt").is_err(), "adc file should not exist")
    }

    #[test]
    fn wrong_file_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let filename = "test3.txt";

        fs::File::create(filename).unwrap();
        let mut perms = fs::metadata(filename).unwrap().permissions();
        perms.set_mode(0o000);

        let res = read_adc(filename);

        perms.set_mode(00644);
        fs::remove_file(filename).unwrap();

        assert!(res.is_err(), "adc file should have improper permissions")
    }
}