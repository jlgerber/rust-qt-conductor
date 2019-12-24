use qt_core::QString;
use qt_widgets::cpp_core::CppBox;

/// Convenience function to construct a CppBox'ed QString from a &str.
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}
