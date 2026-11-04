use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;

use crate::storage;

/// A single hold/drag event stat.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HoldStat {
    pub duration_ms: f64,
    pub is_drag: bool,
}

/// Filter and monitor for mouse hold / drag operations.
///
/// Detects microswitch chatter during prolonged holds (e.g. dragging a window,
/// selecting text, or holding fire in games) where switch contact vibration
/// momentarily breaks contact (< 50ms) causing accidental drop/release.
#[derive(Serialize, Deserialize)]
pub struct HoldClickFilter {
    buffer: VecDeque<HoldStat>,
    capacity: usize,
    pub total_holds: u32,
    pub total_drops_prevented: u32,
    #[serde(default)]
    pub min_hold_duration: Option<f64>,
    #[serde(skip)]
    pub last_hold_duration: Option<f64>,
    #[serde(default = "default_drag_threshold")]
    pub drag_threshold_ms: f64,
}

fn default_drag_threshold() -> f64 {
    150.0 // Holds longer than 150ms are considered drag/hold operations
}

impl HoldClickFilter {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            total_holds: 0,
            total_drops_prevented: 0,
            min_hold_duration: None,
            last_hold_duration: None,
            drag_threshold_ms: default_drag_threshold(),
        }
    }

    // ── Persistence ──────────────────────────────────────────────

    pub fn load() -> Option<Self> {
        let path = storage::hold_click_path();
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn save(&self) {
        let path = storage::hold_click_path();
        if let Ok(json) = serde_json::to_string(self) {
            let _ = fs::write(path, json);
        }
    }

    // ── Mutation ─────────────────────────────────────────────────

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.total_holds = 0;
        self.total_drops_prevented = 0;
        self.min_hold_duration = None;
        self.last_hold_duration = None;
    }

    pub fn record_hold(&mut self, duration_ms: f64) {
        self.last_hold_duration = Some(duration_ms);
        self.total_holds += 1;

        let is_drag = duration_ms >= self.drag_threshold_ms;

        if is_drag {
            match self.min_hold_duration {
                Some(min) if duration_ms < min => self.min_hold_duration = Some(duration_ms),
                None => self.min_hold_duration = Some(duration_ms),
                _ => {}
            }
        }

        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(HoldStat {
            duration_ms,
            is_drag,
        });
    }

    /// Evaluates if an unintentional release occurred during an ongoing hold.
    ///
    /// If the button was released for a micro-interval (< 50ms) and distance is minimal,
    /// it indicates hardware switch chatter breaking contact during a drag.
    pub fn is_drag_bounce(&self, release_interval_ms: f64, distance_px: f64) -> bool {
        distance_px <= 2.0 && release_interval_ms < 50.0
    }

    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
