//! Where Telegram sends button presses.
//!
//! Checks the secret header **before** reading anything else. Without that
//! check, anyone who finds the URL can send fake button presses and poison
//! your training data — quietly, permanently, and in a way you cannot undo
//! once it is mixed in.
//!
//! Replies OK quickly and does the work afterwards. Telegram retries if you
//! are slow, and a slow handler turns one button press into several duplicate
//! verdicts.
