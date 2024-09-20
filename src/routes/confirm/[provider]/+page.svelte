<script lang="ts">
	import type { PageData } from "./$types";
	export let data: PageData;

	function handleConfirm(): void {
		console.log("Confirming GitHub information. Redirecting to app...");
		// Proceed to app
	}

	function goBack(): void {
		history.back();
	}
</script>

<div class="flex h-screen items-center justify-center overflow-hidden bg-gray-100 dark:bg-gray-900">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-md dark:bg-gray-800">
		<h1 class="mb-6 text-center text-2xl font-semibold text-gray-900 dark:text-gray-100">
			{`Confirm ${data.provider} Login`}
		</h1>

		{#if data.avatarUrl}
			<div class="mb-4 flex justify-center">
				<img src="{data.avatarUrl}" alt="Avatar" class="h-24 w-24 rounded-full" />
			</div>
		{/if}

		{#if data.username}
			<p class="mb-4 text-gray-700 dark:text-gray-300">Username: {data.username}</p>
		{/if}

		{#if data.email}
			<p class="mb-6 text-gray-700 dark:text-gray-300">Email: {data.email}</p>
		{/if}

		<button
			on:click="{handleConfirm}"
			class="w-full rounded bg-blue-500 px-4 py-2 font-bold text-white transition-transform duration-150 ease-in-out hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 active:scale-95">
			Confirm
		</button>

		<p class="mt-6 text-center text-gray-600 dark:text-gray-400">
			Not you?
			<button
				on:click="{goBack}"
				on:keydown="{(e) => {
					if (['Enter', ''].includes(e.key)) {
						e.preventDefault();
						goBack();
					}
				}}"
				tabindex="0"
				class="text-blue-500 hover:underline dark:text-blue-300">
				Try again
			</button>
		</p>
	</div>
</div>
