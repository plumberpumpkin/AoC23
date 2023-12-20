<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    import {readText} from "@tauri-apps/api/clipboard";


    let input2a;
    let result2a = "Your answer will appear here";

    async function puzzle2advanced(){
        result2a = await invoke("puzzle2advanced", { input: input2a })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle2advanced}>
        <textarea id="puzzle2a-input" class="input-text-area"  bind:value={input2a}/>
        <button type="submit">Solve 02.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result2a}</p>
</div>
