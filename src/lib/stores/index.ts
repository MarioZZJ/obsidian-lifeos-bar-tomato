import { writable, derived, get } from 'svelte/store';
import type { TimerStatus, PomodoroConfig, Project, VaultTask, TodayStats } from '../types';
import * as api from '../api/tauri';
import { listen } from '@tauri-apps/api/event';

// Timer status store
export const timerStatus = writable<TimerStatus>({
  phase: 'idle',
  mode: 'pomodoro',
  elapsedSecs: 0,
  remainingSecs: null,
  overtimeSecs: 0,
  pomodoroCount: 0,
  currentTask: null,
  currentProject: null,
  currentProjectPath: null,
});

// Config store
export const config = writable<PomodoroConfig>({
  pomodoroDuration: 25,
  shortBreakDuration: 5,
  longBreakDuration: 15,
  longBreakInterval: 4,
  autoStartBreak: false,
  pomodoroSound: true,
});

// Vault path store
export const vaultPath = writable<string | null>(null);

// Projects store
export const projects = writable<Project[]>([]);

// Tasks store
export const tasks = writable<VaultTask[]>([]);

// Today stats store
export const todayStats = writable<TodayStats>({
  totalMinutes: 0,
  pomodoroCount: 0,
});

// Autostart store
export const autostart = writable<boolean>(false);

// Derived: formatted remaining time
export const formattedTime = derived(timerStatus, ($status) => {
  // Show overtime as +MM:SS
  if ($status.overtimeSecs > 0) {
    const mins = Math.floor($status.overtimeSecs / 60);
    const secs = $status.overtimeSecs % 60;
    return `+${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  if ($status.remainingSecs !== null) {
    const mins = Math.floor($status.remainingSecs / 60);
    const secs = $status.remainingSecs % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  const mins = Math.floor($status.elapsedSecs / 60);
  const secs = $status.elapsedSecs % 60;
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
});

// Derived: progress (0-1)
export const progress = derived([timerStatus, config], ([$status, $config]) => {
  if ($status.phase === 'idle') return 0;
  if ($status.mode === 'stopwatch') return 0; // No progress for stopwatch
  if ($status.overtimeSecs > 0) return 1; // Full circle during overtime

  const totalSecs =
    $status.phase === 'shortBreak'
      ? $config.shortBreakDuration * 60
      : $status.phase === 'longBreak'
        ? $config.longBreakDuration * 60
        : $config.pomodoroDuration * 60;

  if ($status.remainingSecs === null) return 0;
  return 1 - $status.remainingSecs / totalSecs;
});

// Initialize stores
export async function initializeStores() {
  try {
    // Check if vault is configured
    const path = await api.getVaultPath();
    vaultPath.set(path);

    if (path) {
      const cfg = await api.getConfig();
      config.set(cfg);

      const projs = await api.scanProjects();
      projects.set(projs);

      const tasksData = await api.scanTasks();
      tasks.set(tasksData);

      const stats = await api.getTodayStats();
      todayStats.set(stats);
    }

    // Get initial timer status
    const status = await api.getTimerStatus();
    timerStatus.set(status);

    // Get autostart state
    const autostartState = await api.getAutostart();
    autostart.set(autostartState);
  } catch (e) {
    console.error('Failed to initialize stores:', e);
  }
}

// Refresh timer status
export async function refreshTimerStatus() {
  try {
    const status = await api.getTimerStatus();
    timerStatus.set(status);
  } catch (e) {
    console.error('Failed to refresh timer status:', e);
  }
}

// Refresh today stats
export async function refreshTodayStats() {
  try {
    const stats = await api.getTodayStats();
    todayStats.set(stats);
  } catch (e) {
    console.error('Failed to refresh today stats:', e);
  }
}

// Set up event listeners
export function setupEventListeners() {
  listen('timer-tick', () => {
    refreshTimerStatus();
  });

  listen('pomodoro-complete', () => {
    refreshTimerStatus();
    refreshTodayStats();
  });

  listen('break-complete', () => {
    refreshTimerStatus();
  });
}
