use anyhow::Result;
use clap::{Parser, Subcommand};
use hyprland::{
    dispatch::*,
    prelude::*,
    shared::{Address, MonitorId, WorkspaceId},
};

mod error;

fn main() {
    println!("Hello, world!");
}
