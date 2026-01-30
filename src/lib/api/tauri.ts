import { invoke } from '@tauri-apps/api/core';
import type { TimerStatus, PomodoroConfig, Project, VaultTask, TodayStats } from '../types';

export async function getTimerStatus(): Promise<TimerStatus> {
  return invoke('get_timer_status');
}

export async function startPomodoro(
  task?: string,
  project?: string,
  projectPath?: string
): Promise<void> {
  return invoke('start_pomodoro', { task, project, projectPath });
}

export async function startStopwatch(
  task?: string,
  project?: string,
  projectPath?: string
): Promise<void> {
  return invoke('start_stopwatch', { task, project, projectPath });
}

export async function pauseTimer(): Promise<void> {
  return invoke('pause_timer');
}

export async function resumeTimer(): Promise<void> {
  return invoke('resume_timer');
}

export async function stopTimer(): Promise<void> {
  return invoke('stop_timer');
}

export async function completePomodoro(): Promise<void> {
  return invoke('complete_pomodoro');
}

export async function skipBreak(): Promise<void> {
  return invoke('skip_break');
}

export async function completeBreak(): Promise<void> {
  return invoke('complete_break');
}

export async function setVaultPath(path: string): Promise<PomodoroConfig> {
  return invoke('set_vault_path', { path });
}

export async function getVaultPath(): Promise<string | null> {
  return invoke('get_vault_path');
}

export async function getConfig(): Promise<PomodoroConfig> {
  return invoke('get_config');
}

export async function scanProjects(): Promise<Project[]> {
  return invoke('scan_projects');
}

export async function scanTasks(): Promise<VaultTask[]> {
  return invoke('scan_tasks');
}

export async function getTrayTitle(): Promise<string> {
  return invoke('get_tray_title');
}

export async function getTodayStats(): Promise<TodayStats> {
  return invoke('get_today_stats');
}

export async function setDialogOpen(open: boolean): Promise<void> {
  return invoke('set_dialog_open', { open });
}

export async function setAutostart(enabled: boolean): Promise<void> {
  return invoke('set_autostart', { enabled });
}

export async function getAutostart(): Promise<boolean> {
  return invoke('get_autostart');
}
