<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    let input6;
    let result6 = "Your answer will appear here";
    async function puzzle6(){
        result6 = await invoke("puzzle6", { input: input6 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle6}>
        <textarea id="puzzle6-input" class="input-text-area"  bind:value={input6}/>
        <button type="submit">Solve 06.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result6}</p>
</div>
