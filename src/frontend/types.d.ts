declare module '*.svelte' {
  import type { ComponentType } from 'svelte';
  const component: ComponentType;
  export default component;
}

declare module '*.png' {
  const value: string;
  export default value;
}

interface SSHKey {
  name: string;
  fingerprint: string;
  last_used: string;
  key_type: string;
}