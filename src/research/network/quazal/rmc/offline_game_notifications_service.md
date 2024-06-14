# OfflineGameNotificationsService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 71 | [OfflineGameNotificationsProtocol](#offlinegamenotificationsprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## OfflineGameNotificationsProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func PollNotifications() -> (listNotifications: qlist<[[NotificationEvent]]((notificationevent))>, nbRemainingNotifs: uint32)
```

</td></tr>
<tr><td>2</td><td>

```swift
func PollSpecificOfflineNotifications(majortype: qlist<uint32>) -> (listTimedNotification: qlist<[[TimedNotification]]((timednotification))>, ret: uint32)
```

</td></tr>
<tr><td>3</td><td>

```swift
func PollAnyOfflineNotifications() -> (listTimedNotification: qlist<[[TimedNotification]]((timednotification))>, nbRemainingNotifs: uint32)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
