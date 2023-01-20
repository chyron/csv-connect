<script lang="ts">
    let delimiter = localStorage.getItem('delimiter') || ';'

    function saveSettings(event) {
        event.preventDefault()
        localStorage.setItem('delimiter', event.target.elements['delimiter'].value)
    }
</script>

<a href="/">Close</a>

<h1>Settings</h1>

<form on:submit={saveSettings}>
    Delimiter: <input type="text" name="delimiter" value={delimiter} />
    <button type="submit">Save</button>
</form>
