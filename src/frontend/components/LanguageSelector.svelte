<script lang="ts">
  import { locale } from '../i18n';
  import { onMount } from 'svelte';
  
  const languages = [
    { code: 'zh', name: '中文' },
    { code: 'en', name: 'English' }
  ];

  let showDropdown = false;
  let buttonRef: HTMLButtonElement;

  onMount(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (buttonRef && !buttonRef.contains(event.target as Node)) {
        showDropdown = false;
      }
    };

    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  });
</script>

<div class="relative">
  <button
    bind:this={buttonRef}
    on:click={() => showDropdown = !showDropdown}
    class="inline-flex items-center justify-between w-24 px-3 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-transparent border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-700/50"
  >
    {languages.find(lang => lang.code === $locale)?.name}
    <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
    </svg>
  </button>
  
  {#if showDropdown}
    <div class="absolute right-0 mt-1 w-24 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md overflow-hidden">
      {#each languages as lang}
        <button
          class="block w-full px-4 py-2 text-sm text-left hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300"
          class:font-medium={$locale === lang.code}
          on:click={() => {
            $locale = lang.code;
            showDropdown = false;
          }}
        >
          {lang.name}
        </button>
      {/each}
    </div>
  {/if}
</div> 