<script lang="ts">
  // Placeholder for the keybinds module (docs/KEYBINDS.md): profile list with health-check
  // badges, search, controller view and the key tester. Data will come from the core's
  // `binds.*` queries.
  const planned = [
    "Profiles with health-check badges (KEYBINDS.md §7.6)",
    "Search and reverse lookup: what does this button do?",
    "Key tester (seeded from s7-input-listener)",
    "Controller and ship-centric views",
  ];
</script>

<h2>Keybinds</h2>
<p>Placeholder module. Planned:</p>
<ul>
  {#each planned as item (item)}
    <li>{item}</li>
  {/each}
</ul>
