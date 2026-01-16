<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import Network from '$lib/components/Network.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import InputOptions from '$lib/components/InputOptions.svelte';
  import { Io } from '$lib/io.svelte';
  import ParsedInput from '../Interpreter/ParsedInput.svelte';

  const io = new Io('Petrinet', { commands: '', steps: 0 });

  function getPlaceNames(map: Record<string, number>[]): string[] {
    const placeSet = new Set<string>();
    for (const tokens of map) {
      for (const place in tokens) {
        placeSet.add(place);
      }
    }
    return Array.from(placeSet).sort();
  }
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
    {@const places = getPlaceNames(output.map)}
    <div class="grid min-h-0 grid-cols-[auto_1fr]">
      <div class="overflow-auto border-r border-t bg-slate-900">
        <div
          class="**:border-t grid w-full grid-flow-dense"
          style="grid-template-columns: auto repeat({places.length}, max-content);"
        >

          <div class="border-none px-6 text-center font-mono font-bold">Step</div>
          {#each places as place}
            <div class="border-none px-6 text-center font-mono font-bold">
              {place}
            </div>
          {/each}

          {#each output.map as tokens, stepIdx}
            <div class="px-4 py-0.5 text-center font-mono text-sm">
              {stepIdx}
            </div>
            {#each places as place}
              <div class="px-2 py-0.5 text-center font-mono text-sm">
                {tokens[place] ?? 0}
              </div>
            {/each}
          {/each}
        </div>
      </div>

      <div class="relative">
        <div class="absolute inset-0 grid overflow-auto">
          <Network dot={output.dot} />
        </div>
      </div>
    </div>
  {/snippet}
</Env>