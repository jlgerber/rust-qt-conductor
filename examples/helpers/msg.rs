//! The Msg type is sent from the ui to the
//! helper thread via a channel

#[derive(Debug)]
pub enum Msg {
    NewJokeRequest,
    Quit,
}
