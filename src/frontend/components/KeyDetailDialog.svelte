<script lang="ts">
  import { t } from '../i18n';
  import type { SSHKey } from '../stores/keys';
  
  export let show = false;
  export let key: SSHKey | null = null;
  export let publicKey = '';
  export let onClose: () => void;
  export let onCopy: () => void;
</script>

{#if show && key}
  <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50">
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full">
      <div class="p-6">
        <div class="flex justify-between items-start mb-4">
          <h3 class="text-xl font-bold text-gray-900 dark:text-white">
            {key.name}
          </h3>
          <button
            class="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
            on:click={onClose}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>
        
        <div class="space-y-4">
          <div>
            <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {$t('fingerprint')}
            </h4>
            <p class="text-sm text-gray-600 dark:text-gray-400 break-all font-mono">
              {key.fingerprint}
            </p>
          </div>
          
          <div>
            <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {$t('publicKey')}
            </h4>
            <div class="relative">
              <pre class="text-sm text-gray-600 dark:text-gray-400 bg-gray-50 dark:bg-gray-900/50 p-2 rounded border border-gray-200 dark:border-gray-700 overflow-x-auto font-mono whitespace-pre-wrap break-all">{publicKey}</pre>
              <button
                class="absolute top-2 right-2 p-1.5 text-white bg-blue-600 rounded hover:bg-blue-700 transition-colors"
                on:click={onCopy}
                title={$t('copy')}
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                  <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
              </button>
            </div>
          </div>
          
          <div>
            <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {$t('keyType')}
            </h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {key.key_type}
            </p>
          </div>
          
          <div>
            <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              {$t('lastUsed')}
            </h4>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {new Date(key.last_used).toLocaleString()}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}