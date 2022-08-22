enum NotificationCategory {
	danger,
	warning,
	success,
	misc
}

type NotificationType = {
	category: NotificationCategory;
	data: string;
	link: string;
};

export type { NotificationType };
export { NotificationCategory };
