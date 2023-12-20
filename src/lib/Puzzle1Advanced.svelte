<script>
    import {invoke}  from "@tauri-apps/api/tauri";

    let input1a = "";
    let result1a = "Your answer will appear here";

    async function puzzle1(){
        result1a = await invoke("puzzle1advanced", { input: input1a })
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={puzzle1}>
        <textarea id="puzzle1-input" placeholder="Enter your puzzle input for 01.12.2023 - Advanced mode" bind:value={input1a}/>
        <button type="submit">Solve 01.12. Advanced</button>
    </form>
    <p>{result1a}</p>
</div>