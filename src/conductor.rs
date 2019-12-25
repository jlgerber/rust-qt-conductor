use crate::qt_utils::qs;
use crate::traits::ToQString;
use qt_core::{QObject, SlotOfQString};
use qt_widgets::cpp_core::{CppBox, MutRef, Ptr};
const RESET: &'static str = "_RESET_CONDUCTOR_";

/// Conductor has one Purpose and one purpose only:
/// enable communication with rust-qt from other thread(s). The primary
/// use case for Conductor is to support long running qeries or computations without
/// blocking the main ui thread.
///
/// The Conductor struct should be instantiated with a reference
/// to a SlotOfQString, whose role it is to respond to Conductor signals.
///
/// During instantiation, Conductor connects its signal to the SlotOfQString Slot.
/// Subsequent calls to `Conductor::signal` will emit a Qt Signal, and ultimately
/// fire off the aforementioned Slot.
///
/// Afer instantiation, the Conductor instance should typically be moved
/// into a separate thread, where it should be used signal change.  Bi-directional
/// communication between the main thread and the secondary thread would generally
/// involve one or more channels for each direction (see `std::sync::mpsc::channel`).
///
/// A typical usage pattern would be to define a sender,receiver pair for each
/// direction of communication and type of data. In the SlotOfQString, one would
/// match against the incoming QString, and pull data out of the appropriate
/// receiver.
///
/// In the secondary thread, one would loop over received values from the
/// incoming channel, peform appropriate work, including sending data over
/// the channel sender, and ultimately calling `conductor.signal()` with the
/// appropriate value. (hopefully a variant of an enum implementing ToQString and
/// FromQString).
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
