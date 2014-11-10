/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![comment = "The Servo Parallel Browser Project"]
#![license = "MPL"]

// TODO : make code compile with unused_XXX
// #![deny(unused_imports, unused_variable)]

extern crate alert;
extern crate compositing;
extern crate geom;
extern crate layers;
extern crate libc;
extern crate msg;
extern crate time;
extern crate util;
extern crate egl;

use compositing::windowing::WindowEvent;
use std::rc::Rc;
use window::Window;

pub mod window;

pub trait NestedEventLoopListener {
    fn handle_event_from_nested_event_loop(&mut self, event: WindowEvent) -> bool;
}

#[cfg(not(test))]
pub fn create_window() -> Rc<Window> {
    print!("Creating window, gonk edition.");
    Window::new()
}
