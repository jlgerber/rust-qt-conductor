//! The Event enum will be defined on a per project basis.
//! It should encompass all of the states which we need to
//! differentiate between in our main thread.
use qt_core::QString;
use qt_thread_conductor::traits::*;
use qt_widgets::cpp_core::{CppBox, Ref};

#[derive(Debug, PartialEq)]
pub enum Event {
    DbJokeUpdate,
    DbPunchlineUpdate,
}

const DDJOKEUPDATE: &'static str = "DbJokeUpdate";
const DDPUNCHLINEUPDATE: &'static str = "DbPunchlineUpdate";

impl ToQString for Event {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &Event::DbPunchlineUpdate => QString::from_std_str(DDPUNCHLINEUPDATE),
            &Event::DbJokeUpdate => QString::from_std_str(DDJOKEUPDATE),
        }
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            DDJOKEUPDATE => Event::DbJokeUpdate,
            DDPUNCHLINEUPDATE => Event::DbPunchlineUpdate,
            _ => panic!("Unable to convert to Event"),
        }
    }
}
