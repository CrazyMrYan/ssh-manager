<script lang="ts">
  import { t } from '../i18n';
  import { locale } from '../i18n';
  import { themeStore } from '../stores/theme';
  import { onMount } from 'svelte';
  
  const languages = [
    { code: 'zh', name: '中文' },
    { code: 'en', name: 'English' }
  ];
  
  const themes: Array<{value: 'light' | 'dark' | 'system', icon: string}> = [
    { value: 'light', icon: 'sun' },
    { value: 'dark', icon: 'moon' },
    { value: 'system', icon: 'computer-desktop' }
  ];
  
  let currentTheme: 'light' | 'dark' | 'system';
  
  onMount(() => {
    const unsubscribe = themeStore.subscribe(value => {
      currentTheme = value;
    });
    
    return unsubscribe;
  });
  
  function setTheme(theme: 'light' | 'dark' | 'system') {
    themeStore.setTheme(theme);
  }
</script>

<div class="space-y-8">
  <div>
    <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">{$t('settings.appearance')}</h2>
    
    <div class="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg">
      <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">{$t('settings.theme')}</h3>
      
      <div class="flex space-x-2">
        {#each themes as theme}
          <button
            class="flex items-center justify-center p-3 rounded-lg border {currentTheme === theme.value ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400' : 'border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700/50 text-gray-700 dark:text-gray-300'}"
            on:click={() => setTheme(theme.value)}
          >
            {#if theme.icon === 'sun'}
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            {:else if theme.icon === 'moon'}
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
              </svg>
            {:else}
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
            {/if}
          </button>
        {/each}
      </div>
      
      <div class="mt-2 text-xs text-gray-500 dark:text-gray-400">
        {#if currentTheme === 'light'}
          {$t('settings.lightThemeDescription')}
        {:else if currentTheme === 'dark'}
          {$t('settings.darkThemeDescription')}
        {:else}
          {$t('settings.systemThemeDescription')}
        {/if}
      </div>
    </div>
  </div>
  
  <div>
    <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">{$t('settings.language')}</h2>
    
    <div class="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg">
      <div class="grid grid-cols-2 gap-2">
        {#each languages as lang}
          <button
            class="flex items-center justify-center p-3 rounded-lg border {$locale === lang.code ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400' : 'border-gray-200 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-700/50 text-gray-700 dark:text-gray-300'}"
            on:click={() => $locale = lang.code}
          >
            {lang.name}
          </button>
        {/each}
      </div>
    </div>
  </div>
  
  <div>
    <h2 class="text-lg font-medium text-gray-900 dark:text-white mb-4">{$t('settings.about')}</h2>
    
    <div class="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg">
      <div class="text-sm text-gray-700 dark:text-gray-300">
        <p class="mb-2">{$t('settings.version')}: 1.0.0-beta.1</p>
        <p>{$t('settings.platform')}: {navigator.platform}</p>
      </div>
    </div>
  </div>
</div> 