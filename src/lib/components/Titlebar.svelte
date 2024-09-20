<script lang="ts">
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { XMark, MinusSmall } from "svelte-heros-v2";
</script>

<div data-tauri-drag-region class="absolute left-0 right-0 top-0 z-50 flex justify-end p-2">
	<button
		class="bg-transparent px-3 py-1 text-gray-800 transition-transform duration-150 ease-in-out hover:text-gray-950 focus:outline-none active:scale-95 dark:text-gray-200 dark:hover:text-gray-50"
		on:click="{() => getCurrentWindow().minimize()}">
		<MinusSmall class="h-6 w-6 text-gray-800 dark:text-gray-200" />
	</button>
	<button
		class="bg-transparent px-3 py-1 text-gray-800 transition-transform duration-150 ease-in-out hover:text-gray-950 focus:outline-none active:scale-95 dark:text-gray-200 dark:hover:text-gray-50"
		on:click="{() => getCurrentWindow().close()}">
		<XMark class="h-6 w-6 text-gray-800 dark:text-gray-200" />
	</button>
</div>
