<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    import {readText} from "@tauri-apps/api/clipboard";


    let input5;
    let result5 = "Your answer will appear here";

    async function puzzle5(){
        result5 = await invoke("puzzle5", { input: input5 })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle5}>
        <textarea id="puzzle5-input" class="input-text-area"  bind:value={input5}/>
        <button type="submit">Solve 05.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result5}</p>
</div>
