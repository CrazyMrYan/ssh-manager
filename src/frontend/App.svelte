<script lang="ts">
  import { t } from './i18n';
  import KeyList from './components/KeyList.svelte';
  import ConfigEditor from './components/ConfigEditor.svelte';
  import Settings from './components/Settings.svelte';
  import { onMount } from 'svelte';
  import logo from '../../icons/128x128.png';
  
  let activeTab = 'keys';
  let showSettings = false;

  onMount(() => {
    console.log('App mounted');
  });

  function toggleSettings() {
    showSettings = !showSettings;
  }
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col">
  <header class="bg-white dark:bg-gray-800 shadow-sm fixed top-0 left-0 right-0 z-10">
    <div class="max-w-6xl mx-auto px-4 py-4">
      <div class="flex justify-between items-center">
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
          <img 
            src={logo}
            alt="SSH Manager"
            class="w-8 h-8"
          />
          SSH Manager
        </h1>
        <button 
          class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300"
          on:click={toggleSettings}
          aria-label={$t('settingsTab')}
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
          </svg>
        </button>
      </div>
    </div>
  </header>

  <main class="flex-1 flex flex-col w-full mt-16">
    <div class="bg-white dark:bg-gray-800 flex flex-col flex-grow w-full">
      <nav class="border-b border-gray-200 dark:border-gray-700 w-full">
        <div class="flex max-w-6xl mx-auto px-4">
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {
              activeTab === 'keys'
                ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
            }"
            on:click={() => (activeTab = 'keys')}
          >
            <div class="flex items-center gap-2">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
              </svg>
              {$t('sshKeys')}
            </div>
          </button>
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {
              activeTab === 'config'
                ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
            }"
            on:click={() => (activeTab = 'config')}
          >
            <div class="flex items-center gap-2">
              <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
              </svg>
              {$t('gitConfig')}
            </div>
          </button>
        </div>
      </nav>
      
      <div class="flex-1 overflow-auto p-6 max-w-6xl mx-auto w-full">
        {#if activeTab === 'keys'}
          <KeyList />
        {:else if activeTab === 'config'}
          <ConfigEditor />
        {/if}
      </div>
    </div>
  </main>
</div>

{#if showSettings}
  <div class="fixed inset-0 bg-black bg-opacity-50 z-20 flex items-center justify-center p-4">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">{$t('settingsTab')}</h2>
        <button 
          class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-600 dark:text-gray-300"
          on:click={toggleSettings}
          aria-label="Close"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      <div class="p-6">
        <Settings />
      </div>
    </div>
  </div>
{/if} 