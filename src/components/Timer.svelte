<script lang="ts">
  import { timerStatus, formattedTime, progress, config } from '../lib/stores';
  import * as api from '../lib/api/tauri';

  // Reactive values
  let phase = $derived($timerStatus.phase);
  let mode = $derived($timerStatus.mode);
  let displayTime = $derived($formattedTime);
  let progressValue = $derived($progress);
  let pomodoroCount = $derived($timerStatus.pomodoroCount);
  let isOvertime = $derived($timerStatus.overtimeSecs > 0);

  // Circle progress calculations
  const radius = 80;
  const circumference = 2 * Math.PI * radius;
  let strokeDashoffset = $derived(circumference * (1 - progressValue));

  function getPhaseColor(): string {
    if (isOvertime) return '#f97316'; // orange for overtime
    switch (phase) {
      case 'running':
        return '#22c55e'; // green
      case 'paused':
        return '#eab308'; // yellow
      case 'shortBreak':
      case 'longBreak':
        return '#3b82f6'; // blue
      default:
        return '#6b7280'; // gray
    }
  }

  let phaseColor = $derived(getPhaseColor());

  function getPhaseIcon(): string {
    if (isOvertime) return '✅';
    switch (phase) {
      case 'running':
        return mode === 'pomodoro' ? '🍅' : '🕙';
      case 'paused':
        return '⏸';
      case 'shortBreak':
      case 'longBreak':
        return '☕';
      default:
        return '🍅';
    }
  }

  let phaseIcon = $derived(getPhaseIcon());

  async function handlePause() {
    await api.pauseTimer();
  }

  async function handleResume() {
    await api.resumeTimer();
  }

  async function handleStop() {
    await api.stopTimer();
  }

  async function handleSkipBreak() {
    await api.skipBreak();
  }

  async function handleCompletePomodoro() {
    await api.completePomodoro();
  }
</script>

<div class="timer-container">
  <!-- Progress Ring -->
  <div class="progress-ring">
    <svg width="200" height="200" viewBox="0 0 200 200">
      <!-- Background circle -->
      <circle
        cx="100"
        cy="100"
        r={radius}
        fill="none"
        stroke="rgba(0, 0, 0, 0.1)"
        stroke-width="8"
      />
      <!-- Progress circle -->
      <circle
        cx="100"
        cy="100"
        r={radius}
        fill="none"
        stroke={phaseColor}
        stroke-width="8"
        stroke-linecap="round"
        stroke-dasharray={circumference}
        stroke-dashoffset={strokeDashoffset}
        transform="rotate(-90 100 100)"
        style="transition: stroke-dashoffset 0.5s ease"
      />
    </svg>
    <div class="timer-display">
      <span class="timer-icon">{phaseIcon}</span>
      <span class="timer-time" class:overtime={isOvertime}>{displayTime}</span>
    </div>
  </div>

  <!-- Controls -->
  <div class="controls">
    {#if phase === 'running' && isOvertime}
      <button class="btn btn-primary" onclick={handleCompletePomodoro}>☕ 休息</button>
      <button class="btn btn-danger" onclick={handleStop}>■ 停止</button>
    {:else if phase === 'running'}
      <button class="btn btn-warning" onclick={handlePause}>⏸ 暂停</button>
      <button class="btn btn-danger" onclick={handleStop}>■ 停止</button>
    {:else if phase === 'paused'}
      <button class="btn btn-success" onclick={handleResume}>▶ 继续</button>
      <button class="btn btn-danger" onclick={handleStop}>■ 停止</button>
    {:else if phase === 'shortBreak' || phase === 'longBreak'}
      <button class="btn btn-primary" onclick={handleSkipBreak}>⏭ 跳过</button>
    {/if}
  </div>

  <!-- Pomodoro count -->
  {#if pomodoroCount > 0}
    <div class="pomodoro-count">
      🍅 × {pomodoroCount}
    </div>
  {/if}
</div>

<style>
  .timer-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 16px;
  }

  .progress-ring {
    position: relative;
    width: 200px;
    height: 200px;
    filter: drop-shadow(0 4px 12px var(--glass-shadow));
  }

  .timer-display {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    background: rgba(255, 255, 255, 0.4);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-radius: 50%;
    width: 140px;
    height: 140px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 8px var(--glass-shadow);
  }

  .timer-icon {
    font-size: 24px;
    display: block;
    margin-bottom: 4px;
  }

  .timer-time {
    font-size: 36px;
    font-weight: bold;
    font-family: monospace;
    color: var(--text-primary);
  }

  .timer-time.overtime {
    color: #f97316;
    font-size: 32px;
  }

  .controls {
    display: flex;
    gap: 12px;
    margin-top: 16px;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    box-shadow: 0 2px 8px var(--glass-shadow);
  }

  .btn:hover {
    transform: scale(1.05);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .btn-success {
    background: var(--color-running);
    color: white;
  }

  .btn-warning {
    background: var(--color-paused);
    color: white;
  }

  .btn-danger {
    background: var(--color-danger);
    color: white;
  }

  .btn-primary {
    background: var(--color-break);
    color: white;
  }

  .pomodoro-count {
    margin-top: 12px;
    font-size: 14px;
    color: var(--text-secondary);
    background: rgba(255, 255, 255, 0.4);
    padding: 4px 12px;
    border-radius: 12px;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }
</style>
