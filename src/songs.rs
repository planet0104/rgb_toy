/*
x4 八分之四拍 (1减时线 二分之一拍)
x6 八分之六拍 (1减时线、1附点 四分之三拍)
x2 八分之二拍 (2减时线 四分之一拍)
x3 八分之三拍 (2减时线、1附点)
x1 八分之一拍 (3减时线)
x12 八分之十二拍 (1附点, 一又二分之一拍， 实际使用时为了对其伴奏可能不会这么写)
其中x为具体音符，包括休止符、曾时线
 */

// 两只老虎
pub const TWO_TIGER: &str = "4,160_c,d,e,c|delayc,d,e,c|e,f,g,-|e,f,g,-|g6,a2,g6,f2,e,c|g6,a2,g6,f2,e,c|delayc,5,c,-|c,5,c,-";
// 新年好
const HAPPY_NEW_YEAR:&str = "3,120_c4,delayc4|delayc,5,e4,delaye4|delaye,c,delayc4,e4|g,delayg,f4,e4|d,-,d4,e4|f,delayf,e4,d4|e,c,delayc4,e4|d,5,74,d4|c,-";
// 祝你生日快乐 [简谱https://www.wenanjuzi.com/1063850280.html]
pub const HAPPY_BIRTHDAY: &str =
    "3,130_54,delay54|6,5,c|7,-,54,delay54|6,5,d|c,-,54,delay54|g,e,c|7,6,-,f4,delayf4|e,c,d|c,-";
