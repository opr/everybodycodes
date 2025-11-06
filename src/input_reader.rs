pub mod input_reader {
    use std::fmt::Display;
    use std::fs;
    pub fn read_input_for_day( year: i32, day: impl Display, test: bool ) -> String {
        let day_num = day.to_string();
        let mut suffix = "";
        if test {
            suffix = "_test"
        }
        let file_path = format!("inputs/{}/{}{}.txt", year, day_num, suffix );
        let contents = fs::read_to_string(file_path)
            .expect(format!("Should have been able to read the file, {}", format!("inputs/{}/{}{}.txt", year, day_num, suffix )).as_str());

        contents
    }
}