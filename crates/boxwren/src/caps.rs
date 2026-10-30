use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
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

/// A network destination the guest may reach.
///
/// Literal IP grants never enable DNS. Hostname grants turn on WASI name
/// lookup for the run and are re-resolved when a connect is checked.
#[derive(Clone, Debug)]
pub struct NetGrant {
    pub target: NetTarget,
    pub port: Option<u16>,
}

#[derive(Clone, Debug)]
pub enum NetTarget {
    Ip(IpAddr),
    Host(String),
}

impl NetGrant {
    pub fn ip(addr: IpAddr, port: Option<u16>) -> Self {
        Self {
            target: NetTarget::Ip(addr),
            port,
        }
    }

    pub fn host(name: impl Into<String>, port: Option<u16>) -> Self {
        Self {
            target: NetTarget::Host(name.into()),
            port,
        }
    }

    pub fn is_hostname(&self) -> bool {
        matches!(self.target, NetTarget::Host(_))
    }

    pub(crate) fn allows(&self, addr: SocketAddr) -> bool {
        if !self.port.is_none_or(|port| port == addr.port()) {
            return false;
        }
        match &self.target {
            NetTarget::Ip(ip) => addr.ip() == *ip,
            NetTarget::Host(name) => resolve_host(name)
                .map(|ips| ips.iter().any(|ip| *ip == addr.ip()))
                .unwrap_or(false),
        }
    }

    pub(crate) fn label_for(&self, addr: SocketAddr) -> String {
        match &self.target {
            NetTarget::Ip(_) => addr.to_string(),
            NetTarget::Host(name) => format!("{name}/{addr}"),
        }
    }
}

fn resolve_host(name: &str) -> Result<Vec<IpAddr>, std::io::Error> {
    // Port 0 is only used so ToSocketAddrs accepts a host string.
    (name, 0u16)
        .to_socket_addrs()
        .map(|iter| iter.map(|addr| addr.ip()).collect())
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
    /// Parse `ip`, `ip:port`, `hostname`, or `hostname:port`.
    ///
    /// A bare address allows every port. Hostnames enable DNS for the run.
    pub fn parse(spec: &str) -> Result<Self, Error> {
        if let Ok(addr) = spec.parse::<IpAddr>() {
            return Ok(Self::ip(addr, None));
        }
        if let Ok(sock) = spec.parse::<SocketAddr>() {
            return Ok(Self::ip(sock.ip(), Some(sock.port())));
        }
        // hostname or hostname:port — reject empty labels and spaces.
        if spec.is_empty() || spec.contains(' ') || spec.contains('/') {
            return Err(Error::BadGrant(format!(
                "network grant `{spec}` must be an IP, ip:port, hostname, or hostname:port"
            )));
        }
        if let Some((host, port)) = spec.rsplit_once(':')
            && !host.is_empty()
            && !host.contains(':')
            && let Ok(port) = port.parse::<u16>()
        {
            if looks_like_hostname(host) {
                return Ok(Self::host(host, Some(port)));
            }
        }
        if looks_like_hostname(spec) {
            return Ok(Self::host(spec, None));
        }
        Err(Error::BadGrant(format!(
            "network grant `{spec}` must be an IP, ip:port, hostname, or hostname:port"
        )))
    }
}

fn looks_like_hostname(name: &str) -> bool {
    if name.is_empty() || name.len() > 253 {
        return false;
    }
    name.bytes().all(|b| {
        b.is_ascii_alphanumeric() || b == b'-' || b == b'.' || b == b'_'
    }) && name.contains(|c: char| c.is_ascii_alphabetic())
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
        for net in &self.net {
            if let NetTarget::Host(name) = &net.target
                && resolve_host(name).is_err()
            {
                warnings.push(format!(
                    "hostname grant `{name}` did not resolve at validation time; connect-time lookup may still succeed"
                ));
            }
        }
        Ok(warnings)
    }

    pub(crate) fn needs_name_lookup(&self) -> bool {
        self.net.iter().any(NetGrant::is_hostname)
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

        let host = NetGrant::parse("localhost:9").unwrap();
        assert!(matches!(host.target, NetTarget::Host(_)));
        assert_eq!(host.port, Some(9));

        let env = EnvGrant::parse("BOXWREN_TEST_VALUE=open").unwrap();
        assert_eq!(env.name, "BOXWREN_TEST_VALUE");
        assert_eq!(env.value, "open");
    }
}
