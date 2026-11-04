import { createSignal, onMount, onCleanup } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import {
  enable as enableAutostart,
  disable as disableAutostart,
  isEnabled as isAutostartEnabled,
} from "@tauri-apps/plugin-autostart";
import { Header } from "./components/Header";
import { LearningProgress } from "./components/LearningProgress";
import { ClickTarget } from "./components/ClickTarget";
import { LiveStatistics } from "./components/LiveStatistics";
import { BlockedToast } from "./components/BlockedToast";
import { FilterStatus, Ripple } from "./types";
import "./App.css";

const DEFAULT_STATS: FilterStatus = {
  total_clicks: 0,
  total_blocked: 0,
  buffer_count: 0,
  capacity: 50,
  is_warmed_up: false,
  is_adaptive: false,
  current_limit: 85.0,
  min_interval: null,
  last_interval: null,
};

function App() {
  const [stats, setStats] = createSignal<FilterStatus>(DEFAULT_STATS);
  const [ripples, setRipples] = createSignal<Ripple[]>([]);
  const [isClickActive, setIsClickActive] = createSignal(false);
  const [isBlockedFlash, setIsBlockedFlash] = createSignal(false);
  const [blockedAlertMsg, setBlockedAlertMsg] = createSignal<string | null>(null);
  const [autostartEnabled, setAutostartEnabled] = createSignal(false);

  let lastBlockedCount = 0;
  let alertTimeout: number | undefined;

  async function checkAutostart() {
    try {
      const active = await isAutostartEnabled();
      setAutostartEnabled(active);
    } catch {
      // Running outside Tauri or in preview
    }
  }

  async function handleToggleAutostart() {
    try {
      if (autostartEnabled()) {
        await disableAutostart();
        setAutostartEnabled(false);
      } else {
        await enableAutostart();
        setAutostartEnabled(true);
      }
    } catch (err) {
      console.error("Autostart toggle failed:", err);
    }
  }

  async function fetchStats() {
    try {
      const data = await invoke<FilterStatus>("get_mouse_stats");

      if (data.total_blocked > lastBlockedCount) {
        const delta = data.total_blocked - lastBlockedCount;
        setIsBlockedFlash(true);
        setBlockedAlertMsg(
          `ตรวจพบการลั่น (+${delta}) — ระบบสกัดกั้นสัญญาณเรียบร้อย`,
        );

        if (alertTimeout) clearTimeout(alertTimeout);
        alertTimeout = window.setTimeout(() => {
          setIsBlockedFlash(false);
          setBlockedAlertMsg(null);
        }, 2200);
      }

      lastBlockedCount = data.total_blocked;
      setStats(data);
    } catch {
      // In browser preview mode without Tauri IPC
    }
  }

  function handleTargetClick(e: MouseEvent) {
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const size = Math.max(rect.width, rect.height);

    const newRipple: Ripple = {
      id: Date.now() + Math.random(),
      x: x - size / 2,
      y: y - size / 2,
      size,
    };

    setRipples((prev) => [...prev.slice(-3), newRipple]);
    setTimeout(() => {
      setRipples((prev) => prev.filter((r) => r.id !== newRipple.id));
    }, 500);

    setIsClickActive(true);
    setTimeout(() => setIsClickActive(false), 90);

    fetchStats();
  }

  async function handleReset() {
    try {
      const data = await invoke<FilterStatus>("reset_mouse_stats");
      lastBlockedCount = 0;
      setStats(data);
      setBlockedAlertMsg(null);
    } catch {
      setStats(DEFAULT_STATS);
      lastBlockedCount = 0;
    }
  }

  onMount(() => {
    checkAutostart();
    fetchStats();
    const timer = setInterval(fetchStats, 100);
    onCleanup(() => {
      clearInterval(timer);
      if (alertTimeout) clearTimeout(alertTimeout);
    });
  });

  const targetClicks = () => stats().capacity || 50;
  const currentClicks = () => Math.min(stats().total_clicks, targetClicks());
  const progressPercent = () =>
    Math.min(100, Math.round((currentClicks() / targetClicks()) * 100));
  const isComplete = () => stats().total_clicks >= targetClicks();

  return (
    <div class="mx-auto flex min-h-screen max-w-3xl flex-col gap-4 p-4 antialiased">
      <Header
        isComplete={isComplete()}
        isAdaptive={stats().is_adaptive}
        autostartEnabled={autostartEnabled()}
        onToggleAutostart={handleToggleAutostart}
        onReset={handleReset}
      />

      <BlockedToast message={blockedAlertMsg()} />

      <LearningProgress
        currentClicks={currentClicks()}
        targetClicks={targetClicks()}
        progressPercent={progressPercent()}
        isComplete={isComplete()}
      />

      <ClickTarget
        isComplete={isComplete()}
        isClickActive={isClickActive()}
        isBlockedFlash={isBlockedFlash()}
        totalClicks={stats().total_clicks}
        lastInterval={stats().last_interval}
        ripples={ripples()}
        onMouseDown={handleTargetClick}
      />

      <LiveStatistics stats={stats()} />
    </div>
  );
}

export default App;
