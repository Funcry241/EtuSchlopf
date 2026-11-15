// This estimate schedules processes. It grants no musical or validation authority.
const CORPUS_MIB: u64 = 1024 * 1024;
const CORPUS_MEMORY_RESERVE: u64 = 512 * CORPUS_MIB;
const CORPUS_PROJECT_MEMORY: u64 = 1536 * CORPUS_MIB;
const CORPUS_SOLVER_WORKER_MEMORY: u64 = 256 * CORPUS_MIB;

#[derive(Default)]
struct CorpusMemory {
    physical: Option<u64>,
    available: Option<u64>,
    limit_headroom: Option<u64>,
    effective: Option<u64>,
}

impl CorpusMemory {
    fn plan(&self, requested_workers: usize, max_projects: usize) -> (usize, usize) {
        let Some(effective) = self.effective else {
            return (1, 1);
        };
        let usable = effective.saturating_sub(CORPUS_MEMORY_RESERVE);
        for projects in (1..=max_projects).rev() {
            let per_project = usable / projects as u64;
            let workers = (per_project.saturating_sub(CORPUS_PROJECT_MEMORY)
                / CORPUS_SOLVER_WORKER_MEMORY) as usize;
            if workers > 0 {
                return (projects, workers.min(requested_workers / projects));
            }
        }
        (1, 1)
    }

    fn report(&self) -> String {
        fn mib(bytes: Option<u64>) -> String {
            bytes.map_or_else(
                || "unknown".into(),
                |value| (value / CORPUS_MIB).to_string(),
            )
        }
        format!(
            "KorpusMemory: PhysicalMiB {} | AvailableMiB {} | LimitHeadroomMiB {} | EffectiveMiB {}",
            mib(self.physical),
            mib(self.available),
            mib(self.limit_headroom),
            mib(self.effective),
        )
    }
}

#[cfg(target_os = "linux")]
fn detect_corpus_memory() -> CorpusMemory {
    fn kib(text: &str, key: &str) -> Option<u64> {
        let line = text.lines().find_map(|line| line.strip_prefix(key))?;
        line.split_whitespace()
            .next()?
            .parse::<u64>()
            .ok()?
            .checked_mul(1024)
    }

    let host = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut memory = CorpusMemory {
        physical: kib(&host, "MemTotal:"),
        available: kib(&host, "MemAvailable:"),
        ..CorpusMemory::default()
    };
    if let Some(headroom) = linux_cgroup_headroom() {
        memory.limit_headroom = headroom;
        memory.effective = match (memory.available, headroom) {
            (Some(host), Some(limit)) => Some(host.min(limit)),
            (host, None) => host,
            (None, limit) => limit,
        };
    }
    memory
}

// Outer None means the effective constraint could not be read. Inner None
// means the visible cgroup v2 hierarchy has no finite limit.
#[cfg(target_os = "linux")]
fn linux_cgroup_headroom() -> Option<Option<u64>> {
    let memberships = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    let mounts = std::fs::read_to_string("/proc/self/mountinfo").ok()?;
    let (mount, relative) = linux_cgroup_path(&memberships, &mounts)?;
    cgroup_memory_headroom(mount, relative, |path| std::fs::read_to_string(path))
}

#[cfg(target_os = "linux")]
fn linux_cgroup_path<'a>(
    memberships: &'a str,
    mounts: &'a str,
) -> Option<(&'a std::path::Path, &'a std::path::Path)> {
    let group = memberships
        .lines()
        .find_map(|line| line.strip_prefix("0::"))?;
    let group = std::path::Path::new(group);
    if !group.is_absolute()
        || group
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return None;
    }
    mounts.lines().find_map(|line| {
        let (details, filesystem) = line.split_once(" - ")?;
        if filesystem.split_whitespace().next()? != "cgroup2" {
            return None;
        }
        let mut fields = details.split_whitespace().skip(3);
        let root = std::path::Path::new(fields.next()?);
        let mount = std::path::Path::new(fields.next()?);
        Some((mount, group.strip_prefix(root).ok()?))
    })
}

