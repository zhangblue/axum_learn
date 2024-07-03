use chrono::{NaiveDateTime};

pub fn date_time_to_date_format(date_time: NaiveDateTime, format: FormatType) -> String {
    date_time.format(&format.format()).to_string()
}

pub fn date_time_to_date_format_default(date_time: NaiveDateTime) -> String {
    date_time_to_date_format(date_time, FormatType::YyyyMmDdHhmmSs)
}


pub enum FormatType {
    YyyyMmDdHhmmSs,
    YyyyMmDd,
}

impl FormatType {
    fn format(self) -> String {
        match self {
            FormatType::YyyyMmDdHhmmSs => {
                String::from("%Y-%m-%d %H:%M:%S")
            }
            FormatType::YyyyMmDd => {
                String::from("%Y-%m-%d")
            }
        }
    }
}