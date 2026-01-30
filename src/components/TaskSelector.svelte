<script lang="ts">
  import { projects, tasks, timerStatus } from '../lib/stores';
  import * as api from '../lib/api/tauri';
  import type { VaultTask, Project } from '../lib/types';

  let searchQuery = $state('');
  let manualTask = $state('');
  let selectedProjectIdx = $state(-1);
  let showManualInput = $state(false);

  // Group tasks by project
  let groupedTasks = $derived.by(() => {
    const groups: Map<string, VaultTask[]> = new Map();
    const query = searchQuery.toLowerCase();

    for (const task of $tasks) {
      if (query && !task.text.toLowerCase().includes(query)) {
        continue;
      }

      const projectName = task.projectName || '其他';
      if (!groups.has(projectName)) {
        groups.set(projectName, []);
      }
      groups.get(projectName)!.push(task);
    }

    return groups;
  });

  let phase = $derived($timerStatus.phase);

  async function startWithTask(task: VaultTask) {
    // Find project
    const project = $projects.find((p) =>
      task.projectTag?.includes(p.name.split('-').pop() || '')
    );

    await api.startPomodoro(
      task.text,
      project?.displayName,
      project?.readmePath
    );
  }

  async function startStopwatchWithTask(task: VaultTask) {
    const project = $projects.find((p) =>
      task.projectTag?.includes(p.name.split('-').pop() || '')
    );

    await api.startStopwatch(
      task.text,
      project?.displayName,
      project?.readmePath
    );
  }

  async function startManual() {
    if (!manualTask.trim()) return;

    const project = selectedProjectIdx >= 0 ? $projects[selectedProjectIdx] : null;

    await api.startPomodoro(
      manualTask,
      project?.displayName,
      project?.readmePath
    );

    manualTask = '';
    showManualInput = false;
  }

  async function startManualStopwatch() {
    if (!manualTask.trim()) return;

    const project = selectedProjectIdx >= 0 ? $projects[selectedProjectIdx] : null;

    await api.startStopwatch(
      manualTask,
      project?.displayName,
      project?.readmePath
    );

    manualTask = '';
    showManualInput = false;
  }
</script>

{#if phase === 'idle'}
  <div class="task-selector">
    <div class="section-header">── 选择任务 ──</div>

    <!-- Search -->
    <div class="search-box">
      <input
        type="text"
        placeholder="🔍 搜索任务..."
        bind:value={searchQuery}
      />
    </div>

    <!-- Task list -->
    <div class="task-list">
      {#each groupedTasks as [projectName, projectTasks]}
        <div class="project-group">
          <div class="project-name">{projectName}:</div>
          {#each projectTasks.slice(0, 5) as task}
            <div class="task-item">
              <span class="task-checkbox">□</span>
              <span class="task-text" title={task.text}>
                {task.text.slice(0, 40)}{task.text.length > 40 ? '...' : ''}
              </span>
              <div class="task-actions">
                <button
                  class="task-btn pomodoro"
                  onclick={() => startWithTask(task)}
                  title="番茄钟"
                >🍅</button>
                <button
                  class="task-btn stopwatch"
                  onclick={() => startStopwatchWithTask(task)}
                  title="秒表"
                >🕙</button>
              </div>
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <!-- Manual input toggle -->
    <button class="manual-toggle" onclick={() => (showManualInput = !showManualInput)}>
      ✏️ 手动输入任务
    </button>

    {#if showManualInput}
      <div class="manual-input">
        <input
          type="text"
          placeholder="输入任务描述..."
          bind:value={manualTask}
        />
        <select bind:value={selectedProjectIdx}>
          <option value={-1}>无项目</option>
          {#each $projects as project, idx}
            <option value={idx}>{project.displayName}</option>
          {/each}
        </select>
        <div class="manual-actions">
          <button class="btn btn-success" onclick={startManual}>🍅 番茄钟</button>
          <button class="btn btn-primary" onclick={startManualStopwatch}>🕙 秒表</button>
        </div>
      </div>
    {/if}
  </div>
{:else}
  <!-- Current task display -->
  <div class="current-task">
    {#if $timerStatus.currentTask}
      <div class="task-label">任务:</div>
      <div class="task-value">{$timerStatus.currentTask}</div>
    {/if}
    {#if $timerStatus.currentProject}
      <div class="task-label">项目:</div>
      <div class="task-value">{$timerStatus.currentProject}</div>
    {/if}
  </div>
{/if}

<style>
  .task-selector {
    padding: 12px;
    border-top: 1px solid var(--glass-border);
  }

  .section-header {
    text-align: center;
    color: var(--text-tertiary);
    font-size: 12px;
    margin-bottom: 8px;
  }

  .search-box input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid rgba(255, 255, 255, 0.4);
    border-radius: 8px;
    font-size: 14px;
    background: rgba(255, 255, 255, 0.4);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    color: var(--text-primary);
  }

  .search-box input::placeholder {
    color: var(--text-tertiary);
  }

  .search-box input:focus {
    outline: none;
    border-color: rgba(59, 130, 246, 0.5);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .task-list {
    max-height: 200px;
    overflow-y: auto;
    margin-top: 8px;
  }

  .project-group {
    margin-bottom: 8px;
  }

  .project-name {
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .task-item {
    display: flex;
    align-items: center;
    padding: 6px 8px;
    border-radius: 6px;
    font-size: 13px;
    transition: background 0.2s;
  }

  .task-item:hover {
    background: rgba(255, 255, 255, 0.4);
  }

  .task-checkbox {
    margin-right: 8px;
    color: var(--text-tertiary);
  }

  .task-text {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-primary);
  }

  .task-actions {
    display: flex;
    gap: 4px;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .task-item:hover .task-actions {
    opacity: 1;
  }

  .task-btn {
    padding: 4px 8px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    transition: transform 0.2s;
  }

  .task-btn:hover {
    transform: scale(1.1);
  }

  .task-btn.pomodoro {
    background: rgba(254, 243, 199, 0.8);
  }

  .task-btn.stopwatch {
    background: rgba(219, 234, 254, 0.8);
  }

  .manual-toggle {
    width: 100%;
    padding: 8px;
    margin-top: 8px;
    border: 1px dashed rgba(209, 213, 219, 0.5);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.25);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 13px;
    transition: all 0.2s;
  }

  .manual-toggle:hover {
    background: rgba(255, 255, 255, 0.45);
    border-color: rgba(209, 213, 219, 0.7);
  }

  .manual-input {
    margin-top: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .manual-input input,
  .manual-input select {
    padding: 8px 12px;
    border: 1px solid rgba(255, 255, 255, 0.4);
    border-radius: 8px;
    font-size: 14px;
    background: rgba(255, 255, 255, 0.4);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    color: var(--text-primary);
  }

  .manual-actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    flex: 1;
    padding: 8px;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 13px;
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    box-shadow: 0 2px 6px var(--glass-shadow);
    transition: all 0.2s;
  }

  .btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.15);
  }

  .btn-success {
    background: var(--color-running);
    color: white;
  }

  .btn-primary {
    background: var(--color-break);
    color: white;
  }

  .current-task {
    padding: 12px;
    border-top: 1px solid var(--glass-border);
  }

  .task-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .task-value {
    font-size: 14px;
    color: var(--text-primary);
    margin-bottom: 8px;
  }
</style>
