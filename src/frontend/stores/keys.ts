import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

export interface SSHKey {
  name: string;
  fingerprint: string;
  last_used: string;
  key_type: string;
}

function createKeyStore() {
  const { subscribe, set } = writable<SSHKey[]>([]);

  return {
    subscribe,
    loadKeys: async () => {
      const keys = await invoke<SSHKey[]>('get_ssh_keys');
      set(keys);
      return keys;
    },
    addKey: async (name: string, keyType: string, comment?: string, email?: string) => {
      await invoke('generate_ssh_key', { name, keyType, comment, email });
      const keys = await invoke<SSHKey[]>('get_ssh_keys');
      set(keys);
    },
    deleteKey: async (name: string) => {
      await invoke('delete_ssh_key', { name });
    },
    copyPublicKey: async (name: string) => {
      await invoke('copy_public_key', { name });
    },
    getPublicKey: async (name: string) => {
      return await invoke<string>('get_public_key', { name });
    }
  };
}

export const keyStore = createKeyStore();