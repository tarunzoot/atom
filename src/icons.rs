#![allow(dead_code)]
use crate::{
    font::{ICOFONT, SYMBOLS},
    style::AtomTheme,
};
use iced::widget::{text, Text};

fn icon_icofont<'a>(codepoint: char) -> Text<'a, AtomTheme> {
    text(codepoint).font(ICOFONT).size(16).center()
}

fn icon_symbols<'a>(codepoint: char) -> Text<'a, AtomTheme> {
    text(codepoint).font(SYMBOLS).size(30).center()
}

pub fn electron<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ead8}')
}

pub fn right_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ea79}')
}

pub fn left<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eabf}')
}

pub fn left_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ea78}')
}

pub fn down_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ea77}')
}

pub fn up_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ea7a}')
}

pub fn maximize<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef52}')
}

pub fn overview<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{e944}')
}

pub fn settings<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec83}')
}

pub fn keyboard<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ea54}')
}

pub fn search<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ed11}')
}

pub fn theme<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec88}')
}

pub fn list<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef72}')
}

pub fn list_line_dots<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef74}')
}

pub fn recycle_bin<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{edec}')
}

pub fn file_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eb08}')
}

pub fn file_size<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{e90b}')
}

pub fn clock<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{f022}')
}

pub fn check_circled<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eed7}')
}

pub fn speedmeter<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eff3}')
}

pub fn calendar<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec45}')
}

pub fn bullhorn<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eecb}')
}

pub fn cloud_download<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eee5}')
}

pub fn tick_boxed<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{f00d}')
}

pub fn close_circled<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eedd}')
}

pub fn close_line<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eee1}')
}

pub fn close_line_circled<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eede}')
}

pub fn info_circle<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{e4fe}')
}

pub fn harddisk<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef43}')
}

pub fn down_zigzag<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eabe}')
}

pub fn plus<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{efc2}')
}

pub fn plus_circle<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{efc0}')
}

pub fn minus<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef9a}')
}

pub fn exit<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef1d}')
}

pub fn expand<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef1e}')
}

pub fn social_link<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec84}')
}

pub fn play_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eca8}')
}

pub fn pause_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eca5}')
}

pub fn envelope_open<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef13}')
}

pub fn trash_bin_closed<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ee09}')
}

pub fn trash_bin_open<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec53}')
}

pub fn pause<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec72}')
}

pub fn reply<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec7f}')
}

pub fn play<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec74}')
}

pub fn rotation<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec80}')
}

pub fn folder<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ef36}')
}

pub fn calculator<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ec05}')
}

pub fn hash<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{ed68}')
}

pub fn joining<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{e984}')
}

pub fn spinner<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{eff4}')
}

pub fn chart_alt<'a>() -> Text<'a, AtomTheme> {
    icon_icofont('\u{e982}')
}

pub fn grip3<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{ee57}')
}

pub fn download_alt<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{f019}')
}

pub fn layout_statusbar<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{ebf5}')
}

pub fn box_open<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{ed95}')
}

pub fn command<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{f0633}')
}

pub fn ctrl<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{f0634}')
}

pub fn failed<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{f0164}')
}

pub fn harddisk_alt<'a>() -> Text<'a, AtomTheme> {
    icon_symbols('\u{f0193}')
}
