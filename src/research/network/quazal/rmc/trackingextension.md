# trackingextension

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| 1001 | [TrackingExtensionProtocol](#trackingextensionprotocol) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## TrackingExtensionProtocol
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func GetTrackingUserGroup(pid: uint32) -> (usergroup: uint32)
```

</td></tr>
<tr><td>2</td><td>

```swift
func GetTrackingUserGroupTags(usergroup: uint32) -> (tags: qvector<string>)
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
