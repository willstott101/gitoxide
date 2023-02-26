use std::{ffi::OsStr, io::ErrorKind};

use bstr::{BString, ByteSlice, ByteVec};

use crate::{
    client::{ssh, ssh::ProgramKind},
    Protocol,
};

impl ProgramKind {
    /// Provide the name of the executable that belongs to this kind, or `None` if the kind is `Simple`.
    pub fn exe(&self) -> Option<&'static OsStr> {
        Some(OsStr::new(match self {
            ProgramKind::Ssh => "ssh",
            ProgramKind::Plink => "plink",
            ProgramKind::Putty => "putty",
            ProgramKind::TortoisePlink => "tortoiseplink.exe",
            ProgramKind::Simple => return None,
        }))
    }

    /// Prepare all information needed to invoke the ssh command
    pub(crate) fn prepare_invocation(
        &self,
        ssh_cmd: &OsStr,
        url: &gix_url::Url,
        desired_version: Protocol,
        disallow_shell: bool,
    ) -> Result<gix_command::Prepare, ssh::invocation::Error> {
        let mut prepare = gix_command::prepare(ssh_cmd).with_shell();
        if disallow_shell {
            prepare.use_shell = false;
        }
        let host = url.host().expect("present in ssh urls");
        match self {
            ProgramKind::Ssh => {
                if desired_version != Protocol::V1 {
                    prepare = prepare
                        .args(["-o", "SendEnv=GIT_PROTOCOL"])
                        .env("GIT_PROTOCOL", format!("version={}", desired_version as usize))
                }
                if let Some(port) = url.port {
                    prepare = prepare.arg(format!("-p{port}"));
                }
            }
            ProgramKind::Plink | ProgramKind::Putty | ProgramKind::TortoisePlink => {
                if *self == ProgramKind::TortoisePlink {
                    prepare = prepare.arg("-batch");
                }
                if let Some(port) = url.port {
                    prepare = prepare.arg("-P");
                    prepare = prepare.arg(port.to_string());
                }
            }
            ProgramKind::Simple => {
                if url.port.is_some() {
                    return Err(ssh::invocation::Error {
                        command: ssh_cmd.into(),
                        function: "setting the port",
                    });
                }
            }
        };
        let host_as_ssh_arg = match url.user() {
            Some(user) => format!("{user}@{host}"),
            None => host.into(),
        };

        // Try to force ssh to yield english messages (for parsing later)
        Ok(prepare.arg(host_as_ssh_arg).env("LANG", "C").env("LC_ALL", "C"))
    }

    /// Note that the caller has to assure that the ssh program is launched in English by setting the locale.
    pub(crate) fn line_to_err(&self, line: BString) -> Result<std::io::Error, BString> {
        let kind = match self {
            ProgramKind::Ssh | ProgramKind::Simple => {
                if line.contains_str(b"Permission denied") || line.contains_str(b"permission denied") {
                    Some(ErrorKind::PermissionDenied)
                } else if line.contains_str(b"resolve hostname") {
                    Some(ErrorKind::ConnectionRefused)
                } else if line.contains_str(b"connect to host")
                    || line.contains_str("Connection to ")
                    || line.contains_str("Connection closed by ")
                {
                    // TODO: turn this into HostUnreachable when stable, or NetworkUnreachable in 'no route' example.
                    //       It's important that it WON'T be considered spurious, but is considered a permanent failure.
                    Some(ErrorKind::NotFound)
                } else {
                    None
                }
            }
            ProgramKind::Plink | ProgramKind::Putty | ProgramKind::TortoisePlink => {
                if line.contains_str(b"publickey") {
                    Some(ErrorKind::PermissionDenied)
                } else {
                    None
                }
            }
        };
        match kind {
            Some(kind) => Ok(std::io::Error::new(kind, Vec::from(line).into_string_lossy())),
            None => Err(line),
        }
    }
}

impl<'a> From<&'a OsStr> for ProgramKind {
    fn from(v: &'a OsStr) -> Self {
        let p = std::path::Path::new(v);
        match p.file_stem().and_then(|s| s.to_str()) {
            None => ProgramKind::Simple,
            Some(stem) => {
                if stem.eq_ignore_ascii_case("ssh") {
                    ProgramKind::Ssh
                } else if stem.eq_ignore_ascii_case("plink") {
                    ProgramKind::Plink
                } else if stem.eq_ignore_ascii_case("putty") {
                    ProgramKind::Putty
                } else if stem.eq_ignore_ascii_case("tortoiseplink") {
                    ProgramKind::TortoisePlink
                } else {
                    ProgramKind::Simple
                }
            }
        }
    }
}
