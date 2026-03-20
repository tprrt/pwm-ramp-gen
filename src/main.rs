/*
 * Copyright (C) 2021 Thomas Perrot <thomas.perrot@tupi.fr>
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! PWM ramp generator

#![no_std]

use riot_wrappers::{println, riot_main};

const PWM_FREQ: u32 = 1000;
const PWM_INTERVAL: u32 = 10 * 1000; /* 10 ms in microseconds */
const PWM_RES: u16 = 256;
const PWM_MAX: u16 = PWM_RES - 1;
const PWM_STEP: u16 = 10;

riot_main!(main);

fn main() {
    println!("PWM ramp generator application");

    let pwm_dev = 0 as riot_sys::pwm_t;

    unsafe {
        riot_sys::pwm_init(pwm_dev, riot_sys::pwm_mode_t_PWM_LEFT, PWM_FREQ, PWM_RES);
    }

    let mut state: u16 = 0;
    let mut up = true;
    let mut last_wakeup = unsafe { riot_sys::inline::ztimer_now(riot_sys::ZTIMER_USEC as *mut _) };

    loop {
        unsafe {
            riot_sys::pwm_set(pwm_dev, 0, state);
        }

        if up {
            if state + PWM_STEP >= PWM_MAX {
                state = PWM_MAX;
                up = false;
            } else {
                state += PWM_STEP;
            }
        } else {
            if state < PWM_STEP {
                state = 0;
                up = true;
            } else {
                state -= PWM_STEP;
            }
        }

        unsafe {
            riot_sys::ztimer_periodic_wakeup(riot_sys::ZTIMER_USEC, &mut last_wakeup, PWM_INTERVAL);
        }
    }
}
