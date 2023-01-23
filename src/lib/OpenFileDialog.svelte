<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    export let onSelect = null
    export let fileNumber = 0
    
    let selected = -1
    let rows = []

    async function open_file_one() {
        const delimiter = localStorage.getItem('delimiter') || ';'
        const result = await invoke('read_file', {fileNumber, delimiter})
        rows = result
    }

    function handleSelect(fieldIndex: number) {
        selected = fieldIndex
        onSelect?.(fieldIndex)
    }
</script>

ID: {selected != -1 ? rows[0][selected] : ''}
<button on:click="{open_file_one}">Open File</button>
<table>
    {#each rows as row, rowIndex}
        <tr>
            {#if rowIndex === 0}
                {#each row as field, fieldIndex}
                    <th>
                        <button on:click={() => handleSelect(fieldIndex)}>{field}</button>
                    </th>
                {/each}
            {:else}
                {#each row as field, fieldIndex}
                    <td>
                        {field}
                    </td>
                {/each}
            {/if}
        </tr>
    {/each}
</table>
