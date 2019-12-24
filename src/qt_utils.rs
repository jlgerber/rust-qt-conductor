use qt_core::QString;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QHBoxLayout, QLabel, QVBoxLayout,
};

/// Convenience function to construct a CppBox'ed QString from a &str.
pub fn qs<S: AsRef<str>>(input: S) -> CppBox<QString> {
    QString::from_std_str(input.as_ref())
}

/// Construct a new label
pub fn new_label(text: Option<&'static str>) -> (CppBox<QLabel>, MutPtr<QLabel>) {
    unsafe {
        let mut label = match text {
            Some(text) => QLabel::from_q_string(&qs(text)),
            None => QLabel::new(),
        };
        let label_ptr = label.as_mut_ptr();
        (label, label_ptr)
    }
}

pub fn new_hblayout() -> (CppBox<QHBoxLayout>, MutPtr<QHBoxLayout>) {
    unsafe {
        let mut layout = QHBoxLayout::new_0a();
        let ref_layout = layout.as_mut_ptr();
        (layout, ref_layout)
    }
}

pub fn new_vblayout() -> (CppBox<QVBoxLayout>, MutPtr<QVBoxLayout>) {
    unsafe {
        let mut layout = QVBoxLayout::new_0a();
        let ref_layout = layout.as_mut_ptr();
        (layout, ref_layout)
    }
}
