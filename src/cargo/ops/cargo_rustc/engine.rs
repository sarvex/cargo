use std::collections::HashMap;
use std::ffi::{AsOsStr, OsString};
use std::fmt;
use std::path::Path;
use std::process::Output;

use util::{CargoResult, ProcessError, ProcessBuilder, process};

/// Trait for objects that can execute commands.
pub trait ExecEngine: Send + Sync {
    fn exec(&self, CommandPrototype) -> Result<(), ProcessError>;
    fn exec_with_output(&self, CommandPrototype) -> Result<Output, ProcessError>;
}

/// Default implementation of `ExecEngine`.
#[derive(Copy)]
pub struct ProcessEngine;

impl ExecEngine for ProcessEngine {
    fn exec(&self, command: CommandPrototype) -> Result<(), ProcessError> {
        command.into_process_builder().exec()
    }

    fn exec_with_output(&self, command: CommandPrototype)
                        -> Result<Output, ProcessError> {
        command.into_process_builder().exec_with_output()
    }
}

/// Prototype for a command that must be executed.
#[derive(Clone)]
pub struct CommandPrototype {
    ty: CommandType,
    builder: ProcessBuilder,
}

impl CommandPrototype {
    pub fn new(ty: CommandType) -> CargoResult<CommandPrototype> {
        Ok(CommandPrototype {
            builder: try!(match ty {
                CommandType::Rustc => process("rustc"),
                CommandType::Rustdoc => process("rustdoc"),
                CommandType::Target(ref s) |
                CommandType::Host(ref s) => process(s),
            }),
            ty: ty,
        })
    }

    pub fn get_type(&self) -> &CommandType { &self.ty }

    pub fn arg<T: AsOsStr + ?Sized>(&mut self, arg: &T) -> &mut CommandPrototype {
        self.builder.arg(arg);
        self
    }

    pub fn args<T: AsOsStr>(&mut self, arguments: &[T]) -> &mut CommandPrototype {
        self.builder.args(arguments);
        self
    }

    pub fn cwd<T: AsOsStr + ?Sized>(&mut self, path: &T) -> &mut CommandPrototype {
        self.builder.cwd(path);
        self
    }

    pub fn env<T: AsOsStr + ?Sized>(&mut self, key: &str, val: &T)
                                    -> &mut CommandPrototype {
        self.builder.env(key, val);
        self
    }

    pub fn get_args(&self) -> &[OsString] { self.builder.get_args() }
    pub fn get_cwd(&self) -> &Path { self.builder.get_cwd() }

    pub fn get_env(&self, var: &str) -> Option<OsString> {
        self.builder.get_env(var)
    }

    pub fn get_envs(&self) -> &HashMap<String, Option<OsString>> {
        self.builder.get_envs()
    }

    pub fn into_process_builder(self) -> ProcessBuilder {
        self.builder
    }
}

impl fmt::Display for CommandPrototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.builder.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub enum CommandType {
    Rustc,
    Rustdoc,

    /// The command is to be executed for the target architecture.
    Target(OsString),

    /// The command is to be executed for the host architecture.
    Host(OsString),
}
