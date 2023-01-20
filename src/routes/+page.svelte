<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import OpenFileDialog from '../lib/OpenFileDialog.svelte'
    
    let firstSelectedFieldIndex = -1
    let secondSelectedFieldIndex = -1

    let selectedColumnIndex = -1

    let result = []

    function handleFirstSelect(fieldIndex: number) {
        firstSelectedFieldIndex = fieldIndex
    }

    function handleSecondSelect(fieldIndex: number) {
        secondSelectedFieldIndex = fieldIndex
    }

    async function connectFiles() {
        result = await invoke('connect', {firstIndex: firstSelectedFieldIndex, secondIndex: secondSelectedFieldIndex})
    }

    async function removeColumn() {
        result = await invoke('remove_column', {columnIndex: selectedColumnIndex})
    }

    async function save() {
        await invoke('save', {delimiter: localStorage.getItem('delimiter') || ';'})
    }
</script>

<a href="/settings">Settings</a>

<br />

<OpenFileDialog onSelect={handleFirstSelect} fileNumber={0} />

<br />

<OpenFileDialog onSelect={handleSecondSelect} fileNumber={1} />

<br />

<button on:click={connectFiles} disabled={firstSelectedFieldIndex + secondSelectedFieldIndex == -2}>
    Connect
</button>

{#if result.length > 0}
<button on:click={removeColumn} disabled={selectedColumnIndex == -1}>
    Remove Column {selectedColumnIndex != -1 ? result[0][selectedColumnIndex] : ''}
</button>

<button on:click={save}>
    Save
</button>
{/if}

<table>
    {#each result as record, recordIndex}
        {#if recordIndex === 0}
            <tr>
                {#each record as field, fieldIndex}
                    <td class={fieldIndex === selectedColumnIndex ? 'selected' : ''}>
                        <button on:click={() => selectedColumnIndex = fieldIndex}>{field}</button>
                    </td>
                {/each}
            </tr>
        {:else}
            <tr>
                {#each record as field, fieldIndex}
                    <td class={fieldIndex === selectedColumnIndex ? 'selected' : ''}>{field}</td>
                {/each}
            </tr>
        {/if}
    {/each}
</table>

<style>
.selected {
    background-color: blue;
}
</style>
