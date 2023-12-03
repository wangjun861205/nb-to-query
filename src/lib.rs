#![feature(iter_intersperse)]

use chrono::{DateTime, NaiveDate, TimeZone};
pub use nb_to_query_derive::ToQueryDerive;

pub trait ToQuery {
    fn to_query(&self, field_name: &str) -> Option<String>;
}

impl ToQuery for String {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for &str {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for bool {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for i8 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for i16 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for i32 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for i64 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for i128 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for u8 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for u16 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for u32 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for u64 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for u128 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for f32 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl ToQuery for f64 {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self))
    }
}

impl<T> ToQuery for Vec<T>
where
    T: ToQuery,
{
    fn to_query(&self, field_name: &str) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        let pairs: String = self
            .iter()
            .flat_map(|v| v.to_query(field_name))
            .intersperse("&".into())
            .collect();
        if pairs.is_empty() {
            return None;
        }
        Some(pairs)
    }
}

impl<T> ToQuery for Option<T>
where
    T: ToQuery,
{
    fn to_query(&self, field_name: &str) -> Option<String> {
        self.as_ref().map(|v| v.to_query(field_name))?
    }
}

#[cfg(feature = "chrono")]
impl<TZ> ToQuery for DateTime<TZ>
where
    TZ: TimeZone,
{
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self.to_rfc3339()))
    }
}

#[cfg(feature = "chrono")]
impl ToQuery for NaiveDate {
    fn to_query(&self, field_name: &str) -> Option<String> {
        Some(format!("{}={}", field_name, self.format("%Y-%m-%d")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[derive(ToQueryDerive)]
    struct TestStruct {
        a: String,
        b: bool,
        c: i32,
        d: Vec<String>,
    }
    #[test]
    fn test_to_query() {
        let test_struct = TestStruct {
            a: "a".to_string(),
            b: true,
            c: 1,
            d: vec!["a".to_string(), "b".to_string()],
        };
        let query = test_struct.to_query("");
        assert_eq!(query, Some("a=a&b=true&c=1&d=a&d=b".into()));
    }

    #[cfg(feature = "chrono")]
    #[derive(ToQueryDerive)]
    struct TestStruct2<TZ>
    where
        TZ: TimeZone,
    {
        a: DateTime<TZ>,
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_to_query2() {
        use chrono::Utc;

        let test_struct = TestStruct2::<Utc> {
            a: Utc.with_ymd_and_hms(2023, 12, 3, 11, 45, 0).unwrap(),
        };
        let query = test_struct.to_query("");
        assert_eq!(query, Some("a=2023-12-03T11:45:00+00:00".into()));
    }
}
