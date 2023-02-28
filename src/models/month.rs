use serde::{de::Visitor, Deserialize, Serialize};

#[derive(PartialEq, Copy, Debug)]
pub enum Month {
    JANUARY = 1,
    FEBUARARY = 2,
    MARCH = 3,
    APRIL = 4,
    MAY = 5,
    JUNE = 6,
    JULY = 7,
    AUGUST = 8,
    SEPTEMBER = 9,
    OCTOBER = 10,
    NOVEMBER = 11,
    DECEMBER = 12,
}

impl From<&str> for Month {
    fn from(value: &str) -> Self {
        match value {
            "1" | "01" => Month::JANUARY,
            "2" | "02" => Month::FEBUARARY,
            "3" | "03" => Month::MARCH,
            "4" | "04" => Month::APRIL,
            "5" | "05" => Month::MAY,
            "6" | "06" => Month::JUNE,
            "7" | "07" => Month::JULY,
            "8" | "08" => Month::AUGUST,
            "9" | "09" => Month::SEPTEMBER,
            "10" => Month::OCTOBER,
            "11" => Month::NOVEMBER,
            "12" => Month::DECEMBER,
            _ => Month::JANUARY,
        }
    }
}

impl Into<&str> for Month {
    fn into(self) -> &'static str {
        match self {
            Self::JANUARY => "01",
            Self::FEBUARARY => "02",
            Self::MARCH => "03",
            Self::APRIL => "04",
            Self::MAY => "05",
            Self::JUNE => "06",
            Self::JULY => "07",
            Self::AUGUST => "08",
            Self::SEPTEMBER => "09",
            Self::OCTOBER => "10",
            Self::NOVEMBER => "11",
            Self::DECEMBER => "12",
        }
    }
}

impl From<String> for Month {
    fn from(value: String) -> Self {
        From::<&str>::from(&value)
    }
}

impl Into<String> for Month {
    fn into(self) -> String {
        Into::<&str>::into(self).to_string()
    }
}

impl Serialize for Month {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(Into::<&str>::into(self.clone().to_owned()))
    }
}

impl Clone for Month {
    fn clone(&self) -> Self {
        From::<&str>::from(Into::<&str>::into(*self))
    }
}
struct MonthVisitor;

impl<'de> Visitor<'de> for MonthVisitor {
    type Value = Month;
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
        formatter.write_str("expect a 2 digit string [01, 12]")
    }
}

impl<'de> Deserialize<'de> for Month {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(MonthVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_models_month_conversion() {
        assert_eq!(Month::APRIL, Month::from("04"));
        assert_eq!(Month::APRIL, Month::from("4"));
        assert_eq!("12", Into::<&str>::into(Month::DECEMBER));
        assert_eq!("01", Into::<&str>::into(Month::JANUARY));
        assert_eq!(Month::MARCH, Month::MARCH.clone());
    }
}
