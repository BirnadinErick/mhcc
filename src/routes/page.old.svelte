<script lang="ts">
	import Notification from '$lib/components/Notification.svelte';
	import { NotificationCategory, type NotificationType } from '$lib/types/NotificationType';

	let ks = [1, 2, 3, 4, 5, 6, 7, 8];

	let notifications: Array<NotificationType> = [
		{
			category: NotificationCategory.danger,
			data: 'Hello world, I am danger',
			link: '/settings'
		},
		{
			category: NotificationCategory.misc,
			data: 'Hey there, you can ignore me',
			link: '/'
		},
		{
			category: NotificationCategory.success,
			data: 'Successsfully transferred backups to cloud!',
			link: ''
		},
		{
			category: NotificationCategory.warning,
			data: 'I am about to explode 💥',
			link: '/stocks'
		},
		{
			category: NotificationCategory.danger,
			data: 'Hello world, I am danger',
			link: '/settings'
		},
		{
			category: NotificationCategory.misc,
			data: 'Hey there, you can ignore me',
			link: '/'
		},
		{
			category: NotificationCategory.success,
			data: 'Successsfully transferred backups to cloud!',
			link: ''
		},
		{
			category: NotificationCategory.warning,
			data: 'I am about to explode 💥',
			link: '/stocks'
		},
		{
			category: NotificationCategory.danger,
			data: 'Hello world, I am danger',
			link: '/settings'
		},
		{
			category: NotificationCategory.misc,
			data: 'Hey there, you can ignore me',
			link: '/'
		},
		{
			category: NotificationCategory.success,
			data: 'Successsfully transferred backups to cloud!',
			link: ''
		},
		{
			category: NotificationCategory.warning,
			data: 'I am about to explode 💥',
			link: '/stocks'
		},
		{
			category: NotificationCategory.danger,
			data: 'Hello world, I am danger',
			link: '/settings'
		},
		{
			category: NotificationCategory.misc,
			data: 'Hey there, you can ignore me',
			link: '/'
		},
		{
			category: NotificationCategory.success,
			data: 'Successsfully transferred backups to cloud!',
			link: ''
		},
		{
			category: NotificationCategory.warning,
			data: 'I am about to explode 💥',
			link: '/stocks'
		},
		{
			category: NotificationCategory.danger,
			data: 'Hello world, I am danger',
			link: '/settings'
		},
		{
			category: NotificationCategory.misc,
			data: 'Hey there, you can ignore me',
			link: '/'
		},
		{
			category: NotificationCategory.success,
			data: 'Successsfully transferred backups to cloud!',
			link: ''
		},
		{
			category: NotificationCategory.warning,
			data: 'I am about to explode 💥',
			link: '/stocks'
		}
	];
</script>

<svelte:head>
	<title>Overview | MHCC</title>
</svelte:head>

<div class="flex justify-around overflow-hidden">
	<div class="flex-auto">
		<div class="text-black pt-4 cursor-default select-none h-24">
			<h1 class="font-serif text-5xl">Good Morning</h1>
			<h3 class="text-medium mt-2 ml-1 capitalize text-gray-600">
				Welcome to Mathagal Health Care Center 🏥
			</h3>
		</div>

		<div class="my-4">
			<h2 class="py-4 text-2xl font-serif">Overview.</h2>
		</div>

		<div class="mr-8 overflow-hidden select-none cursor-default">
			<div class="grid grid-cols-3 grid-rows-3 gap-4">
				{#each ks as k}
					<div
						class="bg-white text-indigo p-4 shadow-md hover:shadow-xl transition-shadow ease-in-out text-center"
					>
						<p class="mt-1 font-serif text-xl tracking-wide">Expiring Stocks</p>
						<p class="mt-1 font-mono text-4xl">{Math.ceil(k * Math.random() * 30)}</p>
						<p class="mt-1 font-sans text-sm uppercase">items</p>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<div class="bg-white w-72 px-4 py-4 select-none cursor-default overflow-hidden h-full">
		<div class="flex justify-between">
			<h2 class="text-2xl font-serif">Notifications</h2>
			<span class="cursor-pointer"><a href="/notifications">👉</a></span>
		</div>
		<div class="overflow-y-auto">
			<div class="cursor-pointer flex-col justify-evenly mt-4 space-y-3">
				{#each notifications as notification}
					<Notification {notification} />
				{/each}
			</div>
		</div>
	</div>
</div>
