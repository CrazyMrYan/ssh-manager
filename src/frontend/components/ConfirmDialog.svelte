<script lang="ts">
  import { t } from '../i18n';
  
  export let show = false;
  export let title = '';
  export let message = '';
  export let onConfirm: () => void;
  export let onCancel: () => void;
</script>

{#if show}
  <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
      <div class="p-6">
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-3">
          {title}
        </h3>
        <p class="text-gray-600 dark:text-gray-300">
          {message}
        </p>
      </div>
      <div class="px-6 py-4 bg-gray-50 dark:bg-gray-700/50 rounded-b-lg flex justify-end space-x-4">
        <button
          class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white font-medium transition-colors"
          on:click={onCancel}
        >
          {$t('cancel')}
        </button>
        <button
          class="px-6 py-2 bg-red-600 text-white rounded-md font-medium hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 transition-colors"
          on:click={onConfirm}
        >
          {$t('confirm')}
        </button>
      </div>
    </div>
  </div>
{/if} 