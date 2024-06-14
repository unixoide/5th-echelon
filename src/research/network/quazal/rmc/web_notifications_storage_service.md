# WebNotificationsStorageService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [WebNotificationsStorageProtocol](#webnotificationsstorageprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## WebNotificationsStorageProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func RegisterUser() -> ()
```

</td></tr>
<tr><td>2</td><td>

```swift
func PollNotifications() -> (listNotifications: string, nbNotifications: int32)
```

</td></tr>
<tr><td>3</td><td>

```swift
func UnregisterUser() -> ()
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
