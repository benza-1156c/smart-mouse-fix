use serde::Serialize;
use std::sync::atomic::Ordering;

use crate::hook::{
    BUTTON_DOWN_STATE, FILTER, HOLD_FILTER, LAST_CLICK, LAST_UP, SWALLOW_NEXT_UP,
};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Snapshot of the left-click debounce filter state.
#[derive(Serialize)]
pub struct FilterStatus {
    pub total_clicks: u32,
    pub total_blocked: u32,
    pub buffer_count: usize,
    pub capacity: usize,
    pub is_warmed_up: bool,
    pub is_adaptive: bool,
    pub current_limit: f64,
    pub min_interval: Option<f64>,
    pub last_interval: Option<f64>,
}

#[tauri::command]
pub fn get_mouse_stats() -> FilterStatus {
    let filter = FILTER.lock().unwrap();
    let count = filter.buffer_len();

    FilterStatus {
        total_clicks: filter.total_clicks,
        total_blocked: filter.total_blocked,
        buffer_count: count,
        capacity: filter.capacity(),
        is_warmed_up: count >= 50,
        is_adaptive: count >= 10,
        current_limit: filter.current_limit(),
        min_interval: filter.min_interval,
        last_interval: filter.last_interval,
    }
}

#[tauri::command]
pub fn reset_mouse_stats() -> FilterStatus {
    {
        let mut filter = FILTER.lock().unwrap();
        let mut last_click = LAST_CLICK.lock().unwrap();
        let mut last_up = LAST_UP.lock().unwrap();
        let mut down_state = BUTTON_DOWN_STATE.lock().unwrap();
        *last_click = None;
        *last_up = None;
        *down_state = None;
        SWALLOW_NEXT_UP.store(false, Ordering::SeqCst);
        filter.reset();
        filter.save();
    }
    get_mouse_stats()
}

/// Snapshot of the hold-click (drag protection) filter state.
#[derive(Serialize)]
pub struct HoldFilterStatus {
    pub total_holds: u32,
    pub total_drops_prevented: u32,
    pub buffer_count: usize,
    pub capacity: usize,
    pub min_hold_duration: Option<f64>,
    pub last_hold_duration: Option<f64>,
}

#[tauri::command]
pub fn get_hold_stats() -> HoldFilterStatus {
    let filter = HOLD_FILTER.lock().unwrap();

    HoldFilterStatus {
        total_holds: filter.total_holds,
        total_drops_prevented: filter.total_drops_prevented,
        buffer_count: filter.buffer_len(),
        capacity: filter.capacity(),
        min_hold_duration: filter.min_hold_duration,
        last_hold_duration: filter.last_hold_duration,
    }
}

#[tauri::command]
pub fn reset_hold_stats() -> HoldFilterStatus {
    {
        let mut filter = HOLD_FILTER.lock().unwrap();
        let mut last_up = LAST_UP.lock().unwrap();
        let mut down_state = BUTTON_DOWN_STATE.lock().unwrap();
        *last_up = None;
        *down_state = None;
        filter.reset();
        filter.save();
    }
    get_hold_stats()
}
