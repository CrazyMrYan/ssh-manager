<script lang="ts">
  import { keyStore, type SSHKey } from '../stores/keys';
  import { onMount } from 'svelte';
  import { t, locale } from '../i18n';
  import { confirm } from '@tauri-apps/api/dialog';
  import KeyDetailDialog from './KeyDetailDialog.svelte';
  import Notification from './Notification.svelte';
  
  export let keys: SSHKey[] = [];
  let keyName = '';
  let keyComment = '';
  let keyEmail = '';
  let status = '';
  let showKeyDialog = false;
  let showDetailDialog = false;
  let selectedKeyType = '';
  let selectedKey: SSHKey | null = null;
  let publicKeyContent = '';

  let refreshInterval: ReturnType<typeof setInterval>;

  let searchTerm = '';
  let currentPage = 1;
  const pageSize = 5;

  $: sortedKeys = [...keys].sort((a, b) => 
    new Date(b.last_used).getTime() - new Date(a.last_used).getTime()
  );

  $: filteredKeys = sortedKeys.filter(key => 
    key.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  $: paginatedKeys = filteredKeys
    .slice((currentPage - 1) * pageSize, currentPage * pageSize);

  $: totalPages = Math.ceil(filteredKeys.length / pageSize);

  onMount(() => {
    let mounted = true;

    (async () => {
      if (mounted) {
        await loadKeys();
        refreshInterval = setInterval(loadKeys, 5000);
      }
    })();

    return () => {
      mounted = false;
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
    };
  });

  async function loadKeys() {
    try {
      keys = await keyStore.loadKeys();
    } catch (e) {
      console.error('Failed to load keys:', e);
      status = $t('status.loadFailed');
    }
  }

  async function showGenerateKeyDialog(type: string) {
    selectedKeyType = type;
    keyName = type === 'ed25519' ? 'id_ed25519' : 'id_rsa';
    keyComment = '';
    keyEmail = '';
    showKeyDialog = true;
  }

  async function generateKey() {
    status = $t('status.generating');
    try {
      await keyStore.addKey(keyName, selectedKeyType, keyComment, keyEmail);
      keys = await keyStore.loadKeys();
      showKeyDialog = false;
      status = $t('status.generated', { 
        type: selectedKeyType.toUpperCase(), 
        name: keyName 
      });
      setTimeout(() => status = '', 5000);
    } catch (e) {
      console.error('Failed to generate key:', e);
      status = $t('status.generateFailed');
    }
  }

  async function copyKeyToClipboard(key: SSHKey) {
    status = $t('status.copying');
    try {
      const publicKey = await keyStore.getPublicKey(key.name);
      await keyStore.copyPublicKey(key.name);
      status = $t('status.copied');
      setTimeout(() => status = '', 3000);
    } catch (e) {
      console.error('Failed to copy key:', e);
      status = $t('status.copyFailed');
    }
  }

  async function showKeyDetails(key: SSHKey) {
    selectedKey = key;
    try {
      publicKeyContent = await keyStore.getPublicKey(key.name);
      showDetailDialog = true;
    } catch (e) {
      console.error('Failed to load public key:', e);
      status = $t('status.loadFailed');
    }
  }

  async function deleteKey(name: string) {
    const confirmed = await confirm(
      $t('confirmDeleteMessage', { name }),
      {
        title: $t('confirmDelete'),
        type: 'warning'
      }
    );

    if (confirmed) {
      try {
        await keyStore.deleteKey(name);
        keys = await keyStore.loadKeys();
        status = $t('status.deleted');
        setTimeout(() => status = '', 3000);
      } catch (e) {
        console.error('Failed to delete key:', e);
        status = $t('status.deleteFailed');
      }
    }
  }
</script>

<div class="space-y-4">
  <div class="flex justify-between items-center">
    <div class="space-x-2">
      <button
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium"
        on:click={() => showGenerateKeyDialog('ed25519')}
      >
        {$t('generateED25519')}
      </button>
      <button
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium"
        on:click={() => showGenerateKeyDialog('rsa')}
      >
        {$t('generateRSA')}
      </button>
    </div>
    <input
      type="text"
      bind:value={searchTerm}
      placeholder={$t('searchKey')}
      autocapitalize="off"
      class="w-64 px-3 py-1.5 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  {#if keys.length === 0}
    <p class="text-gray-500 dark:text-gray-400 text-center py-8">{$t('noKeys')}</p>
  {:else}
    <div class="overflow-x-auto">
      <table class="w-full text-left">
        <thead class="bg-gray-50 dark:bg-gray-700/50">
          <tr>
            <th class="px-4 py-3 text-sm font-medium text-gray-500 dark:text-gray-400">{$t('keyName')}</th>
            <th class="px-4 py-3 text-sm font-medium text-gray-500 dark:text-gray-400">{$t('keyType')}</th>
            <th class="px-4 py-3 text-sm font-medium text-gray-500 dark:text-gray-400">{$t('lastUsed')}</th>
            <th class="px-4 py-3 text-sm font-medium text-gray-500 dark:text-gray-400"></th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
          {#each paginatedKeys as key}
            <tr class="hover:bg-gray-50 dark:hover:bg-gray-800">
              <td class="px-4 py-3">
                <div>
                  <div class="font-medium text-gray-900 dark:text-white">{key.name}</div>
                  <div class="text-sm text-gray-500 dark:text-gray-400 font-mono truncate">{key.fingerprint}</div>
                </div>
              </td>
              <td class="px-4 py-3 text-sm text-gray-500 dark:text-gray-400">{key.key_type}</td>
              <td class="px-4 py-3 text-sm text-gray-500 dark:text-gray-400">
                {new Date(key.last_used).toLocaleString()}
              </td>
              <td class="px-4 py-3 text-right space-x-2">
                <button
                  class="text-sm text-blue-600 hover:text-blue-800"
                  on:click={() => copyKeyToClipboard(key)}
                >
                  {$t('copy')}
                </button>
                <button
                  class="text-sm text-blue-600 hover:text-blue-800"
                  on:click={() => showKeyDetails(key)}
                >
                  {$t('details')}
                </button>
                <button
                  class="text-sm text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300"
                  on:click={() => deleteKey(key.name)}
                >
                  {$t('delete')}
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}

  <div class="flex justify-between items-center text-sm text-gray-500 dark:text-gray-400">
    <div>
      {$t('showing')} {filteredKeys.length} {$t('items')}
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

<Notification
  message={status}
  type={status.includes('Failed') ? 'error' : 'success'}
/>

{#if showKeyDialog}
  <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
      <div class="p-6">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white">
          {$t('generateKey')} ({selectedKeyType.toUpperCase()})
        </h3>
        <div class="mt-4 space-y-4">
          <label class="block">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{$t('keyName')}</span>
            <input
              type="text"
              bind:value={keyName}
              autocapitalize="off"
              class="mt-1 block w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 dark:text-white"
            />
          </label>
          
          <label class="block">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{$t('keyEmail')} ({$t('optional')})</span>
            <input
              type="email"
              bind:value={keyEmail}
              placeholder="your.email@example.com"
              autocapitalize="off"
              class="mt-1 block w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 dark:text-white"
            />
          </label>
          
          <label class="block">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{$t('keyComment')} ({$t('optional')})</span>
            <input
              type="text"
              bind:value={keyComment}
              placeholder="Work laptop, Personal, etc."
              autocapitalize="off"
              class="mt-1 block w-full px-3 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 dark:text-white"
            />
          </label>
        </div>
      </div>
      <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700/50 rounded-b-lg flex justify-end space-x-4">
        <button
          class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white font-medium transition-colors"
          on:click={() => showKeyDialog = false}
        >
          {$t('cancel')}
        </button>
        <button
          class="px-6 py-2 bg-blue-600 text-white rounded-md font-medium hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 transition-colors"
          on:click={generateKey}
        >
          {$t('generate')}
        </button>
      </div>
    </div>
  </div>
{/if}

<KeyDetailDialog
  show={showDetailDialog}
  key={selectedKey}
  publicKey={publicKeyContent}
  onClose={() => showDetailDialog = false}
  onCopy={async () => {
    await keyStore.copyPublicKey(selectedKey?.name || '');
    status = $t('status.copied');
    setTimeout(() => status = '', 3000);
  }}
/>