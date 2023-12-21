<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    import {readText} from "@tauri-apps/api/clipboard";


    let input4;
    let result4 = "Your answer will appear here";

    async function puzzle4(){
        result4 = await invoke("puzzle4", { input: input4 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle4}>
        <textarea id="puzzle4-input" class="input-text-area"  bind:value={input4}/>
        <button type="submit">Solve 04.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result4}</p>
</div>
