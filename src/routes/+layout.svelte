<script lang="ts">
	import Titlebar from "$lib/Titlebar.svelte";
	import "../app.css";

	document.addEventListener("contextmenu", (event) => {
		event.preventDefault();
	});
</script>

<Titlebar />

<slot />
