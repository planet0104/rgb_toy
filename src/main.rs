#![feature(alloc_error_handler)]
#![feature(get_mut_unchecked)]
#![no_std]
#![no_main]

extern crate alloc;
mod note;
mod player;
mod songs;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_time::rate::Fraction;
use embedded_time::Instant;
use note::NOTE_SET;
use panic_halt as _;
use player::Player;
use stm32f1xx_hal::{
    adc::Adc,
    pac,
    timer::{Tim2NoRemap, Tim3NoRemap},
};
use stm32f1xx_hal::{prelude::*, timer::Channel};

#[cfg(feature = "semihosting")]
#[macro_export]
macro_rules! println {
    () => {
        cortex_m_semihosting::hprintln!()
    };
    ($s:expr) => {
        cortex_m_semihosting::hprintln!($s)
    };
    ($s:expr, $($tt:tt)*) => {
        cortex_m_semihosting::hprintln!($s, $($tt)*).unwrap()
    };
}

#[cfg(not(feature = "semihosting"))]
#[macro_export]
#[allow(dead_code)]
macro_rules! println {
    () => {};
    ($s:expr) => {};
    ($s:expr, $($tt:tt)*) => {};
}

#[derive(Debug)]
struct KeyClock;
impl embedded_time::Clock for KeyClock {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);
    fn try_now(&self) -> Result<Instant<Self>, embedded_time::clock::Error> {
        Ok(Instant::<Self>::new(23))
    }
}

// 堆内存分配器
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
/// 堆内存 16K
const HEAP_SIZE: usize = 1024 * 18;

/// 声光模式
const MODE_SOUND: u8 = 0;
// RGB模式
const MODE_RGB: u8 = 1;
// 音乐模式
const MODE_MUSIC: u8 = 2;

