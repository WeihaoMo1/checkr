<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import Network from '$lib/components/Network.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import InputOptions from '$lib/components/InputOptions.svelte';
  import { Io } from '$lib/io.svelte';
  import ParsedInput from '../Interpreter/ParsedInput.svelte';

  const io = new Io('Petrinet', { commands: '', steps: 10 });
</script>

<Env {io}>
  {#snippet inputView()}
    <StandardInput analysis="Petrinet" code="commands" {io}>
      <InputOptions title="Options">
        <label for="steps">Number of steps</label>
        <ParsedInput bind:value={io.input.steps} type="int" />
      </InputOptions>
    </StandardInput>
  {/snippet}
  
  {#snippet outputView({ output })}
    <div class="relative">
      <div class="absolute inset-0 grid overflow-auto">
        <Network dot={output.dot} />
      </div>
    </div>
  {/snippet}
</Env> 