<script lang="ts">
	import type { PageData } from "./$types";

	export let data: PageData;

	function handleConfirm() {
		console.log("Confirming GitHub information...");
	}
</script>

<div class="flex h-screen items-center justify-center overflow-hidden bg-gray-100 dark:bg-gray-900">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-md dark:bg-gray-800">
		<h1 class="mb-6 text-center text-2xl font-semibold text-gray-900 dark:text-gray-100">Confirm GitHub Information</h1>

		<div class="flex justify-center mb-4">
			<img src="{data.avatarUrl}" alt="GitHub Avatar" class="w-24 h-24 rounded-full" />
		</div>

		<p class="mb-4 text-gray-700 dark:text-gray-300">Username: {data.username}</p>
		<p class="mb-6 text-gray-700 dark:text-gray-300">Email: {data.email}</p>

		<button
			on:click="{handleConfirm}"
			class="w-full rounded bg-blue-500 px-4 py-2 font-bold text-white transition-transform duration-150 ease-in-out hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 active:scale-95">
			Confirm
		</button>
		
		<p class="mt-6 text-center text-gray-600 dark:text-gray-400">
			Not you? <a href="/" class="text-blue-500 hover:underline dark:text-blue-300">Try again</a>
		</p>
	</div>
</div>
