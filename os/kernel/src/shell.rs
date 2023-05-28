use crate::console::{kprint, kprintln, CONSOLE};
use stack_vec::StackVec;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

const MAX_CMD_LEN: usize = 512;
const MAX_ARG_NUM: usize = 64;

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    kprintln!("Welcome!");
    loop {
        kprint!("{}", prefix);

        let cmd_buf = &mut [0; MAX_CMD_LEN];
        let args_buf = &mut [""; MAX_ARG_NUM];
        match Command::parse(read_line(cmd_buf), args_buf) {
            Ok(cmd) => execute_cmd(cmd),
            Err(Error::TooManyArgs) => kprintln!("error: too many arguments"),
            Err(Error::Empty) => continue,
        }
    }
}

fn execute_cmd(cmd: Command) {
    match cmd.path() {
        "echo" => match &cmd.args.as_slice()[1..] {
            [heads @ .., tail] => {
                heads.iter().for_each(|arg| kprint!("{} ", arg));
                kprintln!("{}", tail);
            }
            _ => kprintln!(),
        },
        path => kprintln!("unknown command: {}", path),
    }
}

fn read_line(buf: &mut [u8]) -> &str {
    let mut cmd_buf = StackVec::new(buf);

    loop {
        let b = CONSOLE.lock().read_byte();
        match b {
            // enter
            b'\r' | b'\n' => {
                kprintln!();
                break;
            }
            // printable
            0x20..=0x7e => match cmd_buf.push(b) {
                Err(_) => ring_bell(),
                Ok(_) => CONSOLE.lock().write_byte(b),
            },
            // backspace and delete
            8 | 127 => match cmd_buf.pop() {
                Some(_) => kprint!("\u{8} \u{8}"),
                None => ring_bell(),
            },
            // other non-visiable
            _ => ring_bell(),
        }
    }

    unsafe { std::str::from_utf8_unchecked(cmd_buf.into_slice()) }
}

fn ring_bell() {
    CONSOLE.lock().write_byte(7);
}
