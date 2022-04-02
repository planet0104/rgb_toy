use crate::note;
use crate::songs::*;
use crate::{
    alloc::{
        string::{String, ToString},
        vec::Vec,
    },
    println,
};
use core::str::FromStr;
use stm32f1xx_hal::pac::TIM1;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer::Delay;

pub const NOTE_D6: char = 'D';
pub const NOTE_B5: char = 'b';
pub const NOTE_G5: char = 'g';
pub const NOTE_E5: char = 'e';
pub const NOTE_C5: char = 'c';
pub const NOTE_A4: char = '6';
pub const NOTE_F4: char = '4';
pub const NOTE_D4: char = '2';
pub const NOTE_C4: char = '1';
pub const NOTE_E4: char = '3';
pub const NOTE_G4: char = '5';
pub const NOTE_B4: char = '7';
pub const NOTE_D5: char = 'd';
pub const NOTE_F5: char = 'f';
pub const NOTE_A5: char = 'a';
pub const NOTE_C6: char = 'C';
pub const NOTE_E6: char = 'E';

/// 音符
#[derive(Debug, Clone)]
pub struct Note {
    /// 起始节拍(八分之一拍)
    start_beat: u16,
    /// 终止节拍(八分之一拍)
    end_beat: u16,
    /// 琴键
    pub key: char,
    /// 播放音符之前延时
    pub delay: bool,
}

/// 曲子
#[derive(Debug)]
pub struct Song {
    /// 总共多少个八分之一拍
    total_beat: u16,
    /// 所有音符
    notes: Vec<Note>,
    /// 当前正在播放的音符
    cursor: usize,
}

/// 音符播放器
pub struct Player<F>
where
    F: FnMut(u32, bool, u16),
{
    delay: Delay<TIM1, 1000000>,
    current_beat: u16,
    /// 每小节几拍
    beat_per_group: u8,
    /// 每个八分之一拍多少微妙
    time_per_beat: u32,
    ended: bool,

    /// 主题
    theme: Option<Song>,

    play_sound: F,
    sound_index: usize,
    volume: u16,
}

