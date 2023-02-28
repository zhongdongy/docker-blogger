use serde::{de::Visitor, Deserialize, Serialize};

#[derive(PartialEq, Copy, Debug)]
pub enum Date {
    _01 = 1,
    _02 = 2,
    _03 = 3,
    _04 = 4,
    _05 = 5,
    _06 = 6,
    _07 = 7,
    _08 = 8,
    _09 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
    _16 = 16,
    _17 = 17,
    _18 = 18,
    _19 = 19,
    _20 = 20,
    _21 = 21,
    _22 = 22,
    _23 = 23,
    _24 = 24,
    _25 = 25,
    _26 = 26,
    _27 = 27,
    _28 = 28,
    _29 = 29,
    _30 = 30,
    _31 = 31,
}

impl From<&str> for Date {
    fn from(value: &str) -> Self {
        match value {
            "1" | "01" => Date::_01,
            "2" | "02" => Date::_02,
            "3" | "03" => Date::_03,
            "4" | "04" => Date::_04,
            "5" | "05" => Date::_05,
            "6" | "06" => Date::_06,
            "7" | "07" => Date::_07,
            "8" | "08" => Date::_08,
            "9" | "09" => Date::_09,
            "10" => Date::_10,
            "11" => Date::_11,
            "12" => Date::_12,
            "13" => Date::_13,
            "14" => Date::_14,
            "15" => Date::_15,
            "16" => Date::_16,
            "17" => Date::_17,
            "18" => Date::_18,
            "19" => Date::_19,
            "20" => Date::_20,
            "21" => Date::_21,
            "22" => Date::_22,
            "23" => Date::_23,
            "24" => Date::_24,
            "25" => Date::_25,
            "26" => Date::_26,
            "27" => Date::_27,
            "28" => Date::_28,
            "29" => Date::_29,
            "30" => Date::_30,
            "31" => Date::_31,
            _ => Date::_01,
        }
    }
}

impl Into<&str> for Date {
    fn into(self) -> &'static str {
        match self {
            Date::_01 => "01",
            Date::_02 => "02",
            Date::_03 => "03",
            Date::_04 => "04",
            Date::_05 => "05",
            Date::_06 => "06",
            Date::_07 => "07",
            Date::_08 => "08",
            Date::_09 => "09",
            Date::_10 => "10",
            Date::_11 => "11",
            Date::_12 => "12",
            Date::_13 => "13",
            Date::_14 => "14",
            Date::_15 => "15",
            Date::_16 => "16",
            Date::_17 => "17",
            Date::_18 => "18",
            Date::_19 => "19",
            Date::_20 => "20",
            Date::_21 => "21",
            Date::_22 => "22",
            Date::_23 => "23",
            Date::_24 => "24",
            Date::_25 => "25",
            Date::_26 => "26",
            Date::_27 => "27",
            Date::_28 => "28",
            Date::_29 => "29",
            Date::_30 => "30",
            Date::_31 => "31",
        }
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        From::<&str>::from(&value)
    }
}

impl Into<String> for Date {
    fn into(self) -> String {
        Into::<&str>::into(self).to_string()
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(Into::<&str>::into(self.clone().to_owned()))
    }
}

impl Clone for Date {
    fn clone(&self) -> Self {
        From::<&str>::from(Into::<&str>::into(*self))
    }
}
struct DateVisitor;

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(From::<String>::from(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(From::<&str>::from(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expect a 2 digit string [01, 31]")
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(DateVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_models_date_conversion() {
        assert_eq!(Date::_01, Date::from("01"));
        assert_eq!(Date::_04, Date::from("4"));
        assert_eq!("22", Into::<&str>::into(Date::_22));
        assert_eq!("01", Into::<&str>::into(Date::_01));
        assert_eq!(Date::_31, Date::_31.clone());
    }
}
