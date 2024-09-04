<style lang="postcss">
	::-ms-reveal {
		display: none;
	}
</style>

<script lang="ts">
	import { onMount } from "svelte";
	import { register } from "@tauri-apps/plugin-deep-link";

	let email: string = "";
	let password: string = "";
	let errorMessage: string = "";

	onMount(async () => {
		await register("keysync").then(() => {
			console.log("Registered deep link");
		});
	});

	function handleLogin(): void {
		if (email === "" || password === "") {
			errorMessage = "Please enter both email and password.";
			return;
		}

		console.log("Logging in with email:", email, "and password:", password);
	}

	async function handleGitHubLogin(): Promise<void> {
		try {
			// invoke the GitHub OAuth flow
			console.log("Logging in with GitHub...");
		} catch (error) {
			console.error("Error logging in with GitHub:", error);
		}
	}

	async function handleDiscordLogin(): Promise<void> {
		try {
			// invoke the Discord OAuth flow
			console.log("Logging in with Discord...");
		} catch (error) {
			console.error("Error logging in with Discord:", error);
		}
	}

	async function handleGoogleLogin(): Promise<void> {
		try {
			// invoke the Google OAuth flow
			console.log("Logging in with Google...");
		} catch (error) {
			console.error("Error logging in with Google:", error);
		}
	}
</script>

<div class="flex h-screen items-center justify-center overflow-hidden bg-gray-100 dark:bg-gray-900">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-md dark:bg-gray-800">
		<h1 class="mb-6 text-center text-2xl font-semibold text-gray-900 dark:text-gray-100">KeySync Sign up</h1>

		<form on:submit|preventDefault="{handleLogin}">
			<div class="mb-4">
				<label for="email" class="block text-gray-700 dark:text-gray-300">Email</label>
				<input
					type="email"
					id="email"
					bind:value="{email}"
					class="mt-1 block h-8 w-full border-0 border-b-2 border-gray-300 bg-white px-2 text-gray-900 transition-colors duration-300 focus:border-gray-700 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 dark:focus:border-gray-400"
					autocomplete="off"
					required />
			</div>

			<div class="mb-6">
				<label for="password" class="block text-gray-700 dark:text-gray-300">Password</label>
				<input
					type="password"
					id="password"
					bind:value="{password}"
					class="mt-1 block h-8 w-full border-0 border-b-2 border-gray-300 bg-white px-2 text-gray-900 transition-colors duration-300 focus:border-gray-700 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-gray-100 dark:focus:border-gray-400"
					autocomplete="off"
					required />
			</div>

			{#if errorMessage}
				<p class="mb-4 text-sm text-red-500">{errorMessage}</p>
			{/if}

			<button
				type="submit"
				class="w-full rounded bg-blue-500 px-4 py-2 font-bold text-white transition-transform duration-150 ease-in-out hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 active:scale-95">
				Log In
			</button>
		</form>

		<div class="mt-6 flex justify-between">
			<button
				type="button"
				on:click="{handleGitHubLogin}"
				class="ml-2 flex w-1/2 items-center justify-center rounded bg-[#F5F5F5] px-4 py-2 font-bold text-black transition-transform duration-150 ease-in-out hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-200 focus:ring-opacity-50 active:scale-95 dark:bg-black dark:text-white dark:hover:bg-[#131313] dark:focus:ring-black">
				<img src="/github-logo-dark.svg" alt="GitHub Light Logo" class="h-6 w-6 dark:hidden" />
				<img src="/github-logo-light.svg" alt="GitHub Dark Logo" class="hidden h-6 w-6 dark:block" />
			</button>

			<button
				type="button"
				on:click="{handleDiscordLogin}"
				class="ml-2 flex w-1/2 items-center justify-center rounded bg-[#5865F2] px-4 py-2 font-bold text-white transition-transform duration-150 ease-in-out hover:bg-[#4e5bbd] focus:outline-none focus:ring-2 focus:ring-[#5865F2] focus:ring-opacity-50 active:scale-95">
				<img src="/discord-logo.svg" alt="Discord Logo" class="h-6 w-6" />
			</button>

			<button
				type="button"
				on:click="{handleGoogleLogin}"
				class="ml-2 flex w-1/2 items-center justify-center rounded bg-[#F5F5F5] px-4 py-2 font-bold text-black transition-transform duration-150 ease-in-out hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-200 focus:ring-opacity-50 active:scale-95 dark:bg-black dark:text-white dark:hover:bg-[#131313] dark:focus:ring-black">
				<img src="/google-logo.svg" alt="Google Logo" class="h-6 w-6" />
			</button>
		</div>

		<p class="mt-4 text-center text-gray-600 dark:text-gray-400">
			Already have an account? <a href="/" class="text-blue-500 hover:underline dark:text-blue-300">Log in</a>
		</p>
	</div>
</div>
