# ProtocolFoundation

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [NotificationProtocol](#notificationprotocol) |
| ? | [RemoteLogDeviceProtocol](#remotelogdeviceprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## NotificationProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func ProcessNotificationEvent(oEvent: [[NotificationEvent]]((notificationevent))) -> ()
```

</td></tr>
</tbody></table>
## RemoteLogDeviceProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func Log(strLine: string) -> ()
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
