<script>
    import {invoke}  from "@tauri-apps/api/tauri";
    import {readText} from "@tauri-apps/api/clipboard";


    let input2;
    let blueCubes = 0;
    let redCubes = 0;
    let greenCubes = 0;
    let result2 = "Your answer will appear here";

    async function puzzle2(){
        result2 = await invoke("puzzle2", { input: input2, blueCubes: blueCubes, redCubes: redCubes, greenCubes: greenCubes })
    }
</script>

<div class="row">
    <form on:submit|preventDefault={puzzle2}>
        <textarea id="puzzle2-input" class="input-text-area"  bind:value={input2}/>
        <input placeholder="Red Cubes" type="number" id="red-cubes" bind:value={redCubes}/>
        <input placeholder="Green Cubes" type="number" id="green-cubes" bind:value={greenCubes}/>
        <input placeholder="Blue Cubes" type="number" id="blue-cubes" bind:value={blueCubes}/>
        <button type="submit">Solve 02.12.</button>
    </form>
</div>
<div class="row">
    <p class="row">{result2}</p>
</div>
