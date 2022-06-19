use std::option::Option;

use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use pinyin::{Pinyin, ToPinyin};

pub const PAGE_SIZE: usize = 11;

pub fn object_id() -> Option<String> {
    Option::from(ObjectId::new().to_hex())
}

pub fn time_now() -> Option<DateTime<Utc>> {
    Option::from(Utc::now())
}

// pub async fn cursors_to_list<T>(mut cursors: Cursor<T>) -> Vec<T> {
//     let mut res: Vec<T> = Vec::new();
//
//     while let Some(i) = cursors.next().await {
//         res.push(i?)
//     }
//
//     res
// }

// pub fn success_res<T, E>(raw: T) -> Result<impl IntoResponse, E> {
//     Ok((StatusCode::OK, Json(raw)))
// }

pub trait PinyinForString {
    fn to_pinyin(&self) -> Option<String>;
}

impl PinyinForString for String {
    fn to_pinyin(&self) -> Option<String> {
        let pinyin_iter = self.as_str().to_pinyin();

        let mut pinyin = String::from("");

        for i in pinyin_iter {
            let cut = i.map(Pinyin::plain);

            pinyin.push_str(match cut {
                None => { "" }
                Some(s) => { s }
            });
        }

        Option::from(pinyin)
    }
}