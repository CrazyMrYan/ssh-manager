<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api';
  import { confirm } from '@tauri-apps/api/dialog';
  import { t } from '../i18n';
  
  interface ConfigItem {
    key: string;
    value: string;
    lastModified: number;
  }

  let configs: ConfigItem[] = [];
  let searchTerm = '';
  let currentPage = 1;
  const pageSize = 5;
  let newKey = '';
  let newValue = '';
  let saveStatus = '';
  let originalValues = new Map<string, string>();
  
  $: filteredConfigs = configs.filter(config => 
    config.key.toLowerCase().includes(searchTerm.toLowerCase())
  );

  $: paginatedConfigs = filteredConfigs
    .slice((currentPage - 1) * pageSize, currentPage * pageSize);

  $: totalPages = Math.ceil(filteredConfigs.length / pageSize);

  function changePage(page: number) {
    currentPage = page;
  }

  onMount(async () => {
    await loadConfigs();
  });

  async function loadConfigs() {
    try {
      const allConfigs = await invoke<Record<string, string>>('get_all_git_configs');
      configs = Object.entries(allConfigs).map(([key, value]) => ({
        key,
        value,
        lastModified: Date.now()
      }));
    } catch (e) {
      console.error('Failed to load git configs:', e);
      saveStatus = $t('status.loadFailed');
    }
  }

  function showStatus(message: string, isError = false) {
    saveStatus = message;
    if (message) {
      setTimeout(() => saveStatus = '', isError ? 5000 : 3000);
    }
  }

  function showError(error: unknown) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    showStatus($t('status.saveFailed') + ': ' + errorMessage, true);
  }

  async function addConfig() {
    if (!newKey || !newValue) return;
    
    try {
      await invoke('set_git_config', { key: newKey, value: newValue });
      const newConfig = { 
        key: newKey, 
        value: newValue, 
        lastModified: Date.now() 
      };
      configs = [...configs, newConfig];  // 添加到列表末尾
      originalValues.set(newKey, newValue);
      newKey = '';
      newValue = '';
      showStatus($t('status.saved'));
    } catch (e) {
      console.error('Failed to add git config:', e);
      showError(e);
    }
  }

  async function saveConfig(item: ConfigItem) {
    if (item.value === originalValues.get(item.key)) return;
    
    try {
      await invoke('set_git_config', { key: item.key, value: item.value });
      item.lastModified = Date.now();
      originalValues.set(item.key, item.value);
      showStatus($t('status.saved'));
    } catch (e) {
      console.error('Failed to save git config:', e);
      showError(e);
    }
  }

  async function deleteConfig(key: string) {
    const confirmed = await confirm(
      $t('confirmDeleteConfig', { key }),
      {
        title: $t('confirmDelete'),
        type: 'warning'
      }
    );

    if (confirmed) {
      try {
        await invoke('delete_git_config', { key });
        configs = configs.filter(c => c.key !== key);
        originalValues.delete(key);
        showStatus($t('status.deleted'));
      } catch (e) {
        console.error('Failed to delete git config:', e);
        showStatus($t('status.deleteFailed'), true);
      }
    }
  }
</script>

<div class="space-y-4">
  <div class="flex justify-between items-center">
    <div class="flex-1 max-w-xl flex space-x-2">
      <input
        type="text"
        bind:value={newKey}
        placeholder={$t('newConfigKey')}
        autocapitalize="off"
        class="flex-1 px-3 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
      <input
        type="text"
        bind:value={newValue}
        placeholder={$t('newConfigValue')}
        autocapitalize="off"
        class="flex-1 px-3 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
      <button
        class="px-4 py-1.5 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        on:click={addConfig}
      >
        {$t('add')}
      </button>
    </div>
    <input
      type="text"
      bind:value={searchTerm}
      placeholder={$t('searchConfig')}
      autocapitalize="off"
      class="w-64 px-3 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  <div class="grid gap-2">
    {#each paginatedConfigs as config}
      <div class="flex items-center space-x-2 p-2 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div class="flex-1 flex items-center space-x-2">
          <div class="flex-1">
            <div class="text-xs text-gray-500 dark:text-gray-400 mb-1">{config.key}</div>
            <input
              type="text"
              bind:value={config.value}
              on:blur={() => saveConfig(config)}
              class="w-full bg-white dark:bg-gray-700 text-sm text-gray-900 dark:text-white px-2 py-1 rounded border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-1 focus:ring-blue-500"
            />
          </div>
          <button
            class="text-xs text-red-600 hover:text-red-800 px-2"
            on:click={() => deleteConfig(config.key)}
          >
            {$t('delete')}
          </button>
        </div>
      </div>
    {/each}
  </div>

  <div class="flex justify-between items-center text-sm text-gray-500 dark:text-gray-400">
    <div>
      {$t('showing')} {filteredConfigs.length} {$t('items')}
    </div>
    {#if totalPages > 1}
      <div class="flex space-x-1">
        {#each Array(totalPages) as _, i}
          <button
            class="px-2.5 py-1 rounded {currentPage === i + 1 
              ? 'bg-blue-600 text-white' 
              : 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700'}"
            on:click={() => currentPage = i + 1}
          >
            {i + 1}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>