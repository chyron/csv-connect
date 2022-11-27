<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    let getSettings = invoke('get_settings')

    async function saveSettings(event) {
        event.preventDefault()
        await invoke('set_settings')
        // await invoke('write_data', {
        //     key: 'settings',
        //     value: {
        //         delimiter: event.target.elements['delimiter'].value,
        //     },
        // })
    }
</script>

<a href="/">Close</a>

<h1>Settings</h1>

{#await getSettings}
    loading...
{:then settings}
    <form on:submit={saveSettings}>
        Delimiter: <input type="text" name="delimiter" value={settings.delimiter} />
        <button type="submit">Save</button>
    </form>
{/await}
