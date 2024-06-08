use poise::Command;
use crate::commands::hello;

pub fn get_commands() -> Vec<Command<crate::Data, crate::Error>> {
    vec![
        (hello::hello())
    ]
}