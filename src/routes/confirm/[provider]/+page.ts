import type { PageLoad } from "./$types";

export const load = (async ({ params, url }) => {
	const provider = params.provider;
	const username = url.searchParams.get("username");
	const email = url.searchParams.get("email");
	const avatarUrl = url.searchParams.get("avatar_url");

	return {
		provider,
		username,
		email,
		avatarUrl,
	};
}) satisfies PageLoad;
