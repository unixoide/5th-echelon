# TrackingService

## Protocols

<!-- INSERT protocol_idx START -->
| Protocol ID | Name |
|-------------|------|
| ? | [TrackingProtocol3](#trackingprotocol3) |
<!-- INSERT protocol_idx END -->

<!-- INSERT protocols START -->
## TrackingProtocol3
<table><thead><tr><th>Method ID</th><th>Signature</th></tr></thead>
<tbody>
<tr><td>1</td><td>

```swift
func SendTag(trackingID: uint32, tag: string, attributes: string, deltaTime: uint32) -> ()
```

</td></tr>
<tr><td>2</td><td>

```swift
func SendTagAndUpdateUserInfo(trackingID: uint32, tag: string, attributes: string, deltaTime: uint32, userID: string) -> ()
```

</td></tr>
<tr><td>3</td><td>

```swift
func SendUserInfo(deltaTime: uint32) -> (trackingID: uint32)
```

</td></tr>
<tr><td>4</td><td>

```swift
func GetConfiguration() -> (tags: std_list<string>)
```

</td></tr>
<tr><td>5</td><td>

```swift
func SendTags(tagData: std_list<[[TrackingTag]]((trackingtag))>) -> ()
```

</td></tr>
</tbody></table>
<!-- INSERT protocols END -->
