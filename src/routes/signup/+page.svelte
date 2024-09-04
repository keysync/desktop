<script lang="ts">
	let email: string = "";
	let password: string = "";
	let errorMessage: string = "";

	function handleLogin(): void {
		if (email === "" || password === "") {
			errorMessage = "Please enter both email and password.";
			return;
		}

		console.log("Logging in with email:", email, "and password:", password);
	}
</script>

<div class="flex min-h-screen items-center justify-center bg-gray-100">
	<div class="w-full max-w-md rounded-lg bg-white p-8 shadow-md">
		<h1 class="mb-6 text-center text-2xl font-semibold">KeySync Sign up</h1>

		<form on:submit|preventDefault="{handleLogin}">
			<div class="mb-4">
				<label for="email" class="block text-gray-700">Email</label>
				<input
					type="email"
					id="email"
					bind:value="{email}"
					class="mt-1 block h-8 w-full border-0 border-b-2 border-gray-300 transition-colors duration-300 focus:border-gray-700 focus:outline-none"
					autocomplete="off"
					required />
			</div>

			<div class="mb-6">
				<label for="password" class="block text-gray-700">Password</label>
				<input
					type="password"
					id="password"
					bind:value="{password}"
					class="mt-1 block h-8 w-full border-0 border-b-2 border-gray-300 transition-colors duration-300 focus:border-gray-700 focus:outline-none"
					autocomplete="off"
					required />
			</div>

			{#if errorMessage}
				<p class="mb-4 text-sm text-red-500">{errorMessage}</p>
			{/if}

			<button
				type="submit"
				class="w-full rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50">
				Log In
			</button>
		</form>

		<p class="mt-4 text-center text-gray-600">
			Already have an account? <a href="/" class="text-blue-500 hover:underline">Log in</a>
		</p>
	</div>
</div>
