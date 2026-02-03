//! This example shows how to use PWM (Pulse Width Modulation) in the RP2040 chip.
//!
//! We demonstrate two ways of using PWM:
//! 1. Via config
//! 2. Via setting a duty cycle

#![no_std]
#![no_main]


/*
Slice,Kanal A (Pin),Kanal B (Pin)
0,"GPIO 0, 16","GPIO 1, 17"
1,"GPIO 2, 18","GPIO 3, 19"
2,"GPIO 4, 20","GPIO 5, 21"
3,"GPIO 6, 22","GPIO 7, 23"
4,"GPIO 8, 24","GPIO 9, 25"
5,"GPIO 10, 26","GPIO 11, 27"
6,"GPIO 12, 28","GPIO 13, 29"
7,GPIO 14,GPIO 15

Beschriftung (Board),Code (embassy-rp),PWM Slice,Kanal
0 bis 15,p.PIN_0 bis p.PIN_15,Slices 0-7,A & B abwechselnd
16,p.PIN_16,Slice 0,A
17,p.PIN_17,Slice 0,B
18,p.PIN_18,Slice 1,A
25,p.PIN_25,Slice 4,B
*/

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::{
    PIN_0,
    PIN_1,
    PIN_2,
    PIN_3,
    PIN_4,
    PIN_5,
    PIN_6,
    PIN_7,
    PIN_8,
    PIN_9,
    PIN_10,
    PIN_11,
    PIN_12,
    PIN_13,
    PIN_14,
    PIN_15,
    PIN_16,
    PIN_17,
    PIN_18,
    PIN_19,
    PIN_20,
    PIN_21,
    PIN_22,
    PIN_23,
    PIN_24,
    PIN_25, 
    PWM_SLICE0,
    PWM_SLICE1,
    PWM_SLICE2,
    PWM_SLICE3,
    PWM_SLICE4,
    PWM_SLICE5,
    PWM_SLICE6,
    PWM_SLICE7,
};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_rp::Peri;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // onboard LED 
    // spawner.spawn(pwm_set_config(p.PWM_SLICE4, p.PIN_25)).unwrap();
    spawner.spawn(run_task_gpio_4(p.PWM_SLICE2, p.PIN_4)).unwrap();
    spawner.spawn(run_task_gpio_17(p.PWM_SLICE0, p.PIN_17)).unwrap();
}


/// Using the onboard led, if You are using a different Board than plain Pico2 (i.e. W variant)
/// you must use another slice & pin and an appropriate resistor.
#[embassy_executor::task]
async fn pwm_set_config(slice4: Peri<'static, PWM_SLICE4>, pin25: Peri<'static, PIN_25>) {
    let mut c = Config::default();
    c.top = 32_768;
    c.compare_b = 8;
    let mut pwm = Pwm::new_output_b(slice4, pin25, c.clone());

    loop {
        info!("current LED duty cycle: {}/32768", c.compare_b);
        Timer::after_secs(1).await;
        c.compare_b = c.compare_b.rotate_left(4);
        pwm.set_config(&c);
    }
}


#[embassy_executor::task]
async fn run_task_gpio_4(slice2: Peri<'static, PWM_SLICE2>, pin4: Peri<'static, PIN_4>) {
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

    let mut pwm = Pwm::new_output_a(slice2, pin4, c.clone());

    loop {
        // 100% duty cycle, fully on
        pwm.set_duty_cycle_fully_on().unwrap();
        Timer::after_secs(1).await;

        // 66% duty cycle. Expressed as simple percentage.
        pwm.set_duty_cycle_percent(66).unwrap();
        Timer::after_secs(1).await;

        // 25% duty cycle. Expressed as 32768/4 = 8192.
        pwm.set_duty_cycle(c.top / 4).unwrap();
        Timer::after_secs(1).await;

        // 0% duty cycle, fully off.
        pwm.set_duty_cycle_fully_off().unwrap();
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn run_task_gpio_17(slice2: Peri<'static, PWM_SLICE0>, pin: Peri<'static, PIN_17>) {
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

    let mut pwm = Pwm::new_output_b(slice2, pin, c.clone());

    loop {
        // 100% duty cycle, fully on
        pwm.set_duty_cycle_fully_on().unwrap();
        Timer::after_secs(1).await;

        // 66% duty cycle. Expressed as simple percentage.
        pwm.set_duty_cycle_percent(66).unwrap();
        Timer::after_secs(1).await;

        // 25% duty cycle. Expressed as 32768/4 = 8192.
        pwm.set_duty_cycle(c.top / 4).unwrap();
        Timer::after_secs(1).await;

        // 0% duty cycle, fully off.
        pwm.set_duty_cycle_fully_off().unwrap();
        Timer::after_secs(1).await;
    }
}
