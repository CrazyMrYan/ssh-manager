<script lang="ts">
  import { fade } from 'svelte/transition';
  import { onMount } from 'svelte';
  
  export let message = '';
  export let type: 'success' | 'error' = 'success';
  export let duration = type === 'error' ? 5000 : 3000;

  onMount(() => {
    if (message && duration > 0) {
      const timer = setTimeout(() => {
        message = '';
      }, duration);
      return () => clearTimeout(timer);
    }
  });
</script>

{#if message}
  <div
    transition:fade={{ duration: 200 }}
    class="fixed top-4 left-1/2 -translate-x-1/2 z-50 px-4 py-2 rounded-lg shadow-lg {
      type === 'error'
        ? 'bg-red-50 text-red-700 dark:bg-red-900/90 dark:text-red-400'
        : 'bg-green-50 text-green-700 dark:bg-green-900/90 dark:text-green-400'
    }"
  >
    {message}
  </div>
{/if} 