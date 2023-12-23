<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    let input9;
    let result9 = "Your answer will appear here";
    async function puzzle9(){
        result9 = await invoke("puzzle9", { input: input9 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle9}>
        <textarea id="puzzle9-input" class="input-text-area"  bind:value={input9}/>
        <button type="submit">Solve 09.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result9}</p>
</div>
