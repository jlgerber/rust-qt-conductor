#![windows_subsystem = "windows"]
use qt_core::{QString, Slot, SlotOfQString};
use qt_thread_conductor::{conductor::Conductor, qt_utils::qs, traits::*};

use qt_widgets::{
    cpp_core::{CppBox, Ref},
    QApplication, QLabel, QMainWindow, QPushButton, QWidget,
};
mod helpers;
use event::*;
use helpers::event;
use helpers::msg::Msg;
use helpers::utils;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
use utils::*;

struct Form<'a> {
    _main: CppBox<QMainWindow>,
    joke_update: SlotOfQString<'a>,
    next_joke_slot: Slot<'a>,
}
const JOKES: &[(&'static str, &'static str)] = &[
    (
        "What do you call a dinosaur that is sleeping?",
        "A dino-snore!",
    ),
    ("What is fast, loud, and crunchy?", "A rocket chip"),
    (
        "Why did the teddy bear say no to desert?",
        "Because she was stuffed!",
    ),
    ("What has ears but cannot hear?", "A cornfield"),
    (
        "What did the right eye say to the left eye?",
        "Between you and me, something smells.",
    ),
    (
        "What do you get when you cross a vampire and a snowman?",
        "frost bite",
    ),
    (
        "Why did the student eat his homework?",
        "Because the teacher told her it was a piece of cake.",
    ),
    (
        "What is brown, hairy, and wears sun glasses?",
        "A coconut on vacation.",
    ),
    (
        "What did the dalmation say after lunch?",
        "That hit the spot",
    ),
    ("Why was 6 affraid of 7?", "Because 7,8,9"),
    ("when does a joke become a dad joke?", "When it's a parent"),
    (
        "What did the limestone say to the geologist?",
        "Dont take me for granite!",
    ),
    ("What kind of tree fits in your hand?", "A plam tree"),
    (
        "What did the baby corn say to the momma corn?",
        "Where is pop corn?",
    ),
    (
        "What is worse than raining cats and dogs?",
        "Hailing taxies",
    ),
    (
        "What building in New York City has the most stories?",
        "The public library",
    ),
    (
        "What is worse than finding a work in your apple?",
        "Finding half a worm in your apple.",
    ),
    (
        "Where did these jokes come from?",
        "Here: https://redtri.com/best-jokes-for-kids/slide/2",
    ),
];
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = Vec::new();
    // sender, receiver for communicating from secondary thread to primary ui thread
    let (sender, receiver) = channel();
    // sender and receiver for communicating from ui thread to secondary thread
    let (to_thread_sender, to_thread_receiver): (Sender<Msg>, Receiver<Msg>) = channel();
    // sender to handle quitting
    let to_thread_sender_quit = to_thread_sender.clone();
    let quit_slot = Slot::new(move || {
        to_thread_sender_quit
            .send(Msg::Quit)
            .expect("couldn't send");
    });

    QApplication::init(|app| unsafe {
        // create main window
        let mut main_window = QMainWindow::new_0a();
        let mut main_widget = QWidget::new_0a();
        // main window layout
        let (main_layout, mut main_layout_ptr) = new_vblayout();
        // horizontal box layout holding the joke label and joke
        let (joke_layout, mut joke_layout_ptr) = new_hblayout();
        // horizontal box layout holding the puchline label and punchline
        let (punchline_layout, mut punchline_layout_ptr) = new_hblayout();

        main_widget.set_layout(main_layout.into_ptr());
        main_layout_ptr.add_layout_1a(joke_layout.into_ptr());
        main_layout_ptr.add_layout_1a(punchline_layout.into_ptr());
        // top level
        let label = QLabel::from_q_string(&qs("Joke:"));
        joke_layout_ptr.add_widget(label.into_ptr());
        let (joke_result_label, mut joke_result_label_ptr) = new_label(None);
        joke_layout_ptr.add_widget(joke_result_label.into_ptr());
        joke_layout_ptr.add_stretch_1a(1);
        //joke level
        let punchline_label = QLabel::from_q_string(&qs("Answer:"));
        punchline_layout_ptr.add_widget(punchline_label.into_ptr());

        let (punchline_result_label, mut punchline_result_ptr) = new_label(None);
        punchline_layout_ptr.add_widget(punchline_result_label.into_ptr());
        punchline_layout_ptr.add_stretch_1a(1);
        let mut next_joke = QPushButton::from_q_string(&qs("Next Joke"));
        let next_joke_ptr = next_joke.as_mut_ptr();
        main_layout_ptr.add_widget(next_joke.into_ptr());

        main_window.set_central_widget(main_widget.into_ptr());
        main_window.show();
        //
        // Slot to receive conductor singals
        //
        /*
        We match on our Event and pull data out of our channel using the receiver
        */
        let joke_update =
            SlotOfQString::new(move |name: Ref<QString>| match Event::from_qstring(name) {
                Event::DbJokeUpdate => {
                    if let Ok(text) = receiver.recv() {
                        joke_result_label_ptr.set_text(&qs(text));
                        // since we may delay updating the punchline for "dramatic effect",
                        // we zero
                        punchline_result_ptr.set_text(&qs(""));
                    }
                }
                Event::DbPunchlineUpdate => {
                    if let Ok(text) = receiver.recv() {
                        punchline_result_ptr.set_text(&qs(text));
                    }
                }
            });
        let next_joke_slot = Slot::new(move || {
            to_thread_sender
                .send(Msg::NewJokeRequest)
                .expect("couldn't send");
        });
        let mut _form = Form {
            _main: main_window,
            joke_update: joke_update,
            next_joke_slot,
        };
        let mut myobj = Conductor::new(&_form.joke_update);
        next_joke_ptr.clicked().connect(&_form.next_joke_slot);
        let handle = spawn(move || {
            let mut cnt = 0;
            loop {
                let msg = to_thread_receiver
                    .recv()
                    .expect("Unable to unwrap received msg");
                match msg {
                    Msg::NewJokeRequest => {
                        // notice that there doesnt have to be a
                        // one to one relationship between incoming message
                        // and outgoing message. Here, we send
                        // data twice, and then issue two signals
                        sender
                            .send(JOKES[cnt % JOKES.len()].0)
                            .expect("unable to send");
                        myobj.signal(Event::DbJokeUpdate);
                        // lets sleep a bit. Notice that this blocks...
                        // If you press the button, it wont do anything
                        // until we wake
                        sleep(Duration::from_millis(1000));
                        // Now we provide the punchline
                        sender
                            .send(JOKES[cnt % JOKES.len()].1)
                            .expect("unable to send");
                        myobj.signal(Event::DbPunchlineUpdate)
                    }
                    Msg::Quit => return,
                }
                cnt += 1;
            }
        });
        handles.push(handle);
        // lets not let the second thread persist after we quit
        app.about_to_quit().connect(&quit_slot);
        let result = QApplication::exec();
        for handle in handles {
            handle.join().expect("Huh? the child thread panicked");
        }
        result
    });
}
