<script lang="ts">
	import { Cog6Tooth, Home, User } from "svelte-heros-v2";

	let menuItems = [
		{ name: "Dashboard", href: "/", icon: Home },
		{ name: "Settings", href: "/settings", icon: Cog6Tooth },
		{ name: "Profile", href: "/profile", icon: User },
	];
</script>

<aside class="group relative h-full bg-white dark:bg-gray-800 transition-all duration-300">
	<div class="absolute inset-y-0 flex h-full flex-col justify-between p-2 w-16 group-hover:w-64 transition-all duration-300 bg-white dark:bg-gray-800 overflow-hidden">
		<!-- Logo -->
		<div class="mb-8 flex items-center p-4">
			<h1 class="text-xl font-semibold text-gray-900 dark:text-gray-100 group-hover:opacity-100 opacity-0 transition-opacity duration-300">
				KeySync
			</h1>
		</div>

		<!-- Menu Items -->
		<nav>
			<ul>
				{#each menuItems as item}
					<li class="mb-4">
						<a
							href="{item.href}"
							class="flex items-center p-2 text-gray-700 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:text-white rounded-lg">
							<svelte:component this={item.icon} class="mr-0 group-hover:mr-3 h-6 w-6 transition-all duration-300" />
							<span class="group-hover:inline hidden text-sm">{item.name}</span>
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	</div>
</aside>
