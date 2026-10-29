use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::report::{Decision, Event, EventKind};

/// What this run may touch. Empty means deny.
#[derive(Clone, Debug, Default)]
pub struct Grants {
    pub dirs: Vec<DirGrant>,
    pub net: Vec<NetGrant>,
    pub env: Vec<EnvGrant>,
}

#[derive(Clone, Debug)]
pub struct DirGrant {
    pub host: PathBuf,
    pub guest: String,
    pub writable: bool,
}

#[derive(Clone, Debug)]
pub struct NetGrant {
    pub addr: IpAddr,
    pub port: Option<u16>,
}

impl NetGrant {
    pub(crate) fn allows(&self, addr: SocketAddr) -> bool {
        if addr.ip() != self.addr {
            return false;
        }
        self.port.is_none_or(|port| port == addr.port())
    }
}

#[derive(Clone, Debug)]
pub struct EnvGrant {
    pub name: String,
    pub value: String,
}

impl DirGrant {
    /// Parse `host:guest`, `host:guest:ro`, or `host:guest:rw`.
    ///
    /// The split is the last `:`. A missing mode is read-only. Guest paths
    /// cannot contain `:`.
    pub fn parse(spec: &str) -> Result<Self, Error> {
        let (spec, writable) = if let Some(rest) = spec.strip_suffix(":rw") {
            (rest, true)
        } else if let Some(rest) = spec.strip_suffix(":ro") {
            (rest, false)
        } else {
            (spec, false)
        };
        let (host, guest) = spec.rsplit_once(':').ok_or_else(|| {
            Error::BadGrant(format!(
                "directory grant `{spec}` must look like host:guest[:ro|rw]"
            ))
        })?;
        if host.is_empty() || guest.is_empty() {
            return Err(Error::BadGrant(format!(
                "directory grant `{spec}` needs both a host path and a guest path"
            )));
        }
        Ok(Self {
            host: PathBuf::from(host),
            guest: guest.to_string(),
            writable,
        })
    }
}

impl NetGrant {
    /// Parse `ip` or `ip:port`. Any port is allowed when the port is omitted.
    pub fn parse(spec: &str) -> Result<Self, Error> {
        if let Ok(addr) = spec.parse::<IpAddr>() {
            return Ok(Self { addr, port: None });
        }
        if let Ok(sock) = spec.parse::<SocketAddr>() {
            return Ok(Self {
                addr: sock.ip(),
                port: Some(sock.port()),
            });
        }
        Err(Error::BadGrant(format!(
            "network grant `{spec}` must be an IP address or ip:port"
        )))
    }
}

impl EnvGrant {
    /// Parse `NAME` (copied from the host environment) or `NAME=VALUE`.
    pub fn parse(spec: &str) -> Result<Self, Error> {
        if spec.is_empty() || spec.contains('\0') {
            return Err(Error::BadGrant(
                "environment grant name must not be empty".into(),
            ));
        }
        if let Some((name, value)) = spec.split_once('=') {
            if name.is_empty() {
                return Err(Error::BadGrant(
                    "environment grant name must not be empty".into(),
                ));
            }
            return Ok(Self {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
        let value = std::env::var(spec).map_err(|_| {
            Error::BadGrant(format!(
                "environment grant `{spec}` is not set in the host environment"
            ))
        })?;
        Ok(Self {
            name: spec.to_string(),
            value,
        })
    }
}

impl Grants {
    pub(crate) fn validate(&self) -> Result<Vec<String>, Error> {
        let mut guests = Vec::new();
        let mut warnings = Vec::new();
        for dir in &self.dirs {
            if guests.contains(&dir.guest) {
                return Err(Error::BadGrant(format!(
                    "duplicate guest path {}",
                    dir.guest
                )));
            }
            guests.push(dir.guest.clone());
            let meta =
                std::fs::metadata(&dir.host).map_err(|_| Error::InvalidGrant(dir.host.clone()))?;
            if !meta.is_dir() {
                return Err(Error::InvalidGrant(dir.host.clone()));
            }
            if let Some(warning) = broad_dir_warning(&dir.host) {
                warnings.push(warning);
            }
        }
        Ok(warnings)
    }
}

fn broad_dir_warning(host: &Path) -> Option<String> {
    let canonical = host.canonicalize().ok()?;
    if is_fs_root(&canonical) {
        return Some(format!(
            "directory grant {} is the filesystem root; the guest can read everything under it",
            host.display()
        ));
    }
    let home = std::env::var_os("HOME")?;
    let home = PathBuf::from(home).canonicalize().ok()?;
    if canonical == home {
        return Some(format!(
            "directory grant {} is the home directory",
            host.display()
        ));
    }
    None
}

fn is_fs_root(path: &Path) -> bool {
    path.parent()
        .is_none_or(|parent| parent.as_os_str().is_empty())
}

pub(crate) fn event(kind: EventKind, target: impl Into<String>, allowed: bool) -> Event {
    Event {
        kind,
        target: target.into(),
        decision: if allowed {
            Decision::Allowed
        } else {
            Decision::Denied
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dir_grant_modes() {
        let ro = DirGrant::parse("/tmp/host:guest").unwrap();
        assert_eq!(ro.host, PathBuf::from("/tmp/host"));
        assert_eq!(ro.guest, "guest");
        assert!(!ro.writable);

        let rw = DirGrant::parse("/tmp/host:guest:rw").unwrap();
        assert!(rw.writable);

        let explicit = DirGrant::parse("/tmp/host:work:ro").unwrap();
        assert_eq!(explicit.guest, "work");
        assert!(!explicit.writable);
    }

    #[test]
    fn parses_net_and_env_grants() {
        let any_port = NetGrant::parse("127.0.0.1").unwrap();
        assert!(any_port.port.is_none());
        assert!(any_port.allows("127.0.0.1:9".parse().unwrap()));
        assert!(!any_port.allows("127.0.0.2:9".parse().unwrap()));

        let one_port = NetGrant::parse("127.0.0.1:9").unwrap();
        assert!(one_port.allows("127.0.0.1:9".parse().unwrap()));
        assert!(!one_port.allows("127.0.0.1:10".parse().unwrap()));

        let env = EnvGrant::parse("BOXWREN_TEST_VALUE=open").unwrap();
        assert_eq!(env.name, "BOXWREN_TEST_VALUE");
        assert_eq!(env.value, "open");
    }
}
