<script>
  import Greet from './lib/Greet.svelte'
  import Puzzle1 from "./lib/Puzzle1.svelte";
  import Puzzle1Advanced from "./lib/Puzzle1Advanced.svelte";
    import Puzzle2 from "./lib/Puzzle2.svelte";
  import Puzzle2Advanced from "./lib/Puzzle2Advanced.svelte";
</script>

<main class="container">
  <h1>Advent of Code 2023</h1>
  <h2>Puzzle 1 - 01.12.2023</h2>
  <div class="row">
    <Puzzle1/>
  </div>
  <div class="row">
    <Puzzle1Advanced/>
  </div>
  <h2>Puzzle 2 - 02.12.2023</h2>
  <Puzzle2/>
  <div class="row">
    <Puzzle2Advanced/>
  </div>


</main>