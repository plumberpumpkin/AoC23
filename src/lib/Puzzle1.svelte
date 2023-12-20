<script>
    import {invoke}  from "@tauri-apps/api/tauri";

    let input1 = "";
    let result1 = "";

    async function puzzle1(){
        result1 = await invoke("puzzle1", { input: input1 })
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={puzzle1}>
        <textarea id="puzzle1-input" placeholder="Enter your puzzle input for 01.12.2023" bind:value={input1}/>
        <button type="submit">Solve 01.12.</button>
    </form>
    <p>{result1}</p>
</div>