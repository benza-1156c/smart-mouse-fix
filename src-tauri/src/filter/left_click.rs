use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;

use crate::storage;

/// A single recorded click sample.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClickStat {
    pub interval_ms: f64,
    pub distance_px: f64,
}

/// Ring-buffer based adaptive filter for left-click debounce.
///
/// Collects click interval samples into a fixed-capacity ring buffer and
/// uses mean − 2.5×SD as a dynamic threshold to distinguish hardware
/// bounce (chatter) from intentional rapid clicks.
#[derive(Serialize, Deserialize)]
pub struct LeftClickFilter {
    buffer: VecDeque<ClickStat>,
    capacity: usize,
    pub total_clicks: u32,
    pub total_blocked: u32,
    #[serde(default)]
    pub min_interval: Option<f64>,
    #[serde(skip)]
    pub last_interval: Option<f64>,
}

impl LeftClickFilter {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            total_clicks: 0,
            total_blocked: 0,
            min_interval: None,
            last_interval: None,
        }
    }

    // ── Persistence ──────────────────────────────────────────────

    pub fn load() -> Option<Self> {
        let path = storage::left_click_path();
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn save(&self) {
        let path = storage::left_click_path();
        if let Ok(json) = serde_json::to_string(self) {
            let _ = fs::write(path, json);
        }
    }

    // ── Mutation ─────────────────────────────────────────────────

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.total_clicks = 0;
        self.total_blocked = 0;
        self.min_interval = None;
        self.last_interval = None;
    }

    pub fn record_interval(&mut self, interval: f64) {
        self.last_interval = Some(interval);
        match self.min_interval {
            Some(min) if interval < min => self.min_interval = Some(interval),
            None => self.min_interval = Some(interval),
            _ => {}
        }
    }

    pub fn push_stat(&mut self, stat: ClickStat) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(stat);
    }

    // ── Statistics ───────────────────────────────────────────────

    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn mean_interval(&self) -> f64 {
        if self.buffer.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.buffer.iter().map(|s| s.interval_ms).sum();
        sum / self.buffer.len() as f64
    }

    pub fn std_dev_interval(&self, mean: f64) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let variance: f64 = self
            .buffer
            .iter()
            .map(|s| {
                let d = s.interval_ms - mean;
                d * d
            })
            .sum::<f64>()
            / (self.buffer.len() - 1) as f64;
        variance.sqrt()
    }

    /// The dynamic threshold below which a click is considered hardware bounce.
    /// Returns the fixed baseline (85 ms) until enough samples are collected.
    pub fn current_limit(&self) -> f64 {
        if self.buffer.len() < 10 {
            return 85.0;
        }
        let mean = self.mean_interval();
        let sd = self.std_dev_interval(mean).max(1.0);
        (mean - 2.5 * sd).max(5.0)
    }

    // ── Decision ─────────────────────────────────────────────────

    /// Returns `true` when `new_interval` looks like a hardware switch
    /// bounce rather than an intentional click.
    ///
    /// A click that moved more than 1 px is never considered a bounce
    /// (the user physically moved the cursor between clicks).
    pub fn is_hardware_bounce(&self, new_interval: f64, new_distance: f64) -> bool {
        if new_distance > 1.0 {
            return false;
        }
        new_interval < self.current_limit()
    }
}
