<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import { Io } from '$lib/io.svelte';

  const io = new Io('Playground', { text: '' });
</script>

<Env {io}>
  {#snippet inputView()}
    <StandardInput analysis="Playground" code="text" {io} />
  {/snippet}
  {#snippet outputView({ output, referenceOutput })}
    <div class="relative">
      <div class="absolute inset-0 grid">
        <pre class="p-2"><code
            >{#if output}{output.result}{/if}</code
          ></pre>
      </div>
    </div>
  {/snippet}
</Env>