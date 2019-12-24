use crate::qt_utils::qs;
use crate::traits::ToQString;
use qt_core::QObject;
use qt_widgets::cpp_core::{CppBox, MutPtr, MutRef, Ptr};
const RESET: &'static str = "_RESET_CONDUCTOR_";
/// Conductor has one Purpose and one purpose only:
/// to facilitate communication between threads in qt.
#[derive(Debug, Clone)]
pub struct Conductor<T: ToQString + std::cmp::PartialEq> {
    inner: MutPtr<QObject>,
    last: Option<T>,
}

unsafe impl<T> Send for Conductor<T> where T: ToQString + std::cmp::PartialEq {}

//unsafe impl Sync for Conductor {}

impl<T> Conductor<T>
where
    T: ToQString + std::cmp::PartialEq,
{
    /// new up a Conductor
    #[allow(dead_code)]
    pub fn from_q_object(object: MutPtr<QObject>) -> Self {
        Self {
            inner: object,
            last: None,
        }
    }
    /// New up a QObject and Conductor
    ///
    /// The QObject's  set_object_name is used
    pub fn new() -> (CppBox<QObject>, Self) {
        unsafe {
            let mut qobj = QObject::new_0a();
            let qobj_ptr = qobj.as_mut_ptr();
            (qobj, Self::from_q_object(qobj_ptr))
        }
    }

    #[allow(dead_code)]
    pub fn ptr(&self) -> Ptr<QObject> {
        unsafe { self.inner.as_ptr() }
    }
    #[allow(dead_code)]
    pub fn mut_ref(&mut self) -> Option<MutRef<QObject>> {
        unsafe { self.inner.as_mut_ref() }
    }
    /// A more structured api for  signaling. This uses set_object_name under the hood,
    /// but that is an unfortunate implementation detail.
    pub fn signal(&mut self, event: T) {
        unsafe {
            // turns out that qt keeps track of the name and only emits a
            // signal if the name has changed. So.. if our last event was the same
            // as the current event, we have to set a different name first
            if let Some(e) = &self.last {
                if e == &event {
                    self.inner.set_object_name(&qs(RESET))
                }
            }
            let event_qs = event.to_qstring();
            self.last = Some(event);
            self.inner.set_object_name(&event_qs);
        }
    }
}
