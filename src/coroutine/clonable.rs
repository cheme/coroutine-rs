// The MIT License (MIT)

// Copyright (c) 2015 Rustcc developers

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Basic single threaded Coroutine
//!
//! ```rust
//! use coroutine::{spawn, sched};
//!
//! let coro = spawn(|| {
//!     println!("Before yield");
//!
//!     // Yield back to its parent who resume this coroutine
//!     sched();
//!
//!     println!("I am back!");
//! });
//!
//! // Starts the Coroutine
//! coro.resume().ok().expect("Failed to resume");
//!
//! println!("Back to main");
//!
//! // Resume it
//! coro.resume().ok().expect("Failed to resume");
//!
//! println!("Coroutine finished");
//! ```
//!

/* Here is the coroutine(with scheduler) workflow:
 *
 *                               --------------------------------
 * --------------------------    |                              |
 * |                        |    v                              |
 * |                  ----------------                          |  III.Coroutine::yield_now()
 * |             ---> |   Scheduler  |  <-----                  |
 * |    parent   |    ----------------       |   parent         |
 * |             |           ^ parent        |                  |
 * |   --------------  --------------  --------------           |
 * |   |Coroutine(1)|  |Coroutine(2)|  |Coroutine(3)|  ----------
 * |   --------------  --------------  --------------
 * |         ^            |     ^
 * |         |            |     |  II.do_some_works
 * -----------            -------
 *   I.Handle.resume()
 *
 *
 *  First, all coroutines have a link to a parent coroutine, which was set when the coroutine resumed.
 *  In the scheduler/coroutine model, every worker coroutine has a parent pointer pointing to
 *  the scheduler coroutine(which is a raw thread).
 *  Scheduler resumes a proper coroutine and set the parent pointer, like procedure I does.
 *  When a coroutine is awaken, it does some work like procedure II does.
 *  When a coroutine yield(io, finished, paniced or sched), it resumes its parent's context,
 *  like procedure III does.
 *  Now the scheduler is awake again and it simply decides whether to put the coroutine to queue again or not,
 *  according to the coroutine's return status.
 *  And last, the scheduler continues the scheduling loop and selects a proper coroutine to wake up.
 */

use std::default::Default;
use std::mem::transmute;
//use std::rt::unwind::try;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::Arc;
use std::fmt::{self, Debug};
use std::sync::Mutex;


use context::Context;
use context::stack::Stack;
use {Options, Result, Error};

pub struct State;

/// Handle of a Coroutine
#[derive(Clone)]
pub struct Handle;
pub static HANDLE : Handle = Handle;
impl Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      Ok(())
    }
}

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Handle {
    fn new(c: Coroutine) -> Handle {
        Handle
    }



    /// Resume the Coroutine
    pub fn resume(&self) -> Result<State> {
          Ok(State)
    }

    /// Join this Coroutine.
    ///
    /// If the Coroutine panicked, this method will return an `Err` with panic message.
    ///
    /// ```
    /// use coroutine::Coroutine;
    /// use coroutine::sched;
    /// // Wait until the Coroutine exits
    /// Coroutine::spawn(|| {
    ///     println!("Before yield");
    ///     sched();
    ///     println!("Exiting");
    /// }).join().unwrap();
    /// ```
    #[inline]
    pub fn join(&self) -> Result<State> {
        Ok(State)
    }

    /// Get the state of the Coroutine
    #[inline]
    pub fn state(&self) -> State {
    State
    }

    /// Set the state of the Coroutine
    #[inline]
    fn set_state(&self, state: State) {
    }

}


/// A coroutine is nothing more than a (register context, stack) pair.
// #[derive(Debug)]
pub struct Coroutine {
    /// The segment of stack on which the task is currently running or
    /// if the task is blocked, on which the task will resume
    /// execution.
    current_stack_segment: Option<Stack>,

    /// Always valid if the task is alive and not running.
    saved_context: Context,

    /// State
    state: Mutex<State>,

    /// Name
    name: Option<String>,
}

unsafe impl Send for Coroutine {}


impl Coroutine {

    #[doc(hidden)]
    pub unsafe fn empty(name: Option<String>, state: State) -> Handle {
        Handle
    }

    #[doc(hidden)]
    pub fn new(name: Option<String>, stack: Stack, ctx: Context, state: State) -> Handle {
        Handle
    }

    /// Spawn a Coroutine with options
    pub fn spawn_opts<F>(f: F, opts: Options) -> Handle
        where F: FnOnce() + Send + 'static
    {
      Handle

    }

    /// Spawn a Coroutine with default options
    pub fn spawn<F>(f: F) -> Handle
        where F: FnOnce() + Send + 'static
    {
      Handle
    
    }

    /// Yield the current running Coroutine to its parent
    #[inline]
    pub fn yield_now(state: State) {
    }


    /// Yield the current running Coroutine with `Suspended` state
    #[inline]
    pub fn sched() {
    }

    /// Yield the current running Coroutine with `Blocked` state
    #[inline]
    pub fn block() {
    }

    /// Get a Handle to the current running Coroutine.
    ///
    /// It is unsafe because it is an undefined behavior if you resume a Coroutine
    /// in more than one native thread.
    #[inline]
    pub fn current() -> &'static Handle {
      &HANDLE
    }


    /// Get the name of the Coroutine
    #[inline(always)]
    pub fn name(&self) -> Option<&str> {
None
    }

    /// Determines whether the current Coroutine is unwinding because of panic.
    #[inline(always)]
    pub fn panicking(&self) -> bool {
      false
    }

    /// Determines whether the Coroutine is finished
    #[inline(always)]
    pub fn finished(&self) -> bool {
        true
    }
}
