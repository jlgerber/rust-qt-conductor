use crate::traits::*;
use qt_core::QString;
use qt_widgets::cpp_core::{CppBox, Ref};
pub enum Event {
    DbJokeUpdate,
    DbPunchlineUpdate,
    Reset,
}

const DDJOKEUPDATE: &'static str = "DbJokeUpdate";
const DDPUNCHLINEUPDATE: &'static str = "DbPunchlineUpdate";
const RESET: &'static str = "RESET";
impl ToQString for Event {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &Event::DbPunchlineUpdate => QString::from_std_str(DDPUNCHLINEUPDATE),
            &Event::DbJokeUpdate => QString::from_std_str(DDJOKEUPDATE),
            &Event::Reset => QString::from_std_str(RESET),
        }
    }
}

impl FromQString for Event {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            DDJOKEUPDATE => Event::DbJokeUpdate,
            DDPUNCHLINEUPDATE => Event::DbPunchlineUpdate,
            RESET => Event::Reset,
            _ => panic!("Unable to convert to Event"),
        }
    }
}