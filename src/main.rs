//! This example shows how to use PWM (Pulse Width Modulation) in the RP2040 chip.
//!
//! We demonstrate two ways of using PWM:
//! 1. Via config
//! 2. Via setting a duty cycle

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::{
    PIN_0, PIN_1, PIN_10, PIN_11, PIN_12, PIN_13, PIN_14, PIN_15, PIN_16, PIN_17, PIN_18, PIN_19,
    PIN_2, PIN_20, PIN_21, PIN_22, PIN_23, PIN_24, PIN_25, PIN_3, PIN_4, PIN_5, PIN_6, PIN_7,
    PIN_8, PIN_9, PWM_SLICE0, PWM_SLICE1, PWM_SLICE2, PWM_SLICE3, PWM_SLICE4, PWM_SLICE5,
    PWM_SLICE6, PWM_SLICE7,
};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_rp::Peri;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const NUM_GPIO_PINS: u64 = 16;
/*
Slice   Kanal A (Pin)   Kanal B (Pin)
0       GPIO 0, 16      GPIO 1, 17
1       GPIO 2, 18      GPIO 3, 19
2,      GPIO 4, 20      GPIO 5, 21
3       GPIO 6, 22      GPIO 7, 23
4       GPIO 8, 24      GPIO 9, 25
5       GPIO 10, 26     GPIO 11, 27
6       GPIO 12, 28     GPIO 13, 29
7       GPIO 14         GPIO 15


Beschriftung (Board),Code (embassy-rp),PWM Slice,Kanal
0 bis 15,p.PIN_0 bis p.PIN_15,Slices 0-7,A & B abwechselnd
16,p.PIN_16,Slice 0,A
17,p.PIN_17,Slice 0,B
18,p.PIN_18,Slice 1,A
25,p.PIN_25,Slice 4,B
*/


fn get_pwm_config() -> Config {
    // If we aim for a specific frequency, here is how we can calculate the top value.
    // The top value sets the period of the PWM cycle, so a counter goes from 0 to top and then wraps around to 0.
    // Every such wraparound is one PWM cycle. So here is how we get 25KHz:
    let desired_freq_hz = 25_000;
    let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
    let divider = 16u8;
    let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;

    let mut c = Config::default();
    c.top = period;
    c.divider = divider.into();
    c
}

// async fn run_counter(mut pwm_a: Pwm<'_>, mut pwm_b: Pwm<'_>, offset: u64) {
async fn run_counter(mut c: Config, mut pwm: Pwm<'_>, offset: u64) {
    Timer::after_secs(offset).await;

    loop {
        // State 1: A=100%, B=0%
        c.compare_a = c.top;
        c.compare_b = 0;
        pwm.set_config(&c);
        Timer::after_secs(1).await;

        c.compare_a = 0;
        c.compare_b = c.top;
        pwm.set_config(&c);
        Timer::after_secs(1).await;

        c.compare_b = 0;
        pwm.set_config(&c);
        Timer::after_secs(NUM_GPIO_PINS - 1).await;
    }

    // loop {
    //     // 100% duty cycle, fully on
    //     pwm_a.set_duty_cycle_fully_on().unwrap();
    //     Timer::after_secs(1).await;

    //     pwm_a.set_duty_cycle_fully_off().unwrap();
    //     pwm_b.set_duty_cycle_fully_on().unwrap();

    //     Timer::after_secs(1).await;
    //     pwm_b.set_duty_cycle_fully_off().unwrap();
    //     // // 66% duty cycle. Expressed as simple percentage.
    //     // pwm.set_duty_cycle_percent(66).unwrap();
    //     // Timer::after_secs(1).await;

    //     // // 25% duty cycle. Expressed as 32768/4 = 8192.
    //     // pwm.set_duty_cycle(top / 4).unwrap();
    //     // Timer::after_secs(1).await;

    //     // 0% duty cycle, fully off.
    //     Timer::after_secs(NUM_GPIO_PINS-1).await;

    // }
}


#[embassy_executor::task]
async fn run_task_pin0_pin1(
    slice: Peri<'static, PWM_SLICE0>,
    pin_a: Peri<'static, PIN_0>,
    pin_b: Peri<'static, PIN_1>,
) {
    // traffic light simulation: A and B alternate between 100% and 0% duty cycle every 10 seconds
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 0).await;

    // loop {
    //     // State 1: A=100%, B=0%
    //     c.compare_a = c.top;
    //     c.compare_b = 0;
    //     pwm.set_config(&c);
    //     Timer::after_secs(10).await;

    //     // State 3: A=0%, B=100%
    //     c.compare_a = 0;
    //     c.compare_b = c.top;
    //     pwm.set_config(&c);
    //     Timer::after_secs(10).await;
    // }
}

