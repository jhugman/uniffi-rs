/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! This module contains code to handle foreign callbacks - C-ABI functions that are defined by a
//! foreign language, then registered with UniFFI.  These callbacks are used to implement callback
//! interfaces, async scheduling etc. Foreign callbacks are registered at startup, when the foreign
//! code loads the exported library. For each callback type, we also define a "cell" type for
//! storing the callback.

use std::{ptr::NonNull, sync::Mutex};

// Cell type that stores any NonNull<T>
#[doc(hidden)]
pub struct UniffiForeignPointerCell<T> {
    pointers: Mutex<Vec<*mut T>>,
}

impl<T> UniffiForeignPointerCell<T> {
    pub const fn new() -> Self {
        Self {
            pointers: Mutex::new(Vec::new()),
        }
    }

    pub fn set(&self, callback: NonNull<T>) -> usize {
        let mut pointers = self.pointers.lock().unwrap();
        let index = pointers.len();
        pointers.push(callback.as_ptr());
        index
    }

    pub fn get(&self, index: usize) -> &T {
        let pointers = self.pointers.lock().unwrap();
        if index >= pointers.len() {
            panic!("Foreign pointer used before being set. This is likely a uniffi bug.")
        }
        unsafe {
            let ptr = pointers[index];
            NonNull::new(ptr)
                .expect("Foreign pointer not set. This is likely a uniffi bug.")
                .as_ref()
        }
    }
}

unsafe impl<T> Send for UniffiForeignPointerCell<T> {}
unsafe impl<T> Sync for UniffiForeignPointerCell<T> {}
