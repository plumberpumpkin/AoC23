<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    let input7;
    let result7 = "Your answer will appear here";
    async function puzzle7(){
        result7 = await invoke("puzzle7", { input: input7 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle7}>
        <textarea id="puzzle7-input" class="input-text-area"  bind:value={input7}/>
        <button type="submit">Solve 07.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result7}</p>
</div>
