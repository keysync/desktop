import type { PageLoad } from "./$types";

export const load = (async ({ url }) => {
	const username = url.searchParams.get("username");
	const email = url.searchParams.get("email");
	const avatarUrl = url.searchParams.get("avatar_url");

	return {
		username,
		email,
		avatarUrl,
	};
}) satisfies PageLoad;
