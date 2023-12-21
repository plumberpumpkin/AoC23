<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    let input6a;
    let result6a = "Your answer will appear here";
    async function puzzle6advanced(){
        result6a = await invoke("puzzle6advanced", { input: input6a })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle6advanced}>
        <textarea id="puzzle6-input" class="input-text-area"  bind:value={input6a}/>
        <button type="submit">Solve 06.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result6a}</p>
</div>
