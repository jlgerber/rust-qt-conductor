#![windows_subsystem = "windows"]
mod conductor;
use conductor::Conductor;
use qt_core::{QObject, QString, Slot, SlotOfQString};
mod qt_utils;
use crate::qt_utils::*;
use std::sync::mpsc::{Receiver, Sender};
mod traits;
use qt_widgets::{
    cpp_core::{CppBox, Ref},
    QApplication, QLabel, QMainWindow, QPushButton, QWidget,
};

mod event;
use crate::event::*;

use std::sync::mpsc::channel;
use std::thread::spawn;
use traits::*;

struct Form<'a> {
    _main: CppBox<QMainWindow>,
    _widget: CppBox<QObject>,
    joke_update: SlotOfQString<'a>,
    next_joke_slot: Slot<'a>,
}

#[derive(Debug)]
enum Msg {
    NewJokeRequest,
    Quit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = Vec::new();
    let (sender, receiver) = channel();
    let (to_thread_sender, to_thread_receiver): (Sender<Msg>, Receiver<Msg>) = channel();
    let to_thread_sender_quit = to_thread_sender.clone();
    let quit_slot = Slot::new(move || {
        to_thread_sender_quit
            .send(Msg::Quit)
            .expect("couldn't send");
    });
    QApplication::init(|app| unsafe {
        // create main window
        let mut main_window = QMainWindow::new_0a();
        let mut main_w = QWidget::new_0a();
        // main window layout
        let (main_layout, mut main_layout_ptr) = new_vblayout();
        //
        let (top_layout, mut top_layout_ptr) = new_hblayout();
        // second level, joke layout
        let (lowerl, mut lowerl_ptr) = new_hblayout();

        let mut main_w_ptr = main_w.as_mut_ptr();
        main_w_ptr.set_layout(main_layout.into_ptr());
        main_layout_ptr.add_layout_1a(top_layout.into_ptr());
        main_layout_ptr.add_layout_1a(lowerl.into_ptr());
        // top level
        let label = QLabel::from_q_string(&qs("Joke:"));
        top_layout_ptr.add_widget(label.into_ptr());
        let (joke_result_label, mut joke_result_label_ptr) = new_label(None);

        top_layout_ptr.add_widget(joke_result_label.into_ptr());
        //joke level
        let punchline_label = QLabel::from_q_string(&qs("Answer:"));
        lowerl_ptr.add_widget(punchline_label.into_ptr());
        let (punchline_result_label, mut punchline_result_ptr) = new_label(None);
        lowerl_ptr.add_widget(punchline_result_label.into_ptr());

        let mut next_joke = QPushButton::from_q_string(&qs("Next Joke"));
        let next_joke_ptr = next_joke.as_mut_ptr();
        main_layout_ptr.add_widget(next_joke.into_ptr());

        main_window.set_central_widget(main_w.into_ptr());
        main_window.show();
        let joke_update =
            SlotOfQString::new(move |name: Ref<QString>| match Event::from_qstring(name) {
                Event::DbJokeUpdate => {
                    if let Ok(text) = receiver.recv() {
                        joke_result_label_ptr.set_text(&qs(text));
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
        let (myobject, mut myobj) = Conductor::new();
        let mut _form = Form {
            _main: main_window,
            _widget: myobject,
            joke_update: joke_update,
            next_joke_slot,
        };
        next_joke_ptr.clicked().connect(&_form.next_joke_slot);
        myobj
            .ptr()
            .object_name_changed()
            .connect(&_form.joke_update);
        let handle = spawn(move || {
            let mut cnt = 0;
            let jokes = vec![
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
            ];
            loop {
                let msg = to_thread_receiver
                    .recv()
                    .expect("Unable to unwrap received msg");
                //println!("recieved msg");
                match msg {
                    Msg::NewJokeRequest => {
                        //println!("sending joke via myobj");
                        sender
                            .send(jokes[cnt % jokes.len()].0)
                            .expect("unable to send");
                        sender
                            .send(jokes[cnt % jokes.len()].1)
                            .expect("unable to send");
                        //myobj.signal(Event::Reset);
                        myobj.signal(Event::DbJokeUpdate);
                        myobj.signal(Event::DbPunchlineUpdate)
                    }
                    Msg::Quit => return,
                }
                cnt += 1;
            }
        });
        handles.push(handle);
        app.about_to_quit().connect(&quit_slot);
        let result = QApplication::exec();
        for handle in handles {
            handle.join().expect("Huh? the child thread panicked");
        }
        result
    });
}
