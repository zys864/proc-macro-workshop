#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Command {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Command",
            "executable",
            &self.executable,
            "args",
            &self.args,
            "env",
            &self.env,
            "current_dir",
            &&self.current_dir,
        )
    }
}
struct CommandBuilder {
    executable: std::option::Option<String>,
    args: std::option::Option<Vec<String>>,
    env: std::option::Option<Vec<String>>,
    current_dir: std::option::Option<String>,
}
#[automatically_derived]
impl ::core::fmt::Debug for CommandBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "CommandBuilder",
            "executable",
            &self.executable,
            "args",
            &self.args,
            "env",
            &self.env,
            "current_dir",
            &&self.current_dir,
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for CommandBuilder {
    #[inline]
    fn default() -> CommandBuilder {
        CommandBuilder {
            executable: ::core::default::Default::default(),
            args: ::core::default::Default::default(),
            env: ::core::default::Default::default(),
            current_dir: ::core::default::Default::default(),
        }
    }
}
impl CommandBuilder {
    pub fn executable(mut self, v: impl Into<String>) -> Self {
        self.executable = Some(v.into());
        self
    }
    pub fn args(mut self, v: impl Into<Vec<String>>) -> Self {
        self.args = Some(v.into());
        self
    }
    pub fn env(mut self, v: impl Into<Vec<String>>) -> Self {
        self.env = Some(v.into());
        self
    }
    pub fn current_dir(mut self, v: impl Into<String>) -> Self {
        self.current_dir = Some(v.into());
        self
    }
    pub fn build(mut self) -> Result<Command, &'static str> {
        Ok(Command {
            executable: self.executable.take().ok_or("executable must be setted")?,
            args: self.args.take().ok_or("args must be setted")?,
            env: self.env.take().ok_or("env must be setted")?,
            current_dir: self.current_dir.take(),
        })
    }
}
impl Command {
    fn builder() -> CommandBuilder {
        Default::default()
    }
}
fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new(["build".to_owned(), "--release".to_owned()]),
            ),
        )
        .env(::alloc::vec::Vec::new())
        .build()
        .unwrap();
    if !command.current_dir.is_none() {
        ::core::panicking::panic("assertion failed: command.current_dir.is_none()")
    }
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new(["build".to_owned(), "--release".to_owned()]),
            ),
        )
        .env(::alloc::vec::Vec::new())
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    if !command.current_dir.is_some() {
        ::core::panicking::panic("assertion failed: command.current_dir.is_some()")
    }
    {
        ::std::io::_print(format_args!("{0:?}\n", command));
    };
}