#[cfg(target_os = "linux")]
fn cgroup_memory_headroom(
    mount: &std::path::Path,
    relative: &std::path::Path,
    read: impl Fn(&std::path::Path) -> std::io::Result<String>,
) -> Option<Option<u64>> {
    let mut path = mount.join(relative);
    let mut headroom: Option<u64> = None;
    loop {
        match read(&path.join("memory.max")) {
            Ok(limit) if limit.trim() == "max" => {}
            Ok(limit) => {
                let limit = limit.trim().parse::<u64>().ok()?;
                let used = read(&path.join("memory.current")).ok()?;
                let remaining = limit.saturating_sub(used.trim().parse::<u64>().ok()?);
                headroom = Some(headroom.map_or(remaining, |other| other.min(remaining)));
            }
            Err(error)
                if path == mount
                    && error.kind() == std::io::ErrorKind::NotFound
                    && read(&path.join("cgroup.controllers")).is_ok() => {}
            Err(_) => return None,
        }
        if path == mount {
            break;
        }
        path.pop();
    }
    Some(headroom)
}

#[cfg(target_os = "windows")]
fn detect_corpus_memory() -> CorpusMemory {
    let Some(system_root) = std::env::var_os("SystemRoot") else {
        return CorpusMemory::default();
    };
    let powershell = std::path::PathBuf::from(system_root)
        .join("System32/WindowsPowerShell/v1.0/powershell.exe");
    let Ok(output) = std::process::Command::new(powershell)
        .args([
            "-NoProfile", "-NonInteractive", "-Command",
            "$os=Get-CimInstance Win32_OperatingSystem -OperationTimeoutSec 3 -ErrorAction Stop; [Console]::WriteLine($os.TotalVisibleMemorySize); [Console]::WriteLine($os.FreePhysicalMemory)",
        ])
        .output()
    else {
        return CorpusMemory::default();
    };
    if !output.status.success() {
        return CorpusMemory::default();
    }
    let mut lines = std::str::from_utf8(&output.stdout)
        .unwrap_or_default()
        .lines();
    let mut next_bytes = || lines.next()?.trim().parse::<u64>().ok()?.checked_mul(1024);
    let physical = next_bytes();
    let available = next_bytes();
    CorpusMemory {
        physical,
        available,
        effective: available,
        ..CorpusMemory::default()
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn detect_corpus_memory() -> CorpusMemory {
    CorpusMemory::default()
}

#[cfg(test)]
mod corpus_memory_capacity_tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn cgroup_paths_bind_membership_to_its_mount_and_use_tightest_ancestor() {
        let mounts = "17 1 0:25 /base /sys/fs/cgroup rw - cgroup2 cgroup rw";
        assert_eq!(
            linux_cgroup_path("0::/base/parent/leaf", mounts),
            Some((
                std::path::Path::new("/sys/fs/cgroup"),
                std::path::Path::new("parent/leaf")
            )),
        );
        assert!(linux_cgroup_path("0::/other/leaf", mounts).is_none());
        let path = std::path::Path::new("/synthetic");
        let files = [
            ("cgroup.controllers", "memory"),
            ("parent/memory.max", "1000"),
            ("parent/memory.current", "800"),
            ("parent/leaf/memory.max", "500"),
            ("parent/leaf/memory.current", "200"),
        ];
        let measured = cgroup_memory_headroom(path, std::path::Path::new("parent/leaf"), |file| {
            files
                .iter()
                .find_map(|(name, value)| (path.join(name) == file).then(|| value.to_string()))
                .ok_or_else(|| std::io::ErrorKind::NotFound.into())
        });
        assert_eq!(measured, Some(Some(200)));
    }

    #[test]
    fn available_memory_caps_processes_and_solver_workers() {
        let plan = |mib| {
            CorpusMemory {
                effective: Some(mib * CORPUS_MIB),
                ..CorpusMemory::default()
            }
            .plan(19, 3)
        };
        assert_eq!(plan(2048), (1, 1));
        assert_eq!(plan(3072), (1, 4));
        assert_eq!(plan(4608), (2, 2));
        assert_eq!(plan(16384), (3, 6));
        assert_eq!(CorpusMemory::default().plan(19, 3), (1, 1));
    }
}