//世上只有妈妈好 [简谱http://www.jianpu.cn/pu/36/36151.htm]
pub const GREAT_MOTHER:&str = "4,140_a,a4,g4,e,g|C,a4,g4,a,-|e,g4,a4,g,e|c4,64,g4,e4,d,-|d,d4,e4,g,delayg4,a4|e,d,c,-|g,g4,e4,d4,c4,64,c4|5,5,-,-";
//摇篮曲(舒伯特) [简谱http://www.jianpuw.com/htm/iy/304672.htm]
pub const LULLABIES:&str = "4,100_e,g,d4,e4,f|e4,delaye4,d4,74,d,5|e,g,d6,e2,f|e4,delaye4,d2,e2,f2,d2,c,-|d,d4,delayd4,e4,e2,d2,c|g,f4,e4,d,5|e,g,d6,e2,f|e4,delaye4,d2,e2,f2,d2,c,-";
//我爱北京天安门 [简谱http://www.tom163.net/yuepuku/ergegepu/liuziyishangergegepu/200608/18633.html]
pub const TIAN_AN_MEN:&str = "2,140_56,c2,54,44|34,24,1|delay14,delay14,24,34|delay34,14,34,44|5,5|-|56,c2,54,44|34,54,2|46,32,24,64|5,24,34|1,1|-|5,54,34|6,c|7,64,74|5,3|d6,delayd2,delayd4,c4|7,64,c4|5,5|-|5,54,34|6,c|74,64,74,c4|d,-|56,62,74,c4|d,5|c,c|-|56,c2,54,44|34,24,1|delay14,delay14,24,34|delay34,14,34,44|5,5|-|56,c2,54,44|34,54,2|46,32,24,64|5,64,74|c,c|-,-";
//小燕子 [简谱http://www.jianpu.cn/pu/83/83971.htm]
pub const XIAO_YAN_ZI:&str = "4,130_34,54,c4,64,5,-|34,54,64,c4,5,-|c,c4,e4,d,c|d4,c4,64,c4,5,-|3,34,54,6,54,64|c,d4,54,6,-|34,24,1,2,-|2,delay24,34,5,delay5|c,24,34,5,-|34,54,c4,64,5,-|34,54,64,c4,5,-|c,c4,e4,d,c|d4,c4,64,c4,5,-|3,34,54,6,54,64|c,d4,54,6,-|3,34,c4,6,5|34,24,1,2,-|2,24,34,5,-|c,c4,e4,d,c|d4,c4,54,64,c,-";
//春天在哪里 [简谱https://www.jianpu.org/ertong/8241.html]
pub const SPRING:&str = "2,140_delaye4,delaye4,delaye4,c4|5,delay54,04|e4,delaye4,delaye4,c4|e,-|g4,delayg4,e4,c4|54,delay54,delay5|64,74,c4,e4|d,-|e4,delaye4,delaye4,c4|5,delay54,04|e4,delaye4,delaye4,c4|e,-|g4,a4,g4,a4|g4,f4,e4,c4|54,04,e4,04|d4,c4,-|f4,delayf4,delayf4,g4|a4,delaya4,delaya4,04|d4,delayd4,delayd4,delayd4|g,-|c4,delayc4,delayc4,d4|e4,delaye4,delaye4,04|54,delay54,delay54,delay54|d,-|g4,a4,g4,a4|g4,f4,e4,c4|d,5|c4,e4,-|g4,a4,g4,a4|g4,f4,e4,c4|54,04,e4,04|d4,c4,-";
//小小世界 [简谱http://www.jianpu.cn/pu/41/417146.htm]
pub const SMALL_WORLD:&str = "2,140_34,44|5,e|c,d4,c4|delayc,7|delay7,24,34|4,d|7,c4,74|6,5|delay5,34,44|5,c4,d4|e,d4,c4|6,d4,e4|f,e4,d4|5,f|e,d|c,c|c,-|c,c4,delayc4|e,c|d,d4,delayd4|delayd,delayd|delayd,d4,delayd4|f,d|e,e4,delaye4|delaye,e|delaye,e4,delaye4|g,e|f,f4,delayf4|delayf,e4,d4|5,f|e,d|c,c|c,-";
//春江花月夜(片段) [简谱http://www.jianpu.cn/pu/42/424486.htm]
pub const SPRING_RIVER_MOON_NIGHT:&str = "2,60_a4,delaya2,delaya2,C4,D2,a2|g,g6,a2|g4,delayg4,a4,C2,D2|e,e|delaye4,d2,e2,g4,e2,g2|a6,C2,D6,E2|C4,D2,E2,D2,C2,a4|g,delayg6,C2|a2,C2,D4,a2,C2,g2,d2|e,e|delaye4,a2,C2,g2,a2,g2,e2|d,d|e6,g2,a2,g2,a2,C2|d4,e2,d2,c2,d2,e2,c2|d,d4,04";
//蝶恋 [简谱http://www.jianpu.cn/pu/45/45397.htm]
pub const BUTTERFLY_LOVE: &str = r#"4,110_
                                e4,delaye4,delaye4,d4,e,e|
                                d4,e4,d4,delayd4,6,delay64,74|c,d4,c4,7,64,54|6,6,6,6|e4,delaye4,delaye4,d4,e,e4,a4|g4,a4,g4,delayg4,d,delayd4,e4|
                                f,g4,f4,e,d4,c4|e,e,e4,delaye4|a,b4,a4,g,g4,e4|g,g,g,e4,g4|d,a4,g4,e,d4,delayd4|
                                e,e,e,e|d,a4,delaya4,a,a|c,a4,delaya4,a,delaya4,b4|C,b4,a4,b,a4,b4|e,e,e,e4,delaye4|a,b4,a4,g,g4,e4|
                                g,g,g,f4,g4|a,b4,a4,b,a4,b4|e,e,e,e|d,a4,delaya4,a,a|c,a4,delaya4,a,delaya4,b4|C,b4,a4,b,g|
                                a,a,a,a|E4,D4,delayD4,C4,delayC4,b4,delayb4,a4|C4,b4,delayb4,a4,delaya4,g4,delayg4,f4|a4,g4,delayg4,f4,delayf4,e4,delaye4,d4|e,e,e,e|-
                                "#;

