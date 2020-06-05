#[cfg(target_os = "linux")]
use nix::unistd::{execvp, getgid, getpid, getuid, setgid, setuid};
#[cfg(target_os = "linux")]
use std::{
    env,
    error::Error as StdError,
    ffi::{CStr, CString, NulError},
    fs,
    io::{self, BufWriter, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
};

#[cfg(target_os = "linux")]
use talpid_types::SPLIT_TUNNEL_CGROUP_NAME;

#[cfg(target_os = "linux")]
const NETCLS_DIR: &str = "/sys/fs/cgroup/net_cls/";

#[cfg(target_os = "linux")]
const PROGRAM_NAME: &str = "mullvad-exclude";

#[cfg(target_os = "linux")]
#[derive(err_derive::Error, Debug)]
#[error(no_from)]
enum Error {
    #[error(display = "Invalid arguments")]
    InvalidArguments,

    #[error(display = "Cannot set the cgroup")]
    AddProcToCGroup(#[error(source)] io::Error),

    #[error(display = "Failed to drop root user privileges for the process")]
    DropRootUid(#[error(source)] nix::Error),

    #[error(display = "Failed to drop root group privileges for the process")]
    DropRootGid(#[error(source)] nix::Error),

    #[error(display = "Failed to launch the process")]
    Exec(#[error(source)] nix::Error),

    #[error(display = "An argument contains interior nul bytes")]
    ArgumentNulError(#[error(source)] NulError),
}

fn main() {
    #[cfg(target_os = "linux")]
    match run() {
        Err(Error::InvalidArguments) => {
            let mut args = env::args();
            let program = args.next().unwrap_or(PROGRAM_NAME.to_string());
            eprintln!("Usage {} COMMAND [ARGS]", program);
            std::process::exit(1);
        }
        Err(e) => {
            let mut s = format!("{}", e);
            let mut source = e.source();
            while let Some(error) = source {
                s.push_str(&format!("\nCaused by: {}", error));
                source = error.source();
            }
            eprintln!("{}", s);

            std::process::exit(1);
        }
        _ => unreachable!("execv returned unexpectedly"),
    }
}

#[cfg(target_os = "linux")]
fn run() -> Result<void::Void, Error> {
    let mut args_iter = env::args_os().skip(1);
    let program = args_iter.next().ok_or(Error::InvalidArguments)?;
    let program = CString::new(program.as_bytes()).map_err(Error::ArgumentNulError)?;

    let args: Vec<CString> = env::args_os()
        .skip(1)
        .map(|arg| CString::new(arg.as_bytes()))
        .collect::<Result<Vec<CString>, NulError>>()
        .map_err(Error::ArgumentNulError)?;
    let args: Vec<&CStr> = args.iter().map(|arg| &**arg).collect();

    // Set the cgroup of this process
    let cgroup_dir = Path::new(NETCLS_DIR).join(SPLIT_TUNNEL_CGROUP_NAME);
    let procs_path = cgroup_dir.join("cgroup.procs");

    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(procs_path)
        .map_err(Error::AddProcToCGroup)?;

    BufWriter::new(file)
        .write_all(getpid().to_string().as_bytes())
        .map_err(Error::AddProcToCGroup)?;

    // Drop root privileges
    let real_uid = getuid();
    setuid(real_uid).map_err(Error::DropRootUid)?;
    let real_gid = getgid();
    setgid(real_gid).map_err(Error::DropRootGid)?;

    // Launch the process
    execvp(&program, &args).map_err(Error::Exec)
}
