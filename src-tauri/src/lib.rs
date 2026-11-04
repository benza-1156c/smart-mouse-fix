use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::Instant;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, HC_ACTION, MSG, MSLLHOOKSTRUCT, SetWindowsHookExW,
    UnhookWindowsHookEx, WH_MOUSE_LL, WM_LBUTTONDOWN,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClickStat {
    pub interval_ms: f64,
    pub distance_px: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SmartMouseFilter {
    buffer: VecDeque<ClickStat>,
    capacity: usize,
    pub total_clicks: u32,
    pub total_blocked: u32,
    #[serde(default)]
    pub min_interval: Option<f64>,
    #[serde(skip)]
    pub last_interval: Option<f64>,
}

impl SmartMouseFilter {
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

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.total_clicks = 0;
        self.total_blocked = 0;
        self.min_interval = None;
        self.last_interval = None;
    }

    pub fn update_intervals(&mut self, interval: f64) {
        self.last_interval = Some(interval);
        if let Some(min) = self.min_interval {
            if interval < min {
                self.min_interval = Some(interval);
            }
        } else {
            self.min_interval = Some(interval);
        }
    }

    pub fn load() -> Option<Self> {
        let path = get_data_path();
        if let Ok(data) = fs::read_to_string(path) {
            serde_json::from_str(&data).ok()
        } else {
            None
        }
    }

    pub fn save(&self) {
        let path = get_data_path();
        if let Ok(json) = serde_json::to_string(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn add_stat(&mut self, stat: ClickStat) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(stat);
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

    pub fn is_hardware_bounce(&self, new_interval: f64, new_distance: f64) -> bool {
        if new_distance > 1.0 {
            return false;
        }

        if self.buffer.len() < 10 {
            return new_interval < 85.0;
        }

        let mean = self.mean_interval();
        let sd = self.std_dev_interval(mean).max(1.0);

        let dynamic_limit = mean - (2.5 * sd);

        new_interval < dynamic_limit
    }
}

static FILTER: LazyLock<Mutex<SmartMouseFilter>> = LazyLock::new(|| {
    Mutex::new(SmartMouseFilter::load().unwrap_or_else(|| SmartMouseFilter::new(50)))
});

static LAST_CLICK: LazyLock<Mutex<Option<(Instant, (i32, i32))>>> =
    LazyLock::new(|| Mutex::new(None));

unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let msg = wparam.0 as u32;

        if msg == WM_LBUTTONDOWN {
            let hook_struct = unsafe { *(lparam.0 as *const MSLLHOOKSTRUCT) };
            let current_time = Instant::now();
            let current_pos = (hook_struct.pt.x, hook_struct.pt.y);

            let mut last_click_lock = LAST_CLICK.lock().unwrap();
            let mut filter_lock = FILTER.lock().unwrap();

            if let Some((last_time, last_pos)) = *last_click_lock {
                let interval = current_time.duration_since(last_time).as_millis() as f64;

                let dx = (current_pos.0 - last_pos.0) as f64;
                let dy = (current_pos.1 - last_pos.1) as f64;
                let distance = (dx * dx + dy * dy).sqrt();

                filter_lock.update_intervals(interval);

                if filter_lock.is_hardware_bounce(interval, distance) {
                    filter_lock.total_blocked += 1;
                    filter_lock.save();
                    return LRESULT(1);
                } else {
                    filter_lock.total_clicks += 1;
                    filter_lock.add_stat(ClickStat {
                        interval_ms: interval,
                        distance_px: distance,
                    });
                    filter_lock.save();
                }
            }

            *last_click_lock = Some((current_time, current_pos));
        }
    }

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

fn start_mouse_hook() {
    thread::spawn(|| unsafe {
        let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), None, 0)
            .expect("Failed to install mouse hook");

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {}

        UnhookWindowsHookEx(hook).unwrap();
    });
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
fn get_mouse_stats() -> FilterStatus {
    let filter = FILTER.lock().unwrap();
    let mean = filter.mean_interval();
    let sd = filter.std_dev_interval(mean).max(1.0);
    let count = filter.buffer.len();

    FilterStatus {
        total_clicks: filter.total_clicks,
        total_blocked: filter.total_blocked,
        buffer_count: count,
        capacity: filter.capacity,
        is_warmed_up: count >= 50,
        is_adaptive: count >= 10,
        current_limit: if count >= 10 {
            (mean - (2.5 * sd)).max(5.0)
        } else {
            85.0
        },
        min_interval: filter.min_interval,
        last_interval: filter.last_interval,
    }
}

#[tauri::command]
fn reset_mouse_stats() -> FilterStatus {
    let mut filter = FILTER.lock().unwrap();
    let mut last_click = LAST_CLICK.lock().unwrap();
    *last_click = None;
    filter.reset();
    filter.save();
    drop(filter);
    get_mouse_stats()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    start_mouse_hook();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_mouse_stats,
            reset_mouse_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_data_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "benza", "SmartMouseFix") {
        let dir = proj_dirs.data_dir();

        let _ = fs::create_dir_all(dir);

        dir.join("mouse_data.json")
    } else {
        PathBuf::from("mouse_data.json")
    }
}
