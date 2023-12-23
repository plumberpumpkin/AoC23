<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    let input8;
    let result8 = "Your answer will appear here";
    async function puzzle8(){
        result8 = await invoke("puzzle8", { input: input8 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle8}>
        <textarea id="puzzle8-input" class="input-text-area"  bind:value={input8}/>
        <button type="submit">Solve 08.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result8}</p>
</div>
