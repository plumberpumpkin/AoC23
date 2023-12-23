<script>
  import Puzzle1 from "./lib/Puzzle1.svelte";
  import Puzzle1Advanced from "./lib/Puzzle1Advanced.svelte";
  import Puzzle2 from "./lib/Puzzle2.svelte";
  import Puzzle2Advanced from "./lib/Puzzle2Advanced.svelte";
  import Puzzle3 from "./lib/Puzzle3.svelte";
  import Puzzle4 from "./lib/Puzzle4.svelte";
  import Puzzle5 from "./lib/Puzzle5.svelte";
  import Puzzle6 from "./lib/Puzzle6.svelte";
  import Puzzle6Advanced from "./lib/Puzzle6Advanced.svelte";
  import Puzzle7 from "./lib/Puzzle7.svelte";
  import Puzzle8 from "./lib/Puzzle8.svelte";
  import Puzzle9 from "./lib/Puzzle9.svelte";
</script>

<main class="container">

  <a href="routes/2023">AoC 2023</a>
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
  <h2>Puzzle 3</h2>
  <div class="row">
    <Puzzle3/>
  </div>
  <h2>Puzzle 4</h2>
  <div class="row">
    <Puzzle4/>
  </div>
  <h2>Puzzle 5</h2>
  <div class="row">
    <Puzzle5/>
  </div>
  <h2>Puzzle 6</h2>
  <div class="row">
      <Puzzle6/>
  </div>
  <div class="row">
    <Puzzle6Advanced/>
  </div>
  <h2>Puzzle 7</h2>
  <div class="row">
      <Puzzle7/>
  </div>
  <h2>Puzzle 8</h2>
  <div class="row">
      <Puzzle8/>
  </div>
  <h2>Puzzle 9</h2>
    <div class="row">
        <Puzzle9/>
  </div>
</main>