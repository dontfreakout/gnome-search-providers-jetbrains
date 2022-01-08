// Copyright Sebastian Wiesner <sebastian@swsnr.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Mainloop utilities for dbus serch providers.

use tracing::{debug, trace};

/// Create a simple main loop on `context`.
pub fn create_main_loop(context: &glib::MainContext) -> glib::MainLoop {
    let mainloop = glib::MainLoop::new(Some(context), false);

    trace!("Listening for SIGTERM");
    glib::source::unix_signal_add(
        libc::SIGTERM,
        glib::clone!(@strong mainloop =>  move || {
            debug!("Terminated, quitting mainloop");
            mainloop.quit();
            glib::Continue(false)
        }),
    );

    trace!("Listening for SIGINT");
    glib::source::unix_signal_add(
        libc::SIGINT,
        glib::clone!(@strong mainloop =>  move || {
            debug!("Interrupted, quitting mainloop");
            mainloop.quit();
            glib::Continue(false)
        }),
    );

    mainloop
}
