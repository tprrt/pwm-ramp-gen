/*
 * Copyright (C) 2021 Thomas Perrot <thomas.perrot@tupi.fr>
 *
 * SPDX-License-Identifier: GPL-3.0-only
 */

/**
 * @{
 *
 * @file
 * @brief       PWM ramp generator
 *
 * @author      Thomas Perrot <thomas.perrot@tupi.fr>
 *
 * @}
 */

#include <stdbool.h>
#include <stdio.h>

#include "periph/pwm.h"
#include "xtimer.h"

#define PWM_FREQ        (1000U)
#define PWM_INTERVAL    (10LU * US_PER_MS) /* 10 ms */
#define PWM_MODE        PWM_LEFT
#define PWM_RES         (256)
#define PWM_STEP        (10)

int main(void)
{
    uint16_t state = 0;
    bool up = true;
    xtimer_ticks32_t last_wakeup = xtimer_now();

    printf("PWM ramp generator application\n");

    pwm_init(PWM_DEV(0), PWM_MODE, PWM_FREQ, PWM_RES);

    for (;;) {
        pwm_set(PWM_DEV(0), 0, state);

        if (up) {
            if (state + PWM_STEP >= PWM_RES - 1) {
                state = PWM_RES - 1;
                up = false;
            } else {
                state += PWM_STEP;
            }
        } else {
            if (state < PWM_STEP) {
                state = 0;
                up = true;
            }
            else {
                state -= PWM_STEP;
            }
        }

        xtimer_periodic_wakeup(&last_wakeup, PWM_INTERVAL);
    }
}