//梁祝 [简谱:比三呆吉他]
pub const LIANG_ZHU: &str = r#"4,56_
                            3,56,62,c4,32,d2,62,c2,54|g4,delay52,C2,a2,g2,e2,g2,d,-|d4,delayd2,e2,74,64,56,62,c4,d4|34,c4,62,52,62,c2,5,5|
                            e4,52,g2,74,d4,62,c2,54,5|33,51,34,52,62,72,d2,64,34,12,32,52,62|c4,52,d2,g4,e4,d4,e2,d2,c4,62,52|3,c,63,c1,62,52,32,52,62,c2|
                            5,54,e2,g2,d2,e2,d2,c2,74,64|3,56,62,c4,32,d2,62,c2,54|g4,52,C2,a2,g2,e2,g2,d,0|d4,delayd2,e2,74,64,56,62,c4,d4|
                            34,c4,62,52,62,c2,5,5|e4,52,g2,74,d4,62,c2,54,5|33,51,34,52,62,72,d2,64,34,12,32,52,62|c4,52,d2,g4,e4,d4,e2,d2,c4,62,52|
                            3,c,63,c1,62,52,32,52,62,c2|5,54,e2,g2,d2,e2,d2,c2,74,64|5,54,e2,g2,d2,e2,d2,c2,74,d4|c,c,-"#;

//欢乐斗地主BGM1 [简谱:比三呆吉他]
pub const DOU_DI_ZHU_BGM1: &str = r#"
                                4,110_
                                g4,delayg2,a2,g4,e4,g,0|c4,delayc2,d2,c4,64,5,0|
                                64,delay62,52,64,c4,d4,delayd2,c2,d4,e4|g4,delayg2,a2,g4,e4,d,0|g4,delayg2,a2,g4,e4,g,0|
                                d4,delayd2,e2,d4,c4,6,0|54,64,c4,d4,e4,a4,e4,g4|d4,delayd2,e2,d4,64,c4,c4|
                                a4,delaya2,g2,e4,g4,a,a4,e4|g4,delayg2a2,g4,e4,g,g|d4,delayd2,e2,d4,c4,d4,c4,64,54|
                                d,d4,c4,e,e|a4,delaya2,g2,e4,g4,a,a4,e4|g4,delayg2,a2,g4,e4,g,g|
                                d4,delayd2,e2,d4,c4,d4,c4,64,54|c,c,C,C|-
                                "#;

//欢乐斗地主BGM2 [简谱:比三呆吉他]
pub const DOU_DI_ZHU_BGM2: &str = r#"
                                4,76_
                                54,delay52,62,c2,52,62,c2,g2,delayg2,delayg2,e2,g|54,delay52,62,c2,52,62,e2,d2,delayd2,delayd2,c2,d|e4,delaye2,d2,e4,g4,a2,C2,a2,g2,a4,g2,e2|
                                d4,a2,e2,d2,g2,e2,d2,c4,delayc2,62,c|54,delay52,62,c2,52,62,c2,g2,delayg2,delayg2,e2,g|54,delay52,62,c2,52,62,e2,d2,delayd2,delayd2,c2,d|
                                e4,delaye2,d2,e4,g4,a2,C2,a2,g2,a4,g2,e2|d4,a2,e2,d2,g2,e2,d2,c4,64,c|d3,e1,d2,e2,g2,a2,D4,C4,g4,C|-
                                "#;

pub const MUSICS: &[&str] = &[
    //我爱北京天安门
    TIAN_AN_MEN,
    //斗地主BGM1
    DOU_DI_ZHU_BGM1,
    //斗地主BGM2
    DOU_DI_ZHU_BGM2,
    //生日快乐
    HAPPY_BIRTHDAY,
    //新年好
    HAPPY_NEW_YEAR,
    //世上只有妈妈好
    GREAT_MOTHER,
    //两只老虎
    TWO_TIGER,
    //摇篮曲
    LULLABIES,
    //小小世界
    SMALL_WORLD,
    //春天在哪里
    SPRING,
    //小燕子
    XIAO_YAN_ZI,
    //春江花月夜
    SPRING_RIVER_MOON_NIGHT,
    //蝶恋
    BUTTERFLY_LOVE,
    //梁祝
    LIANG_ZHU,
];

pub const FIRST_MUSIC: usize = 0;
