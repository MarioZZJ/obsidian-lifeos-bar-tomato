export type TimerPhase = 'idle' | 'running' | 'paused' | 'shortBreak' | 'longBreak';
export type TimerMode = 'pomodoro' | 'stopwatch';

export interface TimerStatus {
  phase: TimerPhase;
  mode: TimerMode;
  elapsedSecs: number;
  remainingSecs: number | null;
  overtimeSecs: number;
  pomodoroCount: number;
  currentTask: string | null;
  currentProject: string | null;
  currentProjectPath: string | null;
}

export interface PomodoroConfig {
  pomodoroDuration: number;
  shortBreakDuration: number;
  longBreakDuration: number;
  longBreakInterval: number;
  autoStartBreak: boolean;
  pomodoroSound: boolean;
}

export interface Project {
  name: string;
  displayName: string;
  path: string;
  readmePath: string;
}

export interface VaultTask {
  text: string;
  filePath: string;
  lineNumber: number;
  projectTag: string | null;
  projectName: string | null;
}

export interface TodayStats {
  totalMinutes: number;
  pomodoroCount: number;
}
