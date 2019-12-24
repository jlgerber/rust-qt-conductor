use qt_core::QString;
use qt_widgets::cpp_core::{CppBox, Ref};

pub trait ToQString {
    fn to_qstring(&self) -> CppBox<QString>;
}

pub trait FromQString {
    fn from_qstring(qs: Ref<QString>) -> Self;
}
