use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use prometheus::Gauge;
use std::fs;
use std::thread;
use std::time::Duration;

// Parse total jiffies from /proc/stat (first "cpu" line)
fn read_total_jiffies() -> Option<u64> {
    let s = fs::read_to_string("/proc/stat").ok()?;
    for line in s.lines() {
        if line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // parts[0] == "cpu"
            let mut sum: u64 = 0;
            for v in parts.iter().skip(1) {
                if let Ok(n) = v.parse::<u64>() {
                    sum = sum.saturating_add(n);
                }
            }
            return Some(sum);
        }
    }
    None
}

// Parse process jiffies (utime + stime) from /proc/self/stat
fn read_proc_jiffies() -> Option<u64> {
    let s = fs::read_to_string("/proc/self/stat").ok()?;
    // stat fields: see proc manpage. utime is field 14, stime 15 (1-based)
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() > 15 {
        let utime = parts[13].parse::<u64>().ok()?;
        let stime = parts[14].parse::<u64>().ok()?;
        return Some(utime.saturating_add(stime));
    }
    None
}

// Read resident set size (VmRSS) in bytes from /proc/self/status
fn read_proc_rss_bytes() -> Option<u64> {
    let s = fs::read_to_string("/proc/self/status").ok()?;
    for line in s.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // VmRSS: <value> kB
            if parts.len() >= 2 {
                if let Ok(kb) = parts[1].parse::<u64>() {
                    return Some(kb * 1024);
                }
            }
        }
    }
    None
}

// Collect CPU and memory usage for the current process only (Linux /proc implementation).
pub fn build_prom() -> PrometheusMetrics {
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    let cpu_usage = Gauge::new(
        "process_cpu_usage_percent",
        "Current CPU usage of this process in percent",
    )
    .unwrap();
    let mem_usage = Gauge::new(
        "process_memory_bytes",
        "Resident memory used by this process in bytes",
    )
    .unwrap();

    prometheus
        .registry
        .register(Box::new(cpu_usage.clone()))
        .unwrap();

    prometheus
        .registry
        .register(Box::new(mem_usage.clone()))
        .unwrap();

    thread::spawn(move || {
        // initial values
        let mut prev_total = read_total_jiffies().unwrap_or(0);
        let mut prev_proc = read_proc_jiffies().unwrap_or(0);

        loop {
            thread::sleep(Duration::from_secs(1));

            let total = match read_total_jiffies() {
                Some(v) => v,
                None => continue,
            };
            let proc = match read_proc_jiffies() {
                Some(v) => v,
                None => continue,
            };

            let delta_total = total.saturating_sub(prev_total);
            let delta_proc = proc.saturating_sub(prev_proc);

            prev_total = total;
            prev_proc = proc;

            if delta_total > 0 {
                // Percentage = proc_delta / total_delta * 100
                let percent = (delta_proc as f64 / delta_total as f64) * 100.0;
                cpu_usage.set(percent);
            }

            if let Some(rss_bytes) = read_proc_rss_bytes() {
                mem_usage.set(rss_bytes as f64);
            }
        }
    });

    prometheus
}
