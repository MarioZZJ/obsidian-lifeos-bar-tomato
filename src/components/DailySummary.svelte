<script lang="ts">
  import { todayStats } from '../lib/stores';
  import { formatMinutes } from '../lib/utils/format';

  let totalTime = $derived(formatMinutes($todayStats.totalMinutes));
  let pomodoroCount = $derived($todayStats.pomodoroCount);
</script>

<div class="daily-summary">
  <span class="stat">今日: {totalTime}</span>
  <span class="stat">🍅×{pomodoroCount}</span>
</div>

<style>
  .daily-summary {
    display: flex;
    justify-content: center;
    gap: 16px;
    padding: 12px;
    border-top: 1px solid var(--glass-border);
    font-size: 14px;
    color: var(--text-secondary);
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(255, 255, 255, 0.35);
    padding: 4px 12px;
    border-radius: 12px;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }
</style>
