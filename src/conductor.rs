use crate::traits::ToQString;
use qt_core::QObject;
use qt_widgets::cpp_core::{CppBox, MutPtr, MutRef, Ptr};

/// Conductor has one Purpose and one purpose only:
/// to facilitate communication between threads in qt.
#[derive(Debug, Clone)]
pub struct Conductor(MutPtr<QObject>);

unsafe impl Send for Conductor {}

//unsafe impl Sync for Conductor {}

impl Conductor {
    /// new up a Conductor
    #[allow(dead_code)]
    pub fn from_q_object(object: MutPtr<QObject>) -> Self {
        Self(object)
    }
    /// New up a QObject and Conductor
    ///
    /// The QObject's  set_object_name is used
    pub fn new() -> (CppBox<QObject>, Self) {
        unsafe {
            let mut qobj = QObject::new_0a();
            let qobj_ptr = qobj.as_mut_ptr();
            (qobj, Self(qobj_ptr))
        }
    }

    #[allow(dead_code)]
    pub fn ptr(&self) -> Ptr<QObject> {
        unsafe { self.0.as_ptr() }
    }
    #[allow(dead_code)]
    pub fn mut_ref(&mut self) -> Option<MutRef<QObject>> {
        unsafe { self.0.as_mut_ref() }
    }
    /// A more structured api for  signaling. This uses set_object_name under the hood,
    /// but that is an unfortunate implementation detail.
    pub fn signal<T>(&mut self, event: T)
    where
        T: ToQString,
    {
        unsafe {
            let event_qs = event.to_qstring();
            self.0.set_object_name(&event_qs);
        }
    }
}
