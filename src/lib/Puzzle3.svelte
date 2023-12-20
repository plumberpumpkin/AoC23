<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    import {readText} from "@tauri-apps/api/clipboard";


    let input3;
    let result3 = "Your answer will appear here";

    async function puzzle3(){
        result3 = await invoke("puzzle3", { input: input3 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle3}>
        <textarea id="puzzle3-input" class="input-text-area"  bind:value={input3}/>
        <button type="submit">Solve 02.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result3}</p>
</div>