#[entry]
fn main() -> ! {
    // 初始化内存分配器
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // 从 cortex-m crate 访问核心外围设备
    let dp = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

    // 获取原始闪存和 rcc 设备的所有权并将它们转换为相应的 HAL 结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在`clocks`中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    // 获取 GPIOC 外设
    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 模拟器输入A0~A9，选择A4~A7

    // RGB和亮度4个模拟输入值(A4~A7)
    let mut pa4 = gpioa.pa4.into_analog(&mut gpioa.crl);
    let mut pa5 = gpioa.pa5.into_analog(&mut gpioa.crl);
    let mut pa6 = gpioa.pa6.into_analog(&mut gpioa.crl);
    let mut pa7 = gpioa.pa7.into_analog(&mut gpioa.crl);

    // RGB和蜂鸣器4个pwm输出(A0~A2, B0)
    let pwm_r = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let pwm_g = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let pwm_b = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let pwm_pins = (pwm_r, pwm_g, pwm_b);

    let mut pwm =
        dp.TIM2
            .pwm_hz::<Tim2NoRemap, _, _>(pwm_pins, &mut afio.mapr, 1_u32.Hz(), &clocks);
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);
    //最高占空比
    let max_duty = pwm.get_max_duty();
    //设置PWM周期(led灯的频率)
    pwm.set_period(300_u32.Hz());

    // 配置扬声器的PWM
    let speaker_io = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let mut pwm_speaker =
        dp.TIM3
            .pwm_hz::<Tim3NoRemap, _, _>(speaker_io, &mut afio.mapr, 1.kHz(), &clocks);

    let delay = dp.TIM1.delay_us(&clocks);

    // 播放一个声音
    let play_sound = |freq: u32, enable: bool, volume: u16| {
        if !enable {
            pwm_speaker.disable(Channel::C3);
            return;
        } else {
            pwm_speaker.enable(Channel::C3);
        }
        if freq == 0 {
            //频率=0 设置占空比为0，关闭蜂鸣器
            pwm_speaker.set_duty(Channel::C3, pwm_speaker.get_max_duty());
        } else {
            //占空比越高越响
            pwm_speaker.set_duty(Channel::C3, volume);
            //设置频率
            pwm_speaker.set_period(freq.Hz());
        }
    };

    // 初始化音乐播放器
    let mut player = Player::new(play_sound, delay);
    player.init_music();

    // 设置 ADC
    let mut adc = Adc::adc1(dp.ADC1, clocks);

    // 按钮
    let key = gpioa.pa8.into_pull_down_input(&mut gpioa.crh);

    //控制逻辑

    /*
    IO输出: RGB(a0,a1,a2) 喇叭(b0)
    IO输入: 调光电阻(a4,a5,a6) 光敏电阻(a7)  按钮(c5)
    模式：光敏模式、RGB模式、音乐模式
    1、默认启动
       进入光敏模式
    2、按钮逻辑
       单击时:光敏模式->进入RGB模式；RGB模式->音乐模式；音乐模式->切换下一首
       双击时:光敏模式->Beep；RGB模式->Beep；音乐模式->进入光敏模式；
    3、旋钮逻辑
       光敏模式->无效；RGB模式->调整RGB；音乐模式->无效；
    */

    let mut mode = MODE_SOUND;

    let mut last_note = 0;
    let mut quiet_count = 0;

    //按键弹起
    // let mut mode_copy0 = mode.clone();
    let on_key_up = |mode: &mut u8, player: &mut Player<_>, _delay_ms: i32| {
        if *mode == MODE_SOUND {
            player.enable_speaker(false);
            *mode = MODE_RGB;
        } else if *mode == MODE_RGB {
            *mode = MODE_MUSIC;
            player.reset();
        } else {
            //MODE_MUSIC 切换下一首
            player.next_music();
        }
    };

    let on_long_press = |mode: &mut u8| {
        //长按退出RGB模式
        *mode = MODE_SOUND;
    };

    loop {
        // 按键检测
        if key.is_high() {
            player.enable_speaker(false);
            // 延时10ms，这期间不检测,防止按下瞬间的机械抖动产生高电平对检测造成干扰
            player.delay_ms(10);
            // 延时10ms后再检测，检测到低电平状态，说明按键稳定的按下了
            // if key.is_high(){
            //     on_key_press();
            // }
            // 按键是按下状态，就卡在这不动，直到按键松开
            let mut delay_ms = 10;
            while key.is_high() {
                player.delay_ms(10);
                delay_ms += 10;
            }
            if delay_ms > 500 {
                on_long_press(&mut mode);
            } else {
                on_key_up(&mut mode, &mut player, delay_ms);
            }
            continue;
        }

        if mode == MODE_SOUND {
            //读取蓝色按钮电阻,用于调整速度
            let speed: u16 = adc.read(&mut pa6).unwrap();
            let mut speed_perent = speed as f32 / 4000.;
            if speed_perent > 1.0 {
                speed_perent = 1.0;
            }

            let delay_ms = 25. + speed_perent * 100.;

            // 读取光敏电阻
            let b0: u16 = adc.read(&mut pa7).unwrap();

            //亮度 0.0~1.0
            let brightness = 1.0 - (b0 as f32 / 4096.);

            //根据亮度切换音符

            // 亮度小于0.1关闭声音
            if brightness < 0.1 {
                player.enable_speaker(false);
            } else {
                //频率范围
                let note_index = (brightness * (NOTE_SET.len() - 1) as f32) as usize;
                if last_note != note_index {
                    last_note = note_index;
                    quiet_count = 0;

                    player.sound(NOTE_SET[note_index] as u32, delay_ms as u32);
                    // let freq = 100. + brightness * 800.;
                    // player.sound(freq as u32, delay_ms as u32);
                } else {
                    quiet_count += 1;
                }
                //当延迟大时，quiet_count应该尽可能少，延迟小时quiet_count尽量大。
                // let max_quite_count = (6.0*(1.0 - b as f32/255.0)) as i32;
                let max_quite_count = 2;
                if quiet_count >= max_quite_count {
                    quiet_count = max_quite_count;
                    player.enable_speaker(false);
                } else {
                    player.enable_speaker(true);
                }
            }
        } else if mode == MODE_RGB {
            ();
        } else if mode == MODE_MUSIC {
            //读取蓝色按钮电阻,用于调整音量
            let volume: u16 = adc.read(&mut pa6).unwrap();
            player.set_volume(volume);
            if !player.ended() {
                let _c = player.play();
            } else {
                if player.get_theme().is_some() {
                    //重新播放
                    player.reset();
                    player.delay_ms(500);
                }
            }
        }

        //读取rgb电阻电压
        let c0: u16 = adc.read(&mut pa4).unwrap();
        let c1: u16 = adc.read(&mut pa5).unwrap();
        let c2: u16 = adc.read(&mut pa6).unwrap();

        //电压转换成占空比
        let r = c0 as f32 / 4096.;
        let g = c1 as f32 / 4096.;
        let b = c2 as f32 / 4096.;

        //设置占空比
        pwm.set_duty(Channel::C1, (r * max_duty as f32) as u16);
        pwm.set_duty(Channel::C2, (g * max_duty as f32) as u16);
        pwm.set_duty(Channel::C3, (b * max_duty as f32) as u16);
    }
}

// 内存不足执行此处代码
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}
