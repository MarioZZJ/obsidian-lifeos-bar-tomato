<script lang="ts">
  import { vaultPath, config, projects, tasks, autostart } from '../lib/stores';
  import * as api from '../lib/api/tauri';
  import { open } from '@tauri-apps/plugin-dialog';

  let { onClose }: { onClose: () => void } = $props();

  let currentPath = $derived($vaultPath);
  let cfg = $derived($config);
  let projectCount = $derived($projects.length);
  let taskCount = $derived($tasks.length);
  let autostartEnabled = $state($autostart);

  async function handleAutostartToggle() {
    try {
      await api.setAutostart(autostartEnabled);
      autostart.set(autostartEnabled);
    } catch (e) {
      console.error('Failed to update autostart:', e);
      autostartEnabled = !autostartEnabled;
    }
  }

  let errorMessage = $state('');

  async function selectVault() {
    errorMessage = '';
    try {
      // Prevent window from hiding while dialog is open
      await api.setDialogOpen(true);

      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择 Obsidian Vault 目录',
      });

      await api.setDialogOpen(false);

      if (selected && typeof selected === 'string') {
        try {
          const newConfig = await api.setVaultPath(selected);
          config.set(newConfig);
          vaultPath.set(selected);

          // Refresh projects and tasks
          const projs = await api.scanProjects();
          projects.set(projs);

          const tasksData = await api.scanTasks();
          tasks.set(tasksData);
        } catch (e) {
          errorMessage = '无效的 Vault 目录或未安装 lifeos-pro 插件';
          console.error('Failed to set vault path:', e);
        }
      }
    } catch (e) {
      await api.setDialogOpen(false);
      errorMessage = '打开目录选择器失败';
      console.error('Failed to open directory dialog:', e);
    }
  }
</script>

<div class="settings">
  <div class="settings-header">
    <h3>⚙️ 设置</h3>
    <button class="close-btn" onclick={onClose}>×</button>
  </div>

  <div class="settings-content">
    <!-- Error message -->
    {#if errorMessage}
      <div class="error-message">
        ⚠️ {errorMessage}
      </div>
    {/if}

    <!-- Vault path -->
    <div class="setting-item">
      <div class="label">Vault 路径</div>
      <div class="vault-path">
        {#if currentPath}
          <span class="path-text" title={currentPath}>
            .../{currentPath.split('/').pop()}
          </span>
        {:else}
          <span class="path-text empty">未设置</span>
        {/if}
        <button class="btn-small" onclick={selectVault}>选择</button>
      </div>
    </div>

    {#if currentPath}
      <!-- Config display -->
      <div class="setting-item">
        <div class="label">番茄钟时长</div>
        <span>{cfg.pomodoroDuration} 分钟</span>
      </div>

      <div class="setting-item">
        <div class="label">短休息</div>
        <span>{cfg.shortBreakDuration} 分钟</span>
      </div>

      <div class="setting-item">
        <div class="label">长休息</div>
        <span>{cfg.longBreakDuration} 分钟 (每 {cfg.longBreakInterval} 个番茄)</span>
      </div>

      <div class="setting-item">
        <div class="label">检测到的项目</div>
        <span>{projectCount} 个</span>
      </div>

      <div class="setting-item">
        <div class="label">未完成任务</div>
        <span>{taskCount} 个</span>
      </div>

      <p class="note">
        时长设置从 lifeos-pro 插件同步，如需修改请在 Obsidian 中设置。
      </p>
    {/if}

    <!-- Autostart toggle -->
    <div class="setting-item">
      <label for="autostart-checkbox">开机自启</label>
      <input
        id="autostart-checkbox"
        type="checkbox"
        bind:checked={autostartEnabled}
        onchange={handleAutostartToggle}
        class="checkbox-toggle"
      />
    </div>
  </div>
</div>

<style>
  .settings {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(255, 255, 255, 0.6);
    backdrop-filter: blur(30px) saturate(180%);
    -webkit-backdrop-filter: blur(30px) saturate(180%);
    display: flex;
    flex-direction: column;
    border-radius: 12px;
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    border-bottom: 1px solid var(--glass-border);
  }

  .settings-header h3 {
    margin: 0;
    font-size: 16px;
    color: var(--text-primary);
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: var(--text-secondary);
    transition: color 0.2s;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .settings-content {
    padding: 16px;
    flex: 1;
    overflow-y: auto;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid rgba(243, 244, 246, 0.4);
  }

  .setting-item label,
  .setting-item .label {
    font-size: 14px;
    color: rgba(0, 0, 0, 0.75);
  }

  .setting-item span {
    font-size: 14px;
    color: var(--text-secondary);
  }

  .vault-path {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .path-text {
    font-size: 13px;
    color: var(--text-secondary);
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .path-text.empty {
    color: var(--text-tertiary);
    font-style: italic;
  }

  .btn-small {
    padding: 4px 12px;
    border: 1px solid rgba(209, 213, 219, 0.5);
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.5);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    cursor: pointer;
    font-size: 12px;
    transition: all 0.2s;
  }

  .btn-small:hover {
    background: rgba(255, 255, 255, 0.7);
    border-color: rgba(209, 213, 219, 0.7);
  }

  .note {
    margin-top: 16px;
    font-size: 12px;
    color: var(--text-tertiary);
    text-align: center;
  }

  .checkbox-toggle {
    cursor: pointer;
    width: 18px;
    height: 18px;
    accent-color: rgba(34, 197, 94, 0.8);
  }

  .error-message {
    padding: 12px;
    margin-bottom: 8px;
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    font-size: 13px;
    color: rgba(239, 68, 68, 0.9);
  }
</style>
