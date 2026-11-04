use std::sync::{LazyLock, Mutex};
use std::time::Instant;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, HC_ACTION, MSG, MSLLHOOKSTRUCT, SetWindowsHookExW,
    UnhookWindowsHookEx, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP,
};

use crate::filter::{ClickStat, HoldClickFilter, LeftClickFilter};

/// Global left-click filter (loaded from disk or fresh with capacity 50).
pub static FILTER: LazyLock<Mutex<LeftClickFilter>> = LazyLock::new(|| {
    Mutex::new(LeftClickFilter::load().unwrap_or_else(|| LeftClickFilter::new(50)))
});

/// Global hold-click filter for drag / hold monitoring and drop prevention.
pub static HOLD_FILTER: LazyLock<Mutex<HoldClickFilter>> = LazyLock::new(|| {
    Mutex::new(HoldClickFilter::load().unwrap_or_else(|| HoldClickFilter::new(50)))
});

/// Tracks the timestamp + cursor position of the previous left-click down.
pub static LAST_CLICK: LazyLock<Mutex<Option<(Instant, (i32, i32))>>> =
    LazyLock::new(|| Mutex::new(None));

/// Tracks when the button was pressed down to calculate hold duration on release.
pub static BUTTON_DOWN_STATE: LazyLock<Mutex<Option<(Instant, (i32, i32))>>> =
    LazyLock::new(|| Mutex::new(None));

/// Tracks the previous button-up event: (timestamp, position, previous hold duration in ms).
pub static LAST_UP: LazyLock<Mutex<Option<(Instant, (i32, i32), f64)>>> =
    LazyLock::new(|| Mutex::new(None));

unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let msg = wparam.0 as u32;

        // ── 1. Left-click down ───────────────────────────────────
        if msg == WM_LBUTTONDOWN {
            let hook_struct = unsafe { *(lparam.0 as *const MSLLHOOKSTRUCT) };
            let now = Instant::now();
            let pos = (hook_struct.pt.x, hook_struct.pt.y);

            // Check for hold/drag drop bounce:
            // If the user was holding/dragging (> 150ms) and the switch chattered open
            // and closed within 50ms, the contact bounced during a drag.
            {
                let last_up = LAST_UP.lock().unwrap();
                let mut hold_filter = HOLD_FILTER.lock().unwrap();

                if let Some((up_time, up_pos, hold_duration)) = *last_up {
                    let release_interval = now.duration_since(up_time).as_millis() as f64;
                    let dx = (pos.0 - up_pos.0) as f64;
                    let dy = (pos.1 - up_pos.1) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if hold_duration >= 150.0 && hold_filter.is_drag_bounce(release_interval, dist)
                    {
                        hold_filter.total_drops_prevented += 1;
                        hold_filter.save();
                    }
                }
            }

            // Normal left-click debounce logic:
            let mut last = LAST_CLICK.lock().unwrap();
            let mut filter = FILTER.lock().unwrap();

            if let Some((prev_time, prev_pos)) = *last {
                let interval = now.duration_since(prev_time).as_millis() as f64;
                let dx = (pos.0 - prev_pos.0) as f64;
                let dy = (pos.1 - prev_pos.1) as f64;
                let distance = (dx * dx + dy * dy).sqrt();

                filter.record_interval(interval);

                if filter.is_hardware_bounce(interval, distance) {
                    filter.total_blocked += 1;
                    filter.save();
                    return LRESULT(1); // swallow the bounced click
                }

                filter.total_clicks += 1;
                filter.push_stat(ClickStat {
                    interval_ms: interval,
                    distance_px: distance,
                });
                filter.save();
            }

            *last = Some((now, pos));
            *BUTTON_DOWN_STATE.lock().unwrap() = Some((now, pos));
        }

        // ── 2. Left-click up (Hold-click release) ─────────────────
        if msg == WM_LBUTTONUP {
            let hook_struct = unsafe { *(lparam.0 as *const MSLLHOOKSTRUCT) };
            let now = Instant::now();
            let pos = (hook_struct.pt.x, hook_struct.pt.y);

            let mut down_state = BUTTON_DOWN_STATE.lock().unwrap();
            if let Some((down_time, _)) = down_state.take() {
                let hold_duration = now.duration_since(down_time).as_millis() as f64;

                let mut hold_filter = HOLD_FILTER.lock().unwrap();
                hold_filter.record_hold(hold_duration);
                hold_filter.save();

                *LAST_UP.lock().unwrap() = Some((now, pos, hold_duration));
            }
        }
    }

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

pub fn start() {
    thread_spawn();
}

fn thread_spawn() {
    std::thread::spawn(|| unsafe {
        let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), None, 0)
            .expect("Failed to install mouse hook");

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {}

        UnhookWindowsHookEx(hook).unwrap();
    });
}
