use crate::qt_utils::qs;
use crate::traits::ToQString;
use qt_core::{QObject, SlotOfQString};
use qt_widgets::cpp_core::{CppBox, MutRef, Ptr};
const RESET: &'static str = "_RESET_CONDUCTOR_";

/// Conductor has one Purpose and one purpose only:
/// to facilitate communication with qt from other thread(s).
///
/// The Conductor instance should be instantiated with a reference
/// to a SlotOfQString, whose role it is to respond to Conductor signals.
///
/// The Conductor should typically be moved into a separate thread. Where
/// it should be used to instigate change in the aforementioned SlotOfQString.
///
/// One should note that the Conductor::signal's event will typically not
/// be flexible enough to communicate ui state changes. It is recommended
/// to use channels to send and receive UI state changes, followed by a
/// call to conductor.signal(event). See the example for more details.
#[derive(Debug)]
pub struct Conductor<T: ToQString + std::cmp::PartialEq> {
    inner: CppBox<QObject>,
    last: Option<T>,
}

unsafe impl<T> Send for Conductor<T> where T: ToQString + std::cmp::PartialEq {}

//unsafe impl Sync for Conductor {}

impl<T> Conductor<T>
where
    T: ToQString + std::cmp::PartialEq,
{
    /// New up a QObject and Conductor
    ///
    /// # Argument
    /// * slotOfQString reference. This is the slot that we will be updating
    /// when we call Self::signal
    ///
    /// # Returns
    /// * new Conductor instance
    pub fn new<'a>(slot: &SlotOfQString<'a>) -> Self {
        unsafe {
            let qobj = QObject::new_0a();
            qobj.object_name_changed().connect(slot);
            Self::from_q_object(qobj)
        }
    }
    // new up a Conductor from a box'ed QObject
    #[allow(dead_code)]
    fn from_q_object(object: CppBox<QObject>) -> Self {
        Self {
            inner: object,
            last: None,
        }
    }

    #[allow(dead_code)]
    /// Retrieve a Ptr wrapping our inner QObject
    pub fn ptr(&self) -> Ptr<QObject> {
        unsafe { self.inner.as_ptr() }
    }

    #[allow(dead_code)]
    /// Retrieve a mutable pointer wrapping our inner QObject
    pub fn mut_ref(&mut self) -> MutRef<QObject> {
        unsafe { self.inner.as_mut_ref() }
    }
    /// Fire a signal for an event of type T (where T is ToQString + FromQString )
    ///
    /// # Arguments
    /// * `event` - an instance of type T, which can convert to and from a QString.
    /// Our signal has a single argument of type QString. Rather than sling around
    /// ad hoc strings, Conductor expects the user to define an enum which implements
    /// ToQString and FromQString, so that the user gains the benefits of strict typing
    /// at compile time.
    /// Typically, the user will define an Event that impls ToQString and FromQString
    /// and  that exposes all of the states that we wish to respond to in the main thread
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