impl<F> Player<F>
where
    F: FnMut(u32, bool, u16),
{
    pub fn new(play_sound: F, delay: Delay<TIM1, 1000000>) -> Player<F> {
        let mut player = Player {
            delay,
            current_beat: 0,
            beat_per_group: 0,
            time_per_beat: 0,
            theme: None,
            ended: false,
            play_sound,
            sound_index: FIRST_MUSIC,
            volume: 500,
        };
        player.reset();
        player
    }

    pub fn next_music(&mut self) {
        self.sound_index += 1;
        if self.sound_index >= MUSICS.len() {
            self.sound_index = 0;
        }
        self.set_song(MUSICS[self.sound_index].to_string());
    }

    pub fn init_music(&mut self) {
        self.sound_index = FIRST_MUSIC;
        self.set_song(MUSICS[self.sound_index].to_string());
        self.reset();
    }

    pub fn set_song(&mut self, mut songs: String) -> Option<()> {
        songs = songs
            .replace("\t", "")
            .replace("\n", "")
            .replace(" ", "")
            .replace("\r", "");
        //分离歌曲信息、主题和伴奏
        let mut songs = songs.split("_");

        //歌曲信息
        let mut info = songs.next()?.split(",");
        let beat_per_group = parse::<u8>(info.next()?)?; //每小节几拍
        let beat_per_min = parse::<f32>(info.next()?)?; //每分钟多少拍

        // 计算每拍延迟(1ms=1000000us)
        // 一个u32可存储4294967295纳秒，即4294.9毫秒，足够一个八分之一节拍延时使用

        // 每拍毫秒数
        let time_per_total_beat = 60000.0 / beat_per_min;
        //每八分之一拍毫秒数
        let time_per_eighth_beat = time_per_total_beat / 8.0;
        //每八分之一拍微妙数
        let time_per_beat = (time_per_eighth_beat * 1000.0) as u32;

        //至少有主题曲
        let theme_str = songs.next()?;
        self.theme = split_notes(theme_str);

        self.current_beat = 0;
        self.beat_per_group = beat_per_group;
        self.time_per_beat = time_per_beat;
        self.ended = false;
        Some(())
    }

    /// 播放下一个音符，返回开始的弹奏的音符
    fn play_note(&mut self) -> Option<Note> {
        let current_beat = self.current_beat;
        let song = self.theme.as_mut().unwrap();

        let mut current_note = song.notes.get(song.cursor)?;
        //如果当前节拍大于当前音符的结束拍，关闭音符对应的LED，并切换音符
        if current_beat > current_note.end_beat {
            (self.play_sound)(0, true, self.volume);
            // let _ = chordes.turn_off(current_note.key);
            // hprintln(&format!("end note:{}", current_note.key));
            song.cursor += 1;
            //延迟一会儿
            let old_note = current_note.clone();
            current_note = song.notes.get(song.cursor)?;
            if current_note.key != old_note.key {
                self.delay.delay_us(1000 * 10u32);
            }
        }

        //如果当前节拍等于当前音符的起始拍，点亮LED
        if current_beat == current_note.start_beat {
            if current_note.delay {
                self.delay.delay_us(1000 * 10u32);
            }
            match current_note.key {
                NOTE_D6 => (self.play_sound)(note::NOTE_D6, true, self.volume),
                NOTE_B5 => (self.play_sound)(note::NOTE_B5, true, self.volume),
                NOTE_G5 => (self.play_sound)(note::NOTE_G5, true, self.volume),
                NOTE_E5 => (self.play_sound)(note::NOTE_E5, true, self.volume),
                NOTE_C5 => (self.play_sound)(note::NOTE_C5, true, self.volume),
                NOTE_A4 => (self.play_sound)(note::NOTE_A4, true, self.volume),
                NOTE_F4 => (self.play_sound)(note::NOTE_F4, true, self.volume),
                NOTE_D4 => (self.play_sound)(note::NOTE_D4, true, self.volume),
                NOTE_C4 => (self.play_sound)(note::NOTE_C4, true, self.volume),
                NOTE_E4 => (self.play_sound)(note::NOTE_E4, true, self.volume),
                NOTE_G4 => (self.play_sound)(note::NOTE_G4, true, self.volume),
                NOTE_B4 => (self.play_sound)(note::NOTE_B4, true, self.volume),
                NOTE_D5 => (self.play_sound)(note::NOTE_D5, true, self.volume),
                NOTE_F5 => (self.play_sound)(note::NOTE_F5, true, self.volume),
                NOTE_A5 => (self.play_sound)(note::NOTE_A5, true, self.volume),
                NOTE_C6 => (self.play_sound)(note::NOTE_C6, true, self.volume),
                NOTE_E6 => (self.play_sound)(note::NOTE_E6, true, self.volume),
                _ => (),
            };
            return Some(current_note.clone());
        }
        None
    }

    /// 播放主题曲和伴奏的下一个音符
    /// 此方法每隔八分之一拍调用一次
    pub fn play(&mut self) -> Option<Note> {
        if self.ended || self.theme.is_none() {
            self.ended = true;
            return None;
        }

        let theme = self.theme.as_mut().unwrap();

        //检查是否结束
        if self.current_beat == theme.total_beat {
            self.ended = true;
            return None;
        }

        let theme_note = self.play_note();

        //延时us(即微妙)
        self.delay.delay_us(self.time_per_beat);

        self.current_beat += 1;
        theme_note
    }

    pub fn ended(&self) -> bool {
        self.ended
    }

    pub fn get_theme(&self) -> Option<&Song> {
        self.theme.as_ref()
    }

    pub fn delay_ms(&mut self, ms: u32) {
        self.delay.delay_us(1000 * ms);
    }

    pub fn enable_speaker(&mut self, enable: bool) {
        (self.play_sound)(0, enable, self.volume);
    }

    pub fn sound(&mut self, freq: u32, duration_ms: u32) {
        (self.play_sound)(freq, true, 500);
        self.delay_ms(duration_ms);
    }

    pub fn set_volume(&mut self, volume: u16) {
        if volume >= 6 && volume <= 4000 {
            self.volume = volume;
        }
    }

    /// 重置
    pub fn reset(&mut self) {
        self.current_beat = 0;
        if let Some(theme) = self.theme.as_mut() {
            theme.cursor = 0;
        }
        self.ended = false;
    }
}

/// 解析音符
fn split_notes(data: &str) -> Option<Song> {
    // 分别解析每小节的音符
    let mut notes: Vec<Note> = Vec::new();
    let parts = data.split("|");

    //统计总共多少个八分之一拍
    let mut total_beat_count = 0;
    for part in parts {
        for note in part.split(",") {
            let mut note = note.to_string();
            let mut delay = false;
            if note.starts_with("delay") {
                println!("ends_with delay:{:?}", note);

                delay = true;
                note = note.replace("delay", "");
            }
            let mut symbols = note.chars();
            let key_name = symbols.next()?;
            let mut beat_count = 8; //默认为整拍
            if let Some(n) = symbols.next() {
                //读取音符节拍数
                beat_count = parse::<u16>(&n.to_string())?;
            }

            /*
                假设第一个音符是1拍，第二个和第三个音符都是半拍，第四个音符又是1拍
                那么第一个音符是从0开始，第二个音符是从8开始，第三个音符从12开始，第四个音符从16开始
            */
            notes.push(Note {
                start_beat: total_beat_count,
                end_beat: total_beat_count + beat_count - 1,
                key: key_name,
                delay,
                // name: key_name.to_string(),
            });

            //每个音符8个八分之一拍
            total_beat_count += beat_count;
        }
    }
    let song = Song {
        notes,
        cursor: 0,
        total_beat: total_beat_count,
    };
    Some(song)
}

/// 将字符串转换未某种类型的数字
fn parse<F: FromStr>(s: &str) -> Option<F> {
    match s.parse::<F>() {
        Ok(a) => Some(a),
        _ => None,
    }
}
