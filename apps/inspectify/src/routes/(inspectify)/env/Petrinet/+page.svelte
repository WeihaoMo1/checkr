<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import Network from '$lib/components/Network.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import { Io } from '$lib/io.svelte';

  const io = new Io('Petrinet', { commands: '' });
</script>

<Env {io}>
  {#snippet inputView()}
    <StandardInput analysis="Petrinet" code="commands" {io} />
  {/snippet}
<!--   {#snippet outputView({ output, referenceOutput })}
    <div class="relative">
      <div class="absolute inset-0 grid">
        <pre class="p-2"><code
            >{#if output}{output.result}{/if}</code
          ></pre>
      </div>
    </div>
  {/snippet} -->
  {#snippet outputView({ output })}
    <div class="relative">
      <div class="absolute inset-0 grid overflow-auto">
        <Network dot={ output.dot} />
      </div>
    </div>
  {/snippet}
</Env>