#[embassy_executor::task]
async fn run_task_pin2_pin3(
    slice: Peri<'static, PWM_SLICE1>,
    pin_a: Peri<'static, PIN_2>,
    pin_b: Peri<'static, PIN_3>,
) {
    // fire simulation: Random flicker between 10% and 100% duty cycle with random speed
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 2).await;

    // Simple Xorshift RNG state
    // let mut rng = 0xDEAD_BEEF;

    // loop {
    //     // Fire simulation: Random flicker
    //     // Update RNG for Pin A
    //     rng ^= rng << 13;
    //     rng ^= rng >> 17;
    //     rng ^= rng << 5;
    //     let r_a = rng;

    //     // Update RNG for Pin B
    //     rng ^= rng << 13;
    //     rng ^= rng >> 17;
    //     rng ^= rng << 5;
    //     let r_b = rng;

    //     // Generate intensity between 10% and 100% of top for realistic fire glow
    //     let min_val = c.top / 10;
    //     let range = c.top - min_val;

    //     // Calculate random brightness
    //     let val_a = min_val + (r_a % range as u32) as u16;
    //     let val_b = min_val + (r_b % range as u32) as u16;

    //     c.compare_a = val_a;
    //     c.compare_b = val_b;
    //     pwm.set_config(&c);

    //     // Random flicker speed between 20ms and 100ms
    //     let delay_ms = 20 + (r_a % 80) as u64;
    //     Timer::after_millis(delay_ms).await;
    // }
}

#[embassy_executor::task]
async fn run_task_pin4_pin5(
    slice: Peri<'static, PWM_SLICE2>,
    pin_a: Peri<'static, PIN_4>,
    pin_b: Peri<'static, PIN_5>,
) {
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 4).await;
    // // Set initial duty cycles
    // c.compare_a = c.top / 2; // 50%
    // c.compare_b = c.top / 4; // 25%

    // loop {
    //     // Example: Independent duty cycles

    //     // State 1: A=100%, B=0%
    //     c.compare_a = c.top;
    //     c.compare_b = 0;
    //     pwm.set_config(&c);
    //     Timer::after_secs(1).await;

    //     // State 2: A=66%, B=33%
    //     c.compare_a = (c.top as u32 * 2 / 3) as u16;
    //     c.compare_b = (c.top as u32 * 1 / 3) as u16;
    //     pwm.set_config(&c);
    //     Timer::after_secs(1).await;

    //     // State 3: A=0%, B=100%
    //     c.compare_a = 0;
    //     c.compare_b = c.top;
    //     pwm.set_config(&c);
    //     Timer::after_secs(1).await;
    // }
}

#[embassy_executor::task]
async fn run_task_pin6_pin7(
    slice: Peri<'static, PWM_SLICE3>,
    pin_a: Peri<'static, PIN_6>,
    pin_b: Peri<'static, PIN_7>,
) {
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 6).await;
}

#[embassy_executor::task]
async fn run_task_pin8_pin9(
    slice: Peri<'static, PWM_SLICE4>,
    pin_a: Peri<'static, PIN_8>,
    pin_b: Peri<'static, PIN_9>,
) {
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 8).await;
}

#[embassy_executor::task]
async fn run_task_pin10_pin11(
    slice: Peri<'static, PWM_SLICE5>,
    pin_a: Peri<'static, PIN_10>,
    pin_b: Peri<'static, PIN_11>,
) {
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 10).await;
}

#[embassy_executor::task]
async fn run_task_pin12_pin13(
    slice: Peri<'static, PWM_SLICE6>,
    pin_a: Peri<'static, PIN_12>,
    pin_b: Peri<'static, PIN_13>,
) {
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 12).await;
}

#[embassy_executor::task]
async fn run_task_pin14_pin15(
    slice: Peri<'static, PWM_SLICE7>,
    pin_a: Peri<'static, PIN_14>,
    pin_b: Peri<'static, PIN_15>,
) {
    let mut c = get_pwm_config();
    let mut pwm = Pwm::new_output_ab(slice, pin_a, pin_b, c.clone());

    run_counter(c, pwm, 14).await;
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner
        .spawn(run_task_pin0_pin1(p.PWM_SLICE0, p.PIN_0, p.PIN_1))
        .unwrap();
    spawner
        .spawn(run_task_pin2_pin3(p.PWM_SLICE1, p.PIN_2, p.PIN_3))
        .unwrap();
    spawner
        .spawn(run_task_pin4_pin5(p.PWM_SLICE2, p.PIN_4, p.PIN_5))
        .unwrap();
    spawner
        .spawn(run_task_pin6_pin7(p.PWM_SLICE3, p.PIN_6, p.PIN_7))
        .unwrap();
    spawner
        .spawn(run_task_pin8_pin9(p.PWM_SLICE4, p.PIN_8, p.PIN_9))
        .unwrap();
    spawner
        .spawn(run_task_pin10_pin11(p.PWM_SLICE5, p.PIN_10, p.PIN_11))
        .unwrap();
    spawner
        .spawn(run_task_pin12_pin13(p.PWM_SLICE6, p.PIN_12, p.PIN_13))
        .unwrap();
    spawner
        .spawn(run_task_pin14_pin15(p.PWM_SLICE7, p.PIN_14, p.PIN_15))
        .unwrap();
}
