<script lang="ts">
import {NotificationCategory, type NotificationType} from "$lib/types/NotificationType";

    export let notification:NotificationType;

    let bg:string;

    switch (notification.category) {
        case NotificationCategory.danger:
            bg = "bg-red"
            break;
        case NotificationCategory.warning:
            bg = "bg-yellow"
            break
        case NotificationCategory.success:
            bg = "bg-green"
            break
        case NotificationCategory.misc:
            bg = "bg-indigo"
            break
        default:
            throw new Error("Bad arg for notification bubble!");
    }

</script>

<div class="{ bg } p-2 {notification.category === NotificationCategory.misc ? "text-white" : "text-indigo"} font-sans shadow-sm hover:shadow-md text-sm">
    <a href={notification.link === "" ? "/" : notification.link}>
        <p>{notification.data}</p>
    </a>
</div>