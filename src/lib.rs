#![feature(int_log)]

extern crate core;

pub mod board;
mod math;
pub mod player;
mod ships;

pub enum Message {
    NoHit,
    HitOne,
    HitTwo,
    Sunk(u64),
}
