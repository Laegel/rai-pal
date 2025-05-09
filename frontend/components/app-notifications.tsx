import { useAppEvent } from "@hooks/use-app-event";
import { DefaultMantineColor } from "@mantine/core";
import { Notifications, notifications } from "@mantine/notifications";
import { IconCheck, IconInfoCircle, IconX } from "@tabler/icons-react";
import React from "react";

type NotificationType = "error" | "success" | "info";

function getNotificationColor(type: NotificationType): DefaultMantineColor {
	switch (type) {
		case "error":
			return "red";
		case "info":
			return "blue";
		case "success":
			return "green";
	}
}
function getNotificationIcon(type: NotificationType): React.ReactNode {
	switch (type) {
		case "error":
			return <IconX />;
		case "info":
			return <IconInfoCircle />;
		case "success":
			return <IconCheck />;
	}
}

export function showAppNotification(message: string, type: NotificationType) {
	notifications.show({
		message,
		color: getNotificationColor(type),
		icon: getNotificationIcon(type),
	});
}

export function AppNotifications() {
	useAppEvent("errorRaised", "app-notifications", (error) => {
		showAppNotification(error, "error");
	});

	useAppEvent("executedProviderCommand", "app-notifications", () => {
		showAppNotification(
			"Running command... This might take a few seconds.",
			"info",
		);
	});

	return <Notifications />;
}
